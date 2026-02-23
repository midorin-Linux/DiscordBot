use crate::models::memory::ShortTermMessage;
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use tokio::sync::RwLock;

pub struct InMemoryStore {
    conversations: Arc<RwLock<HashMap<u64, VecDeque<ShortTermMessage>>>>,
    max_short_term_messages: usize,
}

impl InMemoryStore {
    pub fn new(max_short_term_messages: usize) -> Self {
        Self {
            conversations: Arc::new(RwLock::new(HashMap::new())),
            max_short_term_messages,
        }
    }

    pub async fn push(&self, channel_id: u64, message: ShortTermMessage) -> Vec<ShortTermMessage> {
        let mut store = self.conversations.write().await;
        let queue = store.entry(channel_id).or_insert_with(VecDeque::new);
        queue.push_back(message);

        let mut overflow = Vec::new();
        while queue.len() > self.max_short_term_messages {
            if let Some(old) = queue.pop_front() {
                overflow.push(old);
            }
        }
        overflow
    }

    pub async fn get_context(&self, channel_id: u64) -> Vec<ShortTermMessage> {
        let store = self.conversations.read().await;
        store
            .get(&channel_id)
            .map(|q| q.iter().cloned().collect())
            .unwrap_or_default()
    }
}
