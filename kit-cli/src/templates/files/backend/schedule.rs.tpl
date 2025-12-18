//! Task Scheduler Registration
//!
//! Register your scheduled tasks here. Tasks can be defined as:
//! - Struct implementing `ScheduledTask` trait (recommended for complex tasks)
//! - Inline closures with fluent schedule configuration (quick tasks)
//!
//! # Examples
//!
//! ## Trait-Based Task
//!
//! ```rust,ignore
//! // In src/tasks/cleanup_logs.rs
//! use kit::{ScheduledTask, CronExpression, FrameworkError};
//! use async_trait::async_trait;
//!
//! pub struct CleanupLogsTask;
//!
//! #[async_trait]
//! impl ScheduledTask for CleanupLogsTask {
//!     fn name(&self) -> &str { "cleanup:logs" }
//!     fn schedule(&self) -> CronExpression { CronExpression::daily_at("03:00") }
//!     async fn handle(&self) -> Result<(), FrameworkError> {
//!         // Your task logic
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Closure-Based Task
//!
//! ```rust,ignore
//! schedule.add(
//!     schedule.call(|| async {
//!         println!("Running hourly!");
//!         Ok(())
//!     }).hourly().name("hourly-ping")
//! );
//! ```
//!
//! # Running Tasks
//!
//! ```bash
//! # Run due tasks once (for cron)
//! kit schedule:run
//!
//! # Run as daemon (checks every minute)
//! kit schedule:work
//!
//! # List all tasks
//! kit schedule:list
//! ```

use kit::Schedule;

// Import your tasks here
// use crate::tasks;

/// Register all scheduled tasks
///
/// Called by the schedule binary when starting the scheduler.
pub fn register(schedule: &mut Schedule) {
    // Example: Register a trait-based task
    // schedule.task(tasks::CleanupLogsTask::new());

    // Example: Register a closure-based task
    // schedule.add(
    //     schedule.call(|| async {
    //         println!("Hello from scheduler!");
    //         Ok(())
    //     }).daily().at("03:00").name("daily-greeting")
    // );
}
