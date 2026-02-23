use crate::shared::config::{Embedding, NLP};

use anyhow::Result;
use async_trait::async_trait;
use rig::completion::Message;

#[async_trait]
pub trait AIClient: Sized {
    async fn new(
        nlp_api_key: String,
        embed_api_key: String,
        nlp: NLP,
        embedding: Embedding,
    ) -> Result<Self>;
    async fn generate(&self, prompt: Message, chat_history: Vec<Message>) -> Result<String>;
    async fn embed(&self, text: String) -> Result<Vec<f32>>;
}
