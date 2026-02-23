use crate::application::traits::ai_client::AIClient;
use crate::infrastructure::store::{in_memory_store::InMemoryStore, vector_store::VectorStore};
use crate::models::memory::*;
use anyhow::Result;
use rig::completion::Message;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn process_message(
    ai_client: &impl AIClient,
    in_memory_store: &InMemoryStore,
    vector_store: &VectorStore,
    channel_id: u64,
    user_id: u64,
    user_message: String,
) -> Result<String> {
    let in_memory_context = in_memory_store.get_context(channel_id).await;

    let query_embedding = ai_client.embed(user_message.clone()).await?;
    let midterm_results = vector_store
        .search_midterm(query_embedding.clone(), user_id, 3)
        .await?;

    let longterm_results = vector_store
        .search_longterm(query_embedding.clone(), user_id, 5)
        .await?;

    let (prompt_message, chat_history) = build_messages(
        &user_message,
        &in_memory_context,
        &midterm_results,
        &longterm_results,
    );

    tracing::debug!("Sending {} messages in chat history", chat_history.len());

    let response = ai_client.generate(prompt_message, chat_history).await?;

    let now = current_timestamp();
    let user_msg = ShortTermMessage {
        role: "user".to_string(),
        user_id,
        content: user_message,
        timestamp: now,
    };
    let overflow = in_memory_store.push(channel_id, user_msg).await;
    promote_overflow(ai_client, vector_store, user_id, channel_id, overflow).await;

    let assistant_msg = ShortTermMessage {
        role: "assistant".to_string(),
        user_id,
        content: response.clone(),
        timestamp: current_timestamp(),
    };
    let overflow = in_memory_store.push(channel_id, assistant_msg).await;
    promote_overflow(ai_client, vector_store, user_id, channel_id, overflow).await;

    Ok(response)
}

async fn promote_overflow(
    ai_client: &impl AIClient,
    vector_store: &VectorStore,
    user_id: u64,
    channel_id: u64,
    overflow: Vec<ShortTermMessage>,
) {
    for msg in overflow {
        let summary = format!("[{}] {}", msg.role, msg.content);
        let embedding = match ai_client.embed(summary.clone()).await {
            Ok(e) => e,
            Err(err) => {
                tracing::warn!("Failed to embed overflow message for midterm: {err}");
                continue;
            }
        };

        let now = current_timestamp();
        let memory = MidTermMemory {
            id: Uuid::new_v4().to_string(),
            user_id,
            channel_id,
            summary,
            created_at: now,
            expires_at: now + 60 * 60 * 24 * 7, // 7日
        };

        if let Err(err) = vector_store.store_midterm(memory, embedding).await {
            tracing::warn!("Failed to store midterm memory: {err}");
        }
    }
}

fn build_messages(
    user_message: &str,
    short_context: &[ShortTermMessage],
    midterm: &[MidTermMemory],
    longterm: &[LongTermMemory],
) -> (Message, Vec<Message>) {
    let mut history: Vec<Message> = Vec::new();

    if !longterm.is_empty() || !midterm.is_empty() {
        let mut context_text = String::new();

        if !longterm.is_empty() {
            context_text.push_str("[What we know about this user]\n");
            for point in longterm {
                context_text.push_str(&format!("- {}\n", point.fact));
            }
            context_text.push('\n');
        }

        if !midterm.is_empty() {
            context_text.push_str("[Summarizing relevant past conversations]\n");
            for point in midterm {
                context_text.push_str(&format!("- {}\n", point.summary));
            }
        }

        history.push(Message::user(context_text.trim_end().to_string()));
        history.push(Message::assistant(
            "Understood. I will use this context in our conversation.",
        ));
    }

    for msg in short_context {
        let message = match msg.role.as_str() {
            "assistant" => Message::assistant(msg.content.clone()),
            _ => Message::user(msg.content.clone()),
        };
        history.push(message);
    }

    let prompt = Message::user(user_message.to_string());

    (prompt, history)
}

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
