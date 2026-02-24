use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("AI generation error: {0}")]
    AIGeneration(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("Store error: {0}")]
    Store(String),

    #[error("Discord error: {0}")]
    Discord(#[from] serenity::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Conversation not found: channel {0}")]
    ConversationNotFound(u64),

    #[error("Permission denied: {reason}")]
    PermissionDenied { reason: String },

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}
