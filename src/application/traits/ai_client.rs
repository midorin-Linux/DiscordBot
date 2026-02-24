use anyhow::Result;
use async_trait::async_trait;
use rig::completion::Message;

#[async_trait]
pub trait AIClient: Send + Sync {
    async fn generate(&self, prompt: Message, chat_history: Vec<Message>) -> Result<String>;
    async fn embed(&self, text: String) -> Result<Vec<f32>>;
}
