use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortTermMessage {
    pub role: String,
    pub user_id: u64,
    pub content: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidTermMemory {
    pub id: String,
    pub user_id: u64,
    pub channel_id: u64,
    pub summary: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermMemory {
    pub id: String,
    pub user_id: u64,
    pub fact: String,
    pub category: String,
    pub created_at: i64,
    pub updated_at: i64,
}
