use std::time::{SystemTime, UNIX_EPOCH};

use rig::completion::Message;
use uuid::Uuid;

use crate::{
    application::traits::{
        ai_client::AIClient, long_term_store::LongTermStore, short_term_store::ShortTermStore,
    },
    models::{error::AppError, memory::*},
};

pub async fn process_message(
    ai_client: &dyn AIClient,
    short_term_store: &dyn ShortTermStore,
    long_term_store: &dyn LongTermStore,
    channel_id: u64,
    user_id: u64,
    user_message: String,
) -> Result<String, AppError> {
    let in_memory_context = short_term_store.get_context(channel_id).await;

    let query_embedding = ai_client
        .embed(user_message.clone())
        .await
        .map_err(|e| AppError::Embedding(e.to_string()))?;

    let midterm_results = long_term_store
        .search_midterm(query_embedding.clone(), user_id, 3)
        .await
        .map_err(|e| AppError::Store(e.to_string()))?;

    let longterm_results = long_term_store
        .search_longterm(query_embedding.clone(), user_id, 5)
        .await
        .map_err(|e| AppError::Store(e.to_string()))?;

    let (prompt_message, chat_history) = build_messages(
        &user_message,
        &in_memory_context,
        &midterm_results,
        &longterm_results,
    );

    tracing::debug!("Sending {} messages in chat history", chat_history.len());

    let response = ai_client
        .generate(prompt_message, chat_history)
        .await
        .map_err(|e| AppError::AIGeneration(e.to_string()))?;

    let now = current_timestamp();
    let user_msg = ShortTermMessage {
        role: Role::User,
        user_id,
        content: user_message,
        timestamp: now,
    };
    let overflow = short_term_store.push(channel_id, user_msg).await;
    promote_overflow(ai_client, long_term_store, user_id, channel_id, overflow).await;

    let assistant_msg = ShortTermMessage {
        role: Role::Assistant,
        user_id,
        content: response.clone(),
        timestamp: current_timestamp(),
    };
    let overflow = short_term_store.push(channel_id, assistant_msg).await;
    promote_overflow(ai_client, long_term_store, user_id, channel_id, overflow).await;

    Ok(response)
}

async fn promote_overflow(
    ai_client: &dyn AIClient,
    long_term_store: &dyn LongTermStore,
    user_id: u64,
    channel_id: u64,
    overflow: Vec<ShortTermMessage>,
) {
    for msg in overflow {
        let role_str = match msg.role {
            Role::User => "user",
            Role::Assistant => "assistant",
        };
        let summary = format!("[{}] {}", role_str, msg.content);
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

        if let Err(err) = long_term_store.store_midterm(memory, embedding).await {
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
        let message = match msg.role {
            Role::Assistant => Message::assistant(msg.content.clone()),
            Role::User => Message::user(msg.content.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_messages_no_context() {
        let (prompt, history) = build_messages("hello", &[], &[], &[]);
        assert_eq!(history.len(), 0);
        assert_eq!(prompt, Message::user("hello".to_string()));
    }

    #[test]
    fn build_messages_with_short_context() {
        let short = vec![
            ShortTermMessage {
                role: Role::User,
                user_id: 1,
                content: "hi".to_string(),
                timestamp: 0,
            },
            ShortTermMessage {
                role: Role::Assistant,
                user_id: 1,
                content: "hello".to_string(),
                timestamp: 1,
            },
        ];

        let (prompt, history) = build_messages("how are you", &short, &[], &[]);
        assert_eq!(history.len(), 2);
        assert_eq!(prompt, Message::user("how are you".to_string()));
    }

    #[test]
    fn build_messages_with_longterm_context() {
        let longterm = vec![LongTermMemory {
            id: "1".to_string(),
            user_id: 1,
            fact: "Likes Rust".to_string(),
            category: "preference".to_string(),
            created_at: 0,
            updated_at: 0,
        }];

        let (_prompt, history) = build_messages("hello", &[], &[], &longterm);
        // Should have context injection pair (user + assistant)
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn build_messages_with_midterm_context() {
        let midterm = vec![MidTermMemory {
            id: "1".to_string(),
            user_id: 1,
            channel_id: 100,
            summary: "Discussed project".to_string(),
            created_at: 0,
            expires_at: 999,
        }];

        let (_prompt, history) = build_messages("hello", &[], &midterm, &[]);
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn build_messages_full_context() {
        let short = vec![ShortTermMessage {
            role: Role::User,
            user_id: 1,
            content: "prev msg".to_string(),
            timestamp: 0,
        }];
        let midterm = vec![MidTermMemory {
            id: "1".to_string(),
            user_id: 1,
            channel_id: 100,
            summary: "Past talk".to_string(),
            created_at: 0,
            expires_at: 999,
        }];
        let longterm = vec![LongTermMemory {
            id: "1".to_string(),
            user_id: 1,
            fact: "Likes cats".to_string(),
            category: "preference".to_string(),
            created_at: 0,
            updated_at: 0,
        }];

        let (prompt, history) = build_messages("new msg", &short, &midterm, &longterm);
        // 2 context injection + 1 short term
        assert_eq!(history.len(), 3);
        assert_eq!(prompt, Message::user("new msg".to_string()));
    }
}
