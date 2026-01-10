//! # Kit Events
//!
//! Event dispatcher and listener system for the Kit framework.
//!
//! Provides a Laravel-inspired event system with support for:
//! - Synchronous listeners
//! - Asynchronous listeners
//! - Queued listeners (via `ShouldQueue` marker)
//! - Automatic listener discovery via inventory
//!
//! ## Example
//!
//! ```rust,ignore
//! use kit_events::{Event, Listener, dispatch};
//!
//! #[derive(Clone)]
//! struct UserRegistered {
//!     user_id: i64,
//!     email: String,
//! }
//!
//! impl Event for UserRegistered {
//!     fn name(&self) -> &'static str {
//!         "UserRegistered"
//!     }
//! }
//!
//! struct SendWelcomeEmail;
//!
//! #[async_trait::async_trait]
//! impl Listener<UserRegistered> for SendWelcomeEmail {
//!     async fn handle(&self, event: &UserRegistered) -> Result<(), kit_events::Error> {
//!         println!("Sending welcome email to {}", event.email);
//!         Ok(())
//!     }
//! }
//!
//! // Dispatch an event
//! dispatch(UserRegistered { user_id: 1, email: "test@example.com".into() }).await;
//! ```

mod dispatcher;
mod error;
mod traits;

pub use dispatcher::{dispatch, dispatch_sync, EventDispatcher};
pub use error::Error;
pub use traits::{Event, Listener, ShouldQueue};

/// Re-export async_trait for convenience
pub use async_trait::async_trait;
