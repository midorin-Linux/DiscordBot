use std::sync::Arc;

use anyhow::{Context, Result};
use serenity::prelude::*;

use crate::{
    application::traits::{
        ai_client::AIClient, long_term_store::LongTermStore, short_term_store::ShortTermStore,
    },
    presentation::handler::Handler,
};

pub struct DiscordClient {
    discord_client: Client,
}

impl DiscordClient {
    pub async fn new(
        discord_token: String,
        guild_id: u64,
        ai_client: Arc<dyn AIClient>,
        short_term_store: Arc<dyn ShortTermStore>,
        long_term_store: Arc<dyn LongTermStore>,
    ) -> Result<Self> {
        let intents = GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let command_framework = crate::presentation::command::command_registry::command_framework(
            guild_id,
            ai_client.clone(),
            short_term_store.clone(),
            long_term_store.clone(),
        )
        .await;

        let client = Client::builder(discord_token, intents)
            .event_handler(Handler {
                ai_client,
                short_term_store,
                long_term_store,
            })
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
