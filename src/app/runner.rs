use crate::app::config::Config;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        if new_message.content == "!ping" {
            if let Err(e) = new_message.channel_id.say(&ctx.http, "Pong!").await {
                tracing::error!("Error sending message: {:?}", e)
            }
        }
    }

    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        tracing::info!("{} is connected to discord!", data_about_bot.user.name)
    }
}
pub struct App {
    config: Config
}

impl App {
    pub fn new(config: Config) -> Self { Self { config } }

    pub async fn run(&mut self) -> Result<()> {
        println!();

        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb.set_style(
            ProgressStyle::with_template("{spinner} {msg}")?
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message("Starting...");

        let intents = GatewayIntents::all();

        let mut client = Client::builder(&self.config.token, intents)
            .event_handler(Handler)
            .await
            .map_err(|e| anyhow::anyhow!("Error creating client: {:?}", e))?;

        client.start().await
            .map_err(|e| anyhow::anyhow!("Error starting client: {:?}", e))?;

        Ok(())
    }
}