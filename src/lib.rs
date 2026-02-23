pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod models;
pub mod presentation;
pub mod shared;

use infrastructure::{
    ai::rig_client::RigClient,
    discord::client::DiscordClient,
    store::{in_memory_store::InMemoryStore, vector_store::VectorStore},
};
use shared::config::Config;

use anyhow::{Context, Result};
use std::sync::Arc;

pub struct Application {
    discord_client: DiscordClient,
}

impl Application {
    pub async fn new(config: Config) -> Result<Self> {
        let rig_client = RigClient::new(
            config.nlp_token.clone(),
            config.embed_token.clone(),
            config.nlp.clone(),
            config.embedding.clone(),
        )
            .await?;

        let in_memory_store = Arc::new(InMemoryStore::new(config.nlp.max_short_term_messages));
        let vector_store = Arc::new(
            VectorStore::new(&config.qdrant_url)
                .await
                .context("Failed to connect to Qdrant")?,
        );

        let discord_client = DiscordClient::new(
            config.discord_token.clone(),
            config.guild_id,
            rig_client,
            in_memory_store,
            vector_store,
        )
            .await?;

        Ok(Self { discord_client })
    }

    pub async fn run(self) -> Result<()> {
        self.discord_client
            .run()
            .await
            .context("Failed to run Discord client")?;

        Ok(())
    }
}