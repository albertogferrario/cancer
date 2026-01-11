//! Job dispatching utilities.

use crate::{Error, Job, JobPayload, Queue};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

/// A pending job dispatch.
///
/// This builder allows configuring how a job is dispatched before
/// actually sending it to the queue.
pub struct PendingDispatch<J> {
    job: J,
    queue: Option<&'static str>,
    delay: Option<Duration>,
}

impl<J> PendingDispatch<J>
where
    J: Job + Serialize + DeserializeOwned,
{
    /// Create a new pending dispatch.
    pub fn new(job: J) -> Self {
        Self {
            job,
            queue: None,
            delay: None,
        }
    }

    /// Set the queue to dispatch to.
    pub fn on_queue(mut self, queue: &'static str) -> Self {
        self.queue = Some(queue);
        self
    }

    /// Set a delay before the job is available for processing.
    pub fn delay(mut self, duration: Duration) -> Self {
        self.delay = Some(duration);
        self
    }

    /// Dispatch the job to the queue.
    pub async fn dispatch(self) -> Result<(), Error> {
        let conn = Queue::connection();
        let queue = self.queue.unwrap_or(&conn.config().default_queue);

        let payload = match self.delay {
            Some(delay) => JobPayload::with_delay(&self.job, queue, delay)?,
            None => JobPayload::new(&self.job, queue)?,
        };

        conn.push(payload).await
    }

    /// Dispatch the job synchronously (fire and forget).
    ///
    /// This spawns the dispatch as a background task.
    pub fn dispatch_sync(self)
    where
        J: Send + 'static,
    {
        tokio::spawn(async move {
            if let Err(e) = self.dispatch().await {
                tracing::error!(error = %e, "Failed to dispatch job");
            }
        });
    }
}

/// Dispatch a job using the global queue.
///
/// # Example
///
/// ```rust,ignore
/// use cancer_queue::{dispatch, Job, Error};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct MyJob { data: String }
///
/// impl Job for MyJob {
///     async fn handle(&self) -> Result<(), Error> { Ok(()) }
/// }
///
/// dispatch(MyJob { data: "hello".into() }).await?;
/// ```
pub async fn dispatch<J>(job: J) -> Result<(), Error>
where
    J: Job + Serialize + DeserializeOwned,
{
    PendingDispatch::new(job).dispatch().await
}

/// Dispatch a job to a specific queue.
pub async fn dispatch_to<J>(job: J, queue: &'static str) -> Result<(), Error>
where
    J: Job + Serialize + DeserializeOwned,
{
    PendingDispatch::new(job).on_queue(queue).dispatch().await
}

/// Dispatch a job with a delay.
pub async fn dispatch_later<J>(job: J, delay: Duration) -> Result<(), Error>
where
    J: Job + Serialize + DeserializeOwned,
{
    PendingDispatch::new(job).delay(delay).dispatch().await
}
