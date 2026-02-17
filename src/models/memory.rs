use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversations {
    pub user_id: u32,
    pub channel_id: u32,
    pub title: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
    pub message_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub conversation_id: u32,
    pub role: MessageRole,
    pub content: String,
    pub tool_call: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub user_id: u32,
    pub max_history: u32,
    pub system_prompt: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Message {
    pub fn new(conversation_id: u32, role: MessageRole, content: String, tool_call: bool) -> Self {
        Self {
            conversation_id,
            role,
            content,
            tool_call,
            created_at: chrono::Utc::now(),
        }
    }
}