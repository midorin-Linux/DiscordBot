use async_trait::async_trait;

use crate::models::memory::ShortTermMessage;

#[async_trait]
pub trait ShortTermStore: Send + Sync {
    async fn push(&self, channel_id: u64, message: ShortTermMessage) -> Vec<ShortTermMessage>;

    async fn get_context(&self, channel_id: u64) -> Vec<ShortTermMessage>;
}
