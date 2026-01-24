use crate::agent::chat::ChatService;
use crate::agent::memory::ConversationMemory;
use crate::models::message::{Message, MessageRole};
use crate::services::openai::OpenAiService;

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// AIエージェントのメイン構造体
pub struct Agent {
    chat_service: ChatService,
    conversations: Arc<RwLock<HashMap<String, ConversationMemory>>>,
    max_history: usize,
}

impl Agent {
    /// 新しいエージェントインスタンスを作成
    pub fn new(openai: OpenAiService, system_prompt: String) -> Self {
        let chat_service = ChatService::new(openai, system_prompt);

        Self {
            chat_service,
            conversations: Arc::new(RwLock::new(HashMap::new())),
            max_history: 20, // デフォルトで20件の履歴を保持
        }
    }

    /// 最大履歴数を設定
    pub fn with_max_history(mut self, max: usize) -> Self {
        self.max_history = max;
        self
    }

    /// ユーザーからのメッセージを処理（会話履歴なし）
    pub async fn process_message_simple(&self, content: &str) -> Result<String> {
        tracing::info!("Processing simple message");
        self.chat_service.single_chat(content).await
    }

    /// ユーザーからのメッセージを処理（会話履歴あり）
    pub async fn process_message(&self, user_id: &str, content: &str) -> Result<String> {
        tracing::info!("Processing message from user: {}", user_id);

        // 会話履歴を取得または作成
        let memory = {
            let mut conversations = self.conversations.write().await;
            conversations
                .entry(user_id.to_string())
                .or_insert_with(|| ConversationMemory::new(self.max_history))
                .clone()
        };

        // ユーザーメッセージを履歴に追加
        self.add_message_to_history(
            user_id,
            Message::new(MessageRole::User, content.to_string()),
        )
            .await;

        // AIの応答を取得
        let response = self.chat_service.chat_with_history(content, &memory).await?;

        // AIの応答を履歴に追加
        self.add_message_to_history(
            user_id,
            Message::new(MessageRole::Assistant, response.clone()),
        )
            .await;

        Ok(response)
    }

    /// 会話履歴にメッセージを追加
    async fn add_message_to_history(&self, user_id: &str, message: Message) {
        let mut conversations = self.conversations.write().await;
        if let Some(memory) = conversations.get_mut(user_id) {
            memory.add_message(message);
        }
    }

    /// 特定ユーザーの会話履歴をクリア
    pub async fn clear_history(&self, user_id: &str) -> Result<()> {
        let mut conversations = self.conversations.write().await;
        if let Some(memory) = conversations.get_mut(user_id) {
            memory.clear();
            tracing::info!("Cleared conversation history for user: {}", user_id);
        }
        Ok(())
    }

    /// 全ユーザーの会話履歴をクリア
    pub async fn clear_all_histories(&self) -> Result<()> {
        let mut conversations = self.conversations.write().await;
        conversations.clear();
        tracing::info!("Cleared all conversation histories");
        Ok(())
    }

    /// 特定ユーザーの会話履歴を取得
    pub async fn get_history(&self, user_id: &str) -> Option<Vec<Message>> {
        let conversations = self.conversations.read().await;
        conversations
            .get(user_id)
            .map(|memory| memory.get_messages().to_vec())
    }

    /// アクティブな会話の数を取得
    pub async fn active_conversations_count(&self) -> usize {
        let conversations = self.conversations.read().await;
        conversations.len()
    }

    /// システムプロンプトを更新
    pub fn update_system_prompt(&mut self, new_prompt: String) {
        self.chat_service.update_system_prompt(new_prompt);
    }
}
