//! Scheduled Tasks
//!
//! This module contains all scheduled task definitions.
//! Use `kit make:task <name>` to generate new tasks.
//!
//! # Creating Tasks
//!
//! ```bash
//! kit make:task CleanupLogs
//! kit make:task SendReminders
//! ```
//!
//! # Example Task
//!
//! ```rust,ignore
//! use kit::{ScheduledTask, CronExpression, FrameworkError};
//! use async_trait::async_trait;
//!
//! pub struct MyTask;
//!
//! impl MyTask {
//!     pub fn new() -> Self { Self }
//! }
//!
//! #[async_trait]
//! impl ScheduledTask for MyTask {
//!     fn name(&self) -> &str { "my:task" }
//!
//!     fn schedule(&self) -> CronExpression {
//!         CronExpression::daily_at("09:00")
//!     }
//!
//!     async fn handle(&self) -> Result<(), FrameworkError> {
//!         println!("Task running!");
//!         Ok(())
//!     }
//! }
//! ```

// Tasks will be added here by the make:task command
