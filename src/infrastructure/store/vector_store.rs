use crate::models::memory::{LongTermMemory, MidTermMemory};

use anyhow::Result;
use qdrant_client::{
    Payload, Qdrant,
    qdrant::{
        Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, QueryPointsBuilder,
        UpsertPointsBuilder, VectorParamsBuilder,
    },
};

const MIDTERM_COLLECTION_NAME: &str = "midterm_memory";
const LONGTERM_COLLECTION_NAME: &str = "longterm_memory";

pub struct VectorStore {
    qdrant_client: Qdrant,
}

impl VectorStore {
    pub async fn new(url: &str) -> Result<Self> {
        let client = Qdrant::from_url(url).build()?;

        if !client.collection_exists(MIDTERM_COLLECTION_NAME).await? {
            client
                .create_collection(
                    CreateCollectionBuilder::new(MIDTERM_COLLECTION_NAME)
                        .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine)),
                )
                .await?;
        }

        if !client.collection_exists(LONGTERM_COLLECTION_NAME).await? {
            client
                .create_collection(
                    CreateCollectionBuilder::new(LONGTERM_COLLECTION_NAME)
                        .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine)),
                )
                .await?;
        }

        Ok(Self {
            qdrant_client: client,
        })
    }

    pub async fn store_longterm(&self, memory: LongTermMemory, embedding: Vec<f32>) -> Result<()> {
        let payload_json = serde_json::to_value(&memory)?;
        let payload: Payload = Payload::try_from(payload_json)
            .map_err(|_| anyhow::anyhow!("Failed to convert memory to payload"))?;

        self.qdrant_client
            .upsert_points(
                UpsertPointsBuilder::new(
                    LONGTERM_COLLECTION_NAME,
                    vec![PointStruct::new(memory.id.clone(), embedding, payload)],
                )
                .wait(true),
            )
            .await?;

        Ok(())
    }

    pub async fn store_midterm(&self, memory: MidTermMemory, embedding: Vec<f32>) -> Result<()> {
        let payload_json = serde_json::to_value(&memory)?;
        let payload: Payload = Payload::try_from(payload_json)
            .map_err(|_| anyhow::anyhow!("Failed to convert memory to payload"))?;

        self.qdrant_client
            .upsert_points(
                UpsertPointsBuilder::new(
                    MIDTERM_COLLECTION_NAME,
                    vec![PointStruct::new(memory.id.clone(), embedding, payload)],
                )
                .wait(true),
            )
            .await?;

        Ok(())
    }

    pub async fn search_longterm(
        &self,
        embedding: Vec<f32>,
        user_id: u64,
        limit: u64,
    ) -> Result<Vec<LongTermMemory>> {
        let response = self
            .qdrant_client
            .query(
                QueryPointsBuilder::new(LONGTERM_COLLECTION_NAME)
                    .query(embedding)
                    .filter(Filter::must([Condition::matches(
                        "user_id",
                        user_id as i64,
                    )]))
                    .limit(limit)
                    .with_payload(true),
            )
            .await?;

        let mut memories = Vec::new();
        for point in response.result {
            let payload_value = serde_json::to_value(point.payload)?;
            let memory: LongTermMemory = serde_json::from_value(payload_value)?;
            memories.push(memory);
        }

        Ok(memories)
    }

    pub async fn search_midterm(
        &self,
        embedding: Vec<f32>,
        user_id: u64,
        limit: u64,
    ) -> Result<Vec<MidTermMemory>> {
        let response = self
            .qdrant_client
            .query(
                QueryPointsBuilder::new(MIDTERM_COLLECTION_NAME)
                    .query(embedding)
                    .filter(Filter::must([Condition::matches(
                        "user_id",
                        user_id as i64,
                    )]))
                    .limit(limit)
                    .with_payload(true),
            )
            .await?;

        let mut memories = Vec::new();
        for point in response.result {
            let payload_value = serde_json::to_value(point.payload)?;
            let memory: MidTermMemory = serde_json::from_value(payload_value)?;
            memories.push(memory);
        }

        Ok(memories)
    }
}
