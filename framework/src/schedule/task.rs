//! Scheduled task trait and entry types
//!
//! This module defines the `ScheduledTask` trait for creating struct-based
//! scheduled tasks, as well as internal types for task management.

use super::expression::CronExpression;
use crate::error::FrameworkError;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for boxed task handlers
pub type BoxedTask = Arc<dyn TaskHandler + Send + Sync>;

/// Type alias for async task result
pub type TaskResult = Result<(), FrameworkError>;

/// Type alias for boxed future result
pub type BoxedFuture<'a> = Pin<Box<dyn Future<Output = TaskResult> + Send + 'a>>;

/// Internal trait for task execution
///
/// This trait is implemented automatically for `ScheduledTask` and closure-based tasks.
#[async_trait]
pub trait TaskHandler: Send + Sync {
    /// Execute the task
    async fn handle(&self) -> TaskResult;
}

/// Trait for defining scheduled tasks
///
/// Implement this trait on a struct to create a reusable scheduled task.
/// The task will be automatically registered when added to the schedule.
///
/// # Example
///
/// ```rust,ignore
/// use kit::{ScheduledTask, CronExpression, FrameworkError};
/// use async_trait::async_trait;
///
/// pub struct CleanupLogsTask;
///
/// impl CleanupLogsTask {
///     pub fn new() -> Self {
///         Self
///     }
/// }
///
/// #[async_trait]
/// impl ScheduledTask for CleanupLogsTask {
///     fn name(&self) -> &str {
///         "cleanup:logs"
///     }
///
///     fn schedule(&self) -> CronExpression {
///         CronExpression::daily_at("03:00")
///     }
///
///     async fn handle(&self) -> Result<(), FrameworkError> {
///         // Cleanup logic here
///         println!("Cleaning up old log files...");
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait ScheduledTask: Send + Sync {
    /// Unique name for the task (used in logs and schedule:list)
    fn name(&self) -> &str;

    /// The cron expression for when this task should run
    fn schedule(&self) -> CronExpression;

    /// Execute the task
    async fn handle(&self) -> TaskResult;

    /// Optional description for the task (shown in schedule:list)
    fn description(&self) -> Option<&str> {
        None
    }

    /// Whether to prevent overlapping runs of this task
    ///
    /// When enabled, the scheduler will skip running this task if
    /// a previous run is still in progress.
    fn without_overlapping(&self) -> bool {
        false
    }

    /// Whether to run the task in background (non-blocking)
    ///
    /// When enabled, the scheduler won't wait for the task to complete
    /// before continuing to the next task.
    fn run_in_background(&self) -> bool {
        false
    }
}

// Implement TaskHandler for any type implementing ScheduledTask
#[async_trait]
impl<T: ScheduledTask> TaskHandler for T {
    async fn handle(&self) -> TaskResult {
        ScheduledTask::handle(self).await
    }
}

/// A registered task entry in the schedule
///
/// This struct holds all the information about a scheduled task,
/// including its schedule expression, configuration, and the task itself.
pub struct TaskEntry {
    /// Unique name for the task
    pub name: String,
    /// Cron expression defining when the task runs
    pub expression: CronExpression,
    /// The task handler
    pub task: BoxedTask,
    /// Optional description
    pub description: Option<String>,
    /// Prevent overlapping runs
    pub without_overlapping: bool,
    /// Run in background (non-blocking)
    pub run_in_background: bool,
}

impl TaskEntry {
    /// Check if this task is due to run now
    pub fn is_due(&self) -> bool {
        self.expression.is_due()
    }

    /// Run the task
    pub async fn run(&self) -> TaskResult {
        self.task.handle().await
    }

    /// Get a human-readable description of the schedule
    pub fn schedule_description(&self) -> &str {
        self.expression.expression()
    }
}

/// Wrapper for closure-based tasks
pub(crate) struct ClosureTask<F>
where
    F: Fn() -> BoxedFuture<'static> + Send + Sync,
{
    pub(crate) handler: F,
}

#[async_trait]
impl<F> TaskHandler for ClosureTask<F>
where
    F: Fn() -> BoxedFuture<'static> + Send + Sync,
{
    async fn handle(&self) -> TaskResult {
        (self.handler)().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTask;

    #[async_trait]
    impl ScheduledTask for TestTask {
        fn name(&self) -> &str {
            "test-task"
        }

        fn schedule(&self) -> CronExpression {
            CronExpression::every_minute()
        }

        async fn handle(&self) -> TaskResult {
            Ok(())
        }

        fn description(&self) -> Option<&str> {
            Some("A test task")
        }
    }

    #[tokio::test]
    async fn test_scheduled_task_trait() {
        let task = TestTask;
        assert_eq!(task.name(), "test-task");
        assert_eq!(task.description(), Some("A test task"));
        assert!(!task.without_overlapping());
        assert!(!task.run_in_background());

        let result: TaskResult = ScheduledTask::handle(&task).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_task_entry() {
        let task = TestTask;
        let entry = TaskEntry {
            name: task.name().to_string(),
            expression: task.schedule(),
            task: Arc::new(task),
            description: Some("A test task".to_string()),
            without_overlapping: false,
            run_in_background: false,
        };

        assert_eq!(entry.name, "test-task");
        assert_eq!(entry.schedule_description(), "* * * * *");

        let result = entry.run().await;
        assert!(result.is_ok());
    }
}
