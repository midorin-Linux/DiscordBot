use crate::agent::{agent::Agent, prompts::load_system_prompt};
use crate::bot::{commands::commands, handler::Handler};
use crate::services::openai::OpenAiService;
use crate::utils::config::Config;
use std::sync::Arc;

use anyhow::Result;
use serenity::prelude::*;

pub struct App {
    config: Config,
}

impl App {
    pub async fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!();

        let database = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename(&self.config.database_url)
                    .create_if_missing(true),
            )
            .await
            .map_err(|e| anyhow::anyhow!("Error connecting to database: {:?}", e))?;
        sqlx::migrate!("./migrations").run(&database).await
            .map_err(|e| {
                anyhow::anyhow!(
                "Error running migrations: {:?}. Please check that the database is up to date.",
                e
            )
        })?;

        let intents = GatewayIntents::all();

        let openai_client = OpenAiService::new(
            &self.config.provider.api_key,
            &self.config.provider.base_url,
            &self.config.model.name,
            self.config.sampling.clone(),
        );

        let system_prompt = load_system_prompt();
        let agent = Arc::new(Agent::new(openai_client, system_prompt));

        let framework = commands::command_framework(self.config.target_guild_id, agent.clone()).await;

        let mut client = Client::builder(&self.config.discord_token, intents)
            .event_handler(Handler {
                allowed_user_id: self.config.allowed_user_id,
                agent,
                pool: database,
            })
            .framework(framework)
            .await
            .map_err(|e| anyhow::anyhow!("Error creating client: {:?}", e))?;

        client
            .start()
            .await
            .map_err(|e| anyhow::anyhow!("Error starting client: {:?}", e))?;

        Ok(())
    }
}
