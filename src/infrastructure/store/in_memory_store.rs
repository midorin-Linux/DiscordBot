use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    application::traits::short_term_store::ShortTermStore, models::memory::ShortTermMessage,
};

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
}

#[async_trait]
impl ShortTermStore for InMemoryStore {
    async fn push(&self, channel_id: u64, message: ShortTermMessage) -> Vec<ShortTermMessage> {
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

    async fn get_context(&self, channel_id: u64) -> Vec<ShortTermMessage> {
        let store = self.conversations.read().await;
        store
            .get(&channel_id)
            .map(|q| q.iter().cloned().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::memory::Role;

    fn make_msg(content: &str) -> ShortTermMessage {
        ShortTermMessage {
            role: Role::User,
            user_id: 1,
            content: content.to_string(),
            timestamp: 0,
        }
    }

    #[tokio::test]
    async fn push_and_get_context() {
        let store = InMemoryStore::new(5);
        let overflow = store.push(100, make_msg("hello")).await;
        assert!(overflow.is_empty());

        let ctx = store.get_context(100).await;
        assert_eq!(ctx.len(), 1);
        assert_eq!(ctx[0].content, "hello");
    }

    #[tokio::test]
    async fn get_context_empty_channel() {
        let store = InMemoryStore::new(5);
        let ctx = store.get_context(999).await;
        assert!(ctx.is_empty());
    }

    #[tokio::test]
    async fn overflow_returns_oldest_messages() {
        let store = InMemoryStore::new(2);
        store.push(100, make_msg("msg1")).await;
        store.push(100, make_msg("msg2")).await;
        let overflow = store.push(100, make_msg("msg3")).await;

        assert_eq!(overflow.len(), 1);
        assert_eq!(overflow[0].content, "msg1");

        let ctx = store.get_context(100).await;
        assert_eq!(ctx.len(), 2);
        assert_eq!(ctx[0].content, "msg2");
        assert_eq!(ctx[1].content, "msg3");
    }

    #[tokio::test]
    async fn separate_channels_are_independent() {
        let store = InMemoryStore::new(5);
        store.push(100, make_msg("ch100")).await;
        store.push(200, make_msg("ch200")).await;

        let ctx100 = store.get_context(100).await;
        let ctx200 = store.get_context(200).await;
        assert_eq!(ctx100.len(), 1);
        assert_eq!(ctx200.len(), 1);
        assert_eq!(ctx100[0].content, "ch100");
        assert_eq!(ctx200[0].content, "ch200");
    }
}
