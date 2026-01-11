//! Mail notification channel.

use serde::{Deserialize, Serialize};

/// A mail message for email notifications.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailMessage {
    /// Email subject line.
    pub subject: String,
    /// Plain text body.
    pub body: String,
    /// Optional HTML body.
    pub html: Option<String>,
    /// From address (if different from default).
    pub from: Option<String>,
    /// Reply-to address.
    pub reply_to: Option<String>,
    /// CC recipients.
    pub cc: Vec<String>,
    /// BCC recipients.
    pub bcc: Vec<String>,
    /// Custom headers.
    pub headers: Vec<(String, String)>,
}

impl MailMessage {
    /// Create a new empty mail message.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the subject line.
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    /// Set the plain text body.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    /// Set the HTML body.
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    /// Set the from address.
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Set the reply-to address.
    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    /// Add a CC recipient.
    pub fn cc(mut self, email: impl Into<String>) -> Self {
        self.cc.push(email.into());
        self
    }

    /// Add a BCC recipient.
    pub fn bcc(mut self, email: impl Into<String>) -> Self {
        self.bcc.push(email.into());
        self
    }

    /// Add a custom header.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mail_message_builder() {
        let mail = MailMessage::new()
            .subject("Welcome!")
            .body("Hello, welcome to our service.")
            .html("<h1>Hello!</h1>")
            .from("noreply@example.com")
            .cc("manager@example.com")
            .bcc("archive@example.com");

        assert_eq!(mail.subject, "Welcome!");
        assert_eq!(mail.body, "Hello, welcome to our service.");
        assert_eq!(mail.html, Some("<h1>Hello!</h1>".into()));
        assert_eq!(mail.from, Some("noreply@example.com".into()));
        assert_eq!(mail.cc, vec!["manager@example.com"]);
        assert_eq!(mail.bcc, vec!["archive@example.com"]);
    }
}
