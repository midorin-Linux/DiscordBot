use crate::presentation::handler::Handler;

use anyhow::{Context, Result};
use serenity::prelude::*;

pub struct DiscordClient {
    discord_client: Client,
}

impl DiscordClient {
    pub async fn new(discord_token: String, guild_id: u64) -> Result<Self> {
        let intents = GatewayIntents::all(); //ToDo: 権限を絞る

        let command_framework =
            crate::application::command::command_registry::command_framework(guild_id).await;

        let client = Client::builder(discord_token, intents)
            .event_handler(Handler {})
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
