//! Notification channel abstraction.

use serde::{Deserialize, Serialize};

/// Available notification channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    /// Email notifications via SMTP.
    Mail,
    /// Database notifications for in-app delivery.
    Database,
    /// Slack webhook notifications.
    Slack,
    /// SMS notifications (future).
    Sms,
    /// Push notifications (future).
    Push,
}

impl Channel {
    /// Get channel name as string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Channel::Mail => "mail",
            Channel::Database => "database",
            Channel::Slack => "slack",
            Channel::Sms => "sms",
            Channel::Push => "push",
        }
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_as_str() {
        assert_eq!(Channel::Mail.as_str(), "mail");
        assert_eq!(Channel::Database.as_str(), "database");
        assert_eq!(Channel::Slack.as_str(), "slack");
    }

    #[test]
    fn test_channel_display() {
        assert_eq!(format!("{}", Channel::Mail), "mail");
    }
}
