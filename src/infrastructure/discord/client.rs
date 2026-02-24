use crate::infrastructure::ai::rig_client::RigClient;
use crate::infrastructure::store::{in_memory_store::InMemoryStore, vector_store::VectorStore};
use crate::presentation::handler::Handler;
use std::sync::Arc;

use anyhow::{Context, Result};
use serenity::prelude::*;

pub struct DiscordClient {
    discord_client: Client,
}

impl DiscordClient {
    pub async fn new(
        discord_token: String,
        guild_id: u64,
        rig_client: RigClient,
        in_memory_store: Arc<InMemoryStore>,
        vector_store: Arc<VectorStore>,
    ) -> Result<Self> {
        let intents = GatewayIntents::all();

        let rig_client = Arc::new(rig_client);

        let command_framework = crate::application::command::command_registry::command_framework(
            guild_id,
            rig_client.clone(),
            in_memory_store,
            vector_store,
        )
        .await;

        let client = Client::builder(discord_token, intents)
            .event_handler(Handler { rig_client })
            .framework(command_framework)
            .await
            .context("Failed to create Discord client")?;

        Ok(Self {
            discord_client: client,
        })
    }

    pub async fn run(mut self) -> Result<()> {
        self.discord_client
            .start()
            .await
            .context("Failed to start Discord client")?;

        Ok(())
    }
}
