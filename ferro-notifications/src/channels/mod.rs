//! Notification channel implementations.

mod database;
mod mail;
mod slack;

pub use database::DatabaseMessage;
pub use mail::MailMessage;
pub use slack::{SlackAttachment, SlackField, SlackMessage};
