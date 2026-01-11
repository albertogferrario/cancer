//! Notification dispatcher for sending notifications through channels.

use crate::channel::Channel;
use crate::channels::{MailMessage, SlackMessage};
use crate::notifiable::Notifiable;
use crate::notification::Notification;
use crate::Error;
use std::sync::OnceLock;
use tracing::{error, info};

/// Global notification dispatcher configuration.
static CONFIG: OnceLock<NotificationConfig> = OnceLock::new();

/// Configuration for the notification dispatcher.
#[derive(Clone)]
pub struct NotificationConfig {
    /// SMTP configuration for mail notifications.
    pub mail: Option<MailConfig>,
    /// Slack webhook URL.
    pub slack_webhook: Option<String>,
}

/// SMTP mail configuration.
#[derive(Clone)]
pub struct MailConfig {
    /// SMTP host.
    pub host: String,
    /// SMTP port.
    pub port: u16,
    /// SMTP username.
    pub username: Option<String>,
    /// SMTP password.
    pub password: Option<String>,
    /// Default from address.
    pub from: String,
    /// Default from name.
    pub from_name: Option<String>,
    /// Use TLS.
    pub tls: bool,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            mail: None,
            slack_webhook: None,
        }
    }
}

impl MailConfig {
    /// Create a new mail config.
    pub fn new(host: impl Into<String>, port: u16, from: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            port,
            username: None,
            password: None,
            from: from.into(),
            from_name: None,
            tls: true,
        }
    }

    /// Set SMTP credentials.
    pub fn credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set the from name.
    pub fn from_name(mut self, name: impl Into<String>) -> Self {
        self.from_name = Some(name.into());
        self
    }

    /// Disable TLS.
    pub fn no_tls(mut self) -> Self {
        self.tls = false;
        self
    }
}

/// The notification dispatcher.
pub struct NotificationDispatcher;

impl NotificationDispatcher {
    /// Configure the global notification dispatcher.
    pub fn configure(config: NotificationConfig) {
        let _ = CONFIG.set(config);
    }

    /// Get the current configuration.
    pub fn config() -> Option<&'static NotificationConfig> {
        CONFIG.get()
    }

    /// Send a notification to a notifiable entity.
    pub async fn send<N, T>(notifiable: &N, notification: T) -> Result<(), Error>
    where
        N: Notifiable + ?Sized,
        T: Notification,
    {
        let channels = notification.via();
        let notification_type = notification.notification_type();

        info!(
            notification = notification_type,
            channels = ?channels,
            "Dispatching notification"
        );

        for channel in channels {
            match channel {
                Channel::Mail => {
                    if let Some(mail) = notification.to_mail() {
                        Self::send_mail(notifiable, &mail).await?;
                    }
                }
                Channel::Database => {
                    if let Some(db_msg) = notification.to_database() {
                        Self::send_database(notifiable, &db_msg).await?;
                    }
                }
                Channel::Slack => {
                    if let Some(slack) = notification.to_slack() {
                        Self::send_slack(notifiable, &slack).await?;
                    }
                }
                Channel::Sms | Channel::Push => {
                    // Not implemented yet
                    info!(channel = %channel, "Channel not implemented");
                }
            }
        }

        Ok(())
    }

    /// Send a mail notification.
    async fn send_mail<N: Notifiable + ?Sized>(
        notifiable: &N,
        message: &MailMessage,
    ) -> Result<(), Error> {
        let to = notifiable
            .route_notification_for(Channel::Mail)
            .ok_or_else(|| Error::ChannelNotAvailable("No mail route configured".into()))?;

        let config = CONFIG
            .get()
            .and_then(|c| c.mail.as_ref())
            .ok_or_else(|| Error::ChannelNotAvailable("Mail not configured".into()))?;

        info!(to = %to, subject = %message.subject, "Sending mail notification");

        // Build the email
        use lettre::message::{header::ContentType, Mailbox};
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

        let from: Mailbox = if let Some(ref name) = config.from_name {
            format!("{} <{}>", name, config.from)
                .parse()
                .map_err(|e| Error::mail(format!("Invalid from address: {}", e)))?
        } else {
            config
                .from
                .parse()
                .map_err(|e| Error::mail(format!("Invalid from address: {}", e)))?
        };

        let to_mailbox: Mailbox = to
            .parse()
            .map_err(|e| Error::mail(format!("Invalid to address: {}", e)))?;

        let mut email_builder = Message::builder()
            .from(from)
            .to(to_mailbox)
            .subject(&message.subject);

        // Add reply-to if specified
        if let Some(ref reply_to) = message.reply_to {
            let reply_to_mailbox: Mailbox = reply_to
                .parse()
                .map_err(|e| Error::mail(format!("Invalid reply-to address: {}", e)))?;
            email_builder = email_builder.reply_to(reply_to_mailbox);
        }

        // Add CC recipients
        for cc in &message.cc {
            let cc_mailbox: Mailbox = cc
                .parse()
                .map_err(|e| Error::mail(format!("Invalid CC address: {}", e)))?;
            email_builder = email_builder.cc(cc_mailbox);
        }

        // Add BCC recipients
        for bcc in &message.bcc {
            let bcc_mailbox: Mailbox = bcc
                .parse()
                .map_err(|e| Error::mail(format!("Invalid BCC address: {}", e)))?;
            email_builder = email_builder.bcc(bcc_mailbox);
        }

        // Build the message body
        let email = if let Some(ref html) = message.html {
            email_builder
                .header(ContentType::TEXT_HTML)
                .body(html.clone())
                .map_err(|e| Error::mail(format!("Failed to build email: {}", e)))?
        } else {
            email_builder
                .header(ContentType::TEXT_PLAIN)
                .body(message.body.clone())
                .map_err(|e| Error::mail(format!("Failed to build email: {}", e)))?
        };

        // Build the transport
        let transport = if config.tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|e| Error::mail(format!("Failed to create transport: {}", e)))?
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
        };

        let transport = transport.port(config.port);

        let transport = if let (Some(ref user), Some(ref pass)) =
            (&config.username, &config.password)
        {
            transport.credentials(Credentials::new(user.clone(), pass.clone()))
        } else {
            transport
        };

        let mailer = transport.build();

        // Send the email
        mailer
            .send(email)
            .await
            .map_err(|e| Error::mail(format!("Failed to send email: {}", e)))?;

        info!(to = %to, "Mail notification sent");
        Ok(())
    }

    /// Send a database notification.
    async fn send_database<N: Notifiable + ?Sized>(
        notifiable: &N,
        message: &crate::channels::DatabaseMessage,
    ) -> Result<(), Error> {
        let notifiable_id = notifiable.notifiable_id();
        let notifiable_type = notifiable.notifiable_type();

        info!(
            notifiable_id = %notifiable_id,
            notification_type = %message.notification_type,
            "Storing database notification"
        );

        // In a real implementation, this would store to the database.
        // For now, we just log it. The user should implement DatabaseNotificationStore.
        info!(
            notifiable_id = %notifiable_id,
            notifiable_type = %notifiable_type,
            notification_type = %message.notification_type,
            data = ?message.data,
            "Database notification stored (placeholder)"
        );

        Ok(())
    }

    /// Send a Slack notification.
    async fn send_slack<N: Notifiable + ?Sized>(
        notifiable: &N,
        message: &SlackMessage,
    ) -> Result<(), Error> {
        let webhook_url = notifiable
            .route_notification_for(Channel::Slack)
            .or_else(|| CONFIG.get().and_then(|c| c.slack_webhook.clone()))
            .ok_or_else(|| Error::ChannelNotAvailable("No Slack webhook configured".into()))?;

        info!(channel = ?message.channel, "Sending Slack notification");

        let client = reqwest::Client::new();
        let response = client
            .post(&webhook_url)
            .json(message)
            .send()
            .await
            .map_err(|e| Error::slack(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Slack webhook failed");
            return Err(Error::slack(format!("Slack returned {}: {}", status, body)));
        }

        info!("Slack notification sent");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mail_config_builder() {
        let config = MailConfig::new("smtp.example.com", 587, "noreply@example.com")
            .credentials("user", "pass")
            .from_name("My App");

        assert_eq!(config.host, "smtp.example.com");
        assert_eq!(config.port, 587);
        assert_eq!(config.from, "noreply@example.com");
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
        assert_eq!(config.from_name, Some("My App".to_string()));
        assert!(config.tls);
    }

    #[test]
    fn test_notification_config_default() {
        let config = NotificationConfig::default();
        assert!(config.mail.is_none());
        assert!(config.slack_webhook.is_none());
    }
}
