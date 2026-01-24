use crate::agent::agent::Agent;
use crate::services::openai::OpenAiService;
use crate::utils::config::Config;

use anyhow::Result;
use owo_colors::OwoColorize;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::path::Path;
use std::sync::Arc;

struct Handler {
    allowed_user_id: Option<u64>,
    agent: Arc<Agent>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.author.bot {
            return;
        }

        if let Some(allowed_id) = self.allowed_user_id {
            if new_message.author.id.get() != allowed_id {
                return;
            }
        }

        if let Err(e) = new_message.channel_id.broadcast_typing(&ctx.http).await {
            tracing::error!("Error sending typing: {:?}", e);
        }

        let user_id = new_message.author.id.to_string();
        let mut response = self
            .agent
            .process_message(&user_id, &new_message.content)
            .await;

        if let Err(e) = &response {
            tracing::error!("Error calling OpenAI API: {:?}", e);
            response = self
                .agent
                .process_message_simple(&new_message.content)
                .await;
        }

        match response {
            Ok(content) => {
                if let Err(e) = new_message.channel_id.say(&ctx.http, content).await {
                    tracing::error!("Error sending message: {:?}", e);
                }
            }
            Err(e) => {
                tracing::error!("Error calling OpenAI API again: {:?}", e);
                let error_message = format!(
                    "Sorry, something went wrong. Please try again later.\nDetails: {}",
                    e
                );
                if let Err(e) = new_message.channel_id.say(&ctx.http, error_message).await {
                    tracing::error!("Error sending error message: {:?}", e);
                }
            }
        }
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        tracing::info!("{} is connected to discord!", data_about_bot.user.name);
        println!("{} Ready!\n", "Ready!".green());
    }
}

pub struct App {
    config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!();

        let intents = GatewayIntents::all();

        let openai_client = OpenAiService::new(
            &self.config.openai_api_key,
            &self.config.openai_base_url,
            &self.config.openai_model,
        );

        let system_prompt = load_system_prompt();
        let agent = Arc::new(Agent::new(openai_client, system_prompt));

        let mut client = Client::builder(&self.config.discord_token, intents)
            .event_handler(Handler {
                allowed_user_id: self.config.allowed_user_id,
                agent,
            })
            .await
            .map_err(|e| anyhow::anyhow!("Error creating client: {:?}", e))?;

        client
            .start()
            .await
            .map_err(|e| anyhow::anyhow!("Error starting client: {:?}", e))?;

        Ok(())
    }
}

fn load_system_prompt() -> String {
    const DEFAULT_PROMPT: &str = "You are a helpful assistant.";
    let path = Path::new("prompts/system_prompt.txt");

    match std::fs::read_to_string(path) {
        Ok(content) => {
            let trimmed = content.trim();
            if trimmed.is_empty() {
                DEFAULT_PROMPT.to_string()
            } else {
                trimmed.to_string()
            }
        }
        Err(err) => {
            tracing::warn!(
                "Failed to read system prompt from {:?}: {:?}",
                path,
                err
            );
            DEFAULT_PROMPT.to_string()
        }
    }
}
