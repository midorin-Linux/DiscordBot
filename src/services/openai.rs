use anyhow::Result;
use async_openai::{
    Client as OpenAiClient,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
};

pub struct OpenAiService {
    client: OpenAiClient<OpenAIConfig>,
    model: String,
}

impl OpenAiService {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Self {
        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(base_url);

        let client = OpenAiClient::with_config(config);

        Self {
            client,
            model: model.to_string(),
        }
    }

    pub async fn create_chat_completion(
        &self,
        messages: Vec<ChatCompletionRequestMessage>,
    ) -> Result<String> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.model)
            .messages(messages)
            .build()?;

        let response = self.client.chat().create(request).await?;

        let content = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response content from OpenAI"))?;

        Ok(content)
    }

    pub fn create_system_message(&self, content: &str) -> Result<ChatCompletionRequestMessage> {
        Ok(ChatCompletionRequestSystemMessageArgs::default()
            .content(content)
            .build()?
            .into())
    }

    pub fn create_user_message(&self, content: &str) -> Result<ChatCompletionRequestMessage> {
        Ok(ChatCompletionRequestUserMessageArgs::default()
            .content(content)
            .build()?
            .into())
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn client(&self) -> &OpenAiClient<OpenAIConfig> {
        &self.client
    }
}
