//! Queue connection and operations.

use crate::{Error, JobPayload, QueueConfig};
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::sync::Arc;
use tracing::debug;

/// A connection to the queue backend.
#[derive(Clone)]
pub struct QueueConnection {
    /// Redis connection manager.
    conn: ConnectionManager,
    /// Queue configuration.
    config: Arc<QueueConfig>,
}

impl QueueConnection {
    /// Create a new queue connection.
    pub async fn new(config: QueueConfig) -> Result<Self, Error> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        let conn = ConnectionManager::new(client)
            .await
            .map_err(|e| Error::ConnectionFailed(e.to_string()))?;

        Ok(Self {
            conn,
            config: Arc::new(config),
        })
    }

    /// Get the configuration.
    pub fn config(&self) -> &QueueConfig {
        &self.config
    }

    /// Push a job to a queue.
    pub async fn push(&self, payload: JobPayload) -> Result<(), Error> {
        let queue = &payload.queue;
        let json = payload.to_json()?;

        if payload.is_available() {
            // Push to the immediate queue
            let key = self.config.queue_key(queue);
            self.conn
                .clone()
                .lpush::<_, _, ()>(&key, &json)
                .await
                .map_err(Error::Redis)?;

            debug!(queue = queue, job_id = %payload.id, "Job pushed to queue");
        } else {
            // Push to the delayed queue (sorted set by available_at timestamp)
            let key = self.config.delayed_key(queue);
            let score = payload.available_at.timestamp() as f64;
            self.conn
                .clone()
                .zadd::<_, _, _, ()>(&key, &json, score)
                .await
                .map_err(Error::Redis)?;

            debug!(
                queue = queue,
                job_id = %payload.id,
                available_at = %payload.available_at,
                "Job pushed to delayed queue"
            );
        }

        Ok(())
    }

    /// Pop a job from a queue (blocking).
    pub async fn pop(&self, queue: &str) -> Result<Option<JobPayload>, Error> {
        let key = self.config.queue_key(queue);
        let timeout = self.config.block_timeout.as_secs() as f64;

        // BRPOP returns [key, value] or nil
        let result: Option<(String, String)> = self
            .conn
            .clone()
            .brpop(&key, timeout)
            .await
            .map_err(Error::Redis)?;

        match result {
            Some((_, json)) => {
                let mut payload = JobPayload::from_json(&json)?;
                payload.reserve();
                Ok(Some(payload))
            }
            None => Ok(None),
        }
    }

    /// Pop a job from a queue (non-blocking).
    pub async fn pop_nowait(&self, queue: &str) -> Result<Option<JobPayload>, Error> {
        let key = self.config.queue_key(queue);

        let result: Option<String> = self
            .conn
            .clone()
            .rpop(&key, None)
            .await
            .map_err(Error::Redis)?;

        match result {
            Some(json) => {
                let mut payload = JobPayload::from_json(&json)?;
                payload.reserve();
                Ok(Some(payload))
            }
            None => Ok(None),
        }
    }

    /// Move delayed jobs that are ready to the main queue.
    pub async fn migrate_delayed(&self, queue: &str) -> Result<usize, Error> {
        let delayed_key = self.config.delayed_key(queue);
        let queue_key = self.config.queue_key(queue);
        let now = chrono::Utc::now().timestamp() as f64;

        // Get jobs that are ready (score <= now)
        let ready_jobs: Vec<String> = self
            .conn
            .clone()
            .zrangebyscore(&delayed_key, "-inf", now)
            .await
            .map_err(Error::Redis)?;

        let count = ready_jobs.len();

        for job in ready_jobs {
            // Remove from delayed set
            self.conn
                .clone()
                .zrem::<_, _, ()>(&delayed_key, &job)
                .await
                .map_err(Error::Redis)?;

            // Push to main queue
            self.conn
                .clone()
                .lpush::<_, _, ()>(&queue_key, &job)
                .await
                .map_err(Error::Redis)?;
        }

        if count > 0 {
            debug!(queue = queue, count = count, "Migrated delayed jobs");
        }

        Ok(count)
    }

    /// Release a job back to the queue (for retry).
    pub async fn release(
        &self,
        mut payload: JobPayload,
        delay: std::time::Duration,
    ) -> Result<(), Error> {
        payload.increment_attempts();
        payload.reserved_at = None;

        if delay.is_zero() {
            payload.available_at = chrono::Utc::now();
        } else {
            payload.available_at =
                chrono::Utc::now() + chrono::Duration::from_std(delay).unwrap_or_default();
        }

        self.push(payload).await
    }

    /// Mark a job as failed.
    pub async fn fail(&self, payload: JobPayload, error: &Error) -> Result<(), Error> {
        let failed_key = self.config.failed_key();

        #[derive(serde::Serialize)]
        struct FailedJob {
            payload: JobPayload,
            error: String,
            failed_at: chrono::DateTime<chrono::Utc>,
        }

        let failed = FailedJob {
            payload,
            error: error.to_string(),
            failed_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&failed)
            .map_err(|e| Error::SerializationFailed(e.to_string()))?;

        self.conn
            .clone()
            .lpush::<_, _, ()>(&failed_key, &json)
            .await
            .map_err(Error::Redis)?;

        Ok(())
    }

    /// Get the number of jobs in a queue.
    pub async fn size(&self, queue: &str) -> Result<usize, Error> {
        let key = self.config.queue_key(queue);
        let len: usize = self.conn.clone().llen(&key).await.map_err(Error::Redis)?;
        Ok(len)
    }

    /// Get the number of delayed jobs in a queue.
    pub async fn delayed_size(&self, queue: &str) -> Result<usize, Error> {
        let key = self.config.delayed_key(queue);
        let len: usize = self.conn.clone().zcard(&key).await.map_err(Error::Redis)?;
        Ok(len)
    }

    /// Clear all jobs from a queue.
    pub async fn clear(&self, queue: &str) -> Result<(), Error> {
        let queue_key = self.config.queue_key(queue);
        let delayed_key = self.config.delayed_key(queue);

        self.conn
            .clone()
            .del::<_, ()>(&queue_key)
            .await
            .map_err(Error::Redis)?;
        self.conn
            .clone()
            .del::<_, ()>(&delayed_key)
            .await
            .map_err(Error::Redis)?;

        Ok(())
    }
}

/// Queue facade for static access.
pub struct Queue;

impl Queue {
    /// Get the global queue connection.
    pub fn connection() -> &'static QueueConnection {
        GLOBAL_CONNECTION
            .get()
            .expect("Queue not initialized. Call Queue::init() first.")
    }

    /// Initialize the global queue connection.
    pub async fn init(config: QueueConfig) -> Result<(), Error> {
        let conn = QueueConnection::new(config).await?;
        GLOBAL_CONNECTION
            .set(conn)
            .map_err(|_| Error::custom("Queue already initialized"))?;
        Ok(())
    }

    /// Check if the queue is initialized.
    pub fn is_initialized() -> bool {
        GLOBAL_CONNECTION.get().is_some()
    }
}

static GLOBAL_CONNECTION: std::sync::OnceLock<QueueConnection> = std::sync::OnceLock::new();
