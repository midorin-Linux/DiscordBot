use crate::application::traits::ai_client::AIClient;
use crate::infrastructure::ai::tools::*;
use crate::shared::config::{Embedding, NLP};

use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use rig::completion::{Chat, Message};
use rig::completion::request::PromptError;
use rig::embeddings::EmbeddingModel;
use rig::prelude::*;
use rig::providers;

pub struct RigClient {
    nlp_client: rig::agent::Agent<providers::openai::responses_api::ResponsesCompletionModel>,
    embed_client: rig::providers::openai::EmbeddingModel,
}

impl RigClient {
    pub async fn new(
        nlp_api_key: String,
        embed_api_key: String,
        nlp: NLP,
        embedding: Embedding,
    ) -> Result<Self> {
        let system_instruction =
            std::fs::read_to_string("INSTRUCTION.md").context("Failed to read INSTRUCTION.md")?;

        let openai_comp_nlp_client = providers::openai::Client::builder()
            .api_key(nlp_api_key)
            .base_url(nlp.api_url)
            .build()
            .context("Failed to build openai nlp client")?;

        let nlp_client = openai_comp_nlp_client
            .agent(nlp.model_name)
            .preamble(system_instruction.as_str())
            .tool(test::Test)
            .default_max_turns(10)
            .build();

        let openai_comp_embed_client = providers::openai::Client::builder()
            .api_key(embed_api_key)
            .base_url(embedding.api_url)
            .build()
            .context("Failed to build openai embed client")?;

        let embed_client = openai_comp_embed_client.embedding_model(embedding.model_name);

        Ok(Self {
            nlp_client,
            embed_client,
        })
    }
}

#[async_trait]
impl AIClient for RigClient {
    async fn new(
        nlp_api_key: String,
        embed_api_key: String,
        nlp: NLP,
        embedding: Embedding,
    ) -> Result<Self> {
        RigClient::new(nlp_api_key, embed_api_key, nlp, embedding).await
    }

    async fn generate(&self, prompt: Message, chat_history: Vec<Message>) -> Result<String> {
        self.nlp_client
            .chat(prompt, chat_history)
            .await
            .map_err(|e: PromptError| anyhow!(e.to_string()))
    }

    async fn embed(&self, text: String) -> Result<Vec<f32>> {
        let embeddings = self
            .embed_client
            .embed_texts(vec![text])
            .await
            .map_err(|e| anyhow!(e.to_string()))?;

        embeddings
            .into_iter()
            .next()
            .map(|e| e.vec.into_iter().map(|v| v as f32).collect())
            .ok_or_else(|| anyhow!("No embedding returned"))
    }
}
