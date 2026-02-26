use anyhow::Result;
use async_trait::async_trait;

use crate::models::memory::ChatMessage;

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn generate(&self, prompt: ChatMessage, chat_history: Vec<ChatMessage>)
    -> Result<String>;
    async fn embed(&self, text: String) -> Result<Vec<f32>>;
}
