use crate::infrastructure::ai::tools::*;
use crate::shared::config::{Model, Provider};

use anyhow::Result;
use rig::prelude::*;
use rig::{providers, tool::Tool};

pub struct RigClient {
    rig_client: rig::agent::Agent<providers::openai::responses_api::ResponsesCompletionModel>,
}

impl RigClient {
    pub async fn new(api_key: String, provider: Provider, model: Model) -> Result<Self> {
        let system_instruction = std::fs::read_to_string("INSTRUCTION.md")?;

        let openai_comp_client = providers::openai::Client::builder()
            .api_key(api_key)
            .base_url(provider.api_url)
            .build()?;

        let discord_agent = openai_comp_client
            .agent(model.name)
            .preamble(system_instruction.as_str())
            .tool(test::Test)
            .build();

        Ok(Self {
            rig_client: discord_agent,
        })
    }

    pub async fn run(self) -> Result<()> {
        Ok(())
    }
}
