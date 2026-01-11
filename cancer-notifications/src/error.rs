//! Error types for the notification system.

use thiserror::Error;

/// Errors that can occur during notification dispatch.
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to send email notification.
    #[error("mail error: {0}")]
    Mail(String),

    /// Failed to send Slack notification.
    #[error("slack error: {0}")]
    Slack(String),

    /// Failed to store database notification.
    #[error("database error: {0}")]
    Database(String),

    /// Channel not configured or available.
    #[error("channel not available: {0}")]
    ChannelNotAvailable(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Generic notification error.
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create a mail error.
    pub fn mail(msg: impl Into<String>) -> Self {
        Self::Mail(msg.into())
    }

    /// Create a slack error.
    pub fn slack(msg: impl Into<String>) -> Self {
        Self::Slack(msg.into())
    }

    /// Create a database error.
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }
}
