use anyhow::Result;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    content: String,
    target_channel_id: u64,
}

#[derive(Debug, thiserror::Error)]
#[error("Discord message send error")]
pub struct DiscordMessageSendError;

#[derive(Deserialize, Serialize)]
pub struct Test;
impl Tool for Test {
    const NAME: &'static str = "add";
    type Error = DiscordMessageSendError;
    type Args = OperationArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "send_message".to_string(),
            description: "Send message to target channel".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "content": {
                        "type": "text",
                        "description": "Message content"
                    },
                    "target_channel_id": {
                        "type": "number",
                        "description": "Target channel ID"
                    },
                },
                "required": ["content", "target_channel_id"],
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!(
            "[tool-call] Adding {} and {}",
            args.content, args.target_channel_id
        );
        let result = "Succeccfly send message".to_string();
        Ok(result)
    }
}
