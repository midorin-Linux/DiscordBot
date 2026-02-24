use anyhow::Result;
use config::{Config as ConfigBuilder, ConfigError, Environment, File};
use serde::Deserialize;

fn default_log_level() -> String {
    "info".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct NLP {
    pub api_url: String,
    pub model_name: String,
    pub max_short_term_messages: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Embedding {
    pub api_url: String,
    pub model_name: String,
    pub dimension: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub nlp_token: String,
    pub embed_token: String,
    pub discord_token: String,
    pub guild_id: u64,
    pub qdrant_url: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    pub nlp: NLP,
    pub embedding: Embedding,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let config = ConfigBuilder::builder()
            .add_source(
                File::with_name(".env")
                    .format(config::FileFormat::Ini)
                    .required(true),
            )
            .add_source(
                File::with_name("config/settings.toml")
                    .format(config::FileFormat::Toml)
                    .required(true),
            )
            .add_source(Environment::default().separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
