use anyhow::Result;
use async_trait::async_trait;

use crate::models::memory::{LongTermMemory, MidTermMemory};

#[async_trait]
pub trait LongTermStore: Send + Sync {
    async fn store_longterm(&self, memory: LongTermMemory, embedding: Vec<f32>) -> Result<()>;

    async fn store_midterm(&self, memory: MidTermMemory, embedding: Vec<f32>) -> Result<()>;

    async fn search_longterm(
        &self,
        embedding: Vec<f32>,
        user_id: u64,
        limit: u64,
    ) -> Result<Vec<LongTermMemory>>;

    async fn search_midterm(
        &self,
        embedding: Vec<f32>,
        user_id: u64,
        limit: u64,
    ) -> Result<Vec<MidTermMemory>>;

    async fn delete_expired_midterm(&self) -> Result<()>;
}
