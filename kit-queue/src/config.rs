//! Queue configuration.

use std::time::Duration;

/// Queue system configuration.
#[derive(Debug, Clone)]
pub struct QueueConfig {
    /// Redis connection URL.
    pub redis_url: String,
    /// Default queue name.
    pub default_queue: String,
    /// Prefix for queue keys in Redis.
    pub prefix: String,
    /// How long to block waiting for jobs (in seconds).
    pub block_timeout: Duration,
    /// Maximum number of concurrent jobs per worker.
    pub max_concurrent_jobs: usize,
    /// How often to check for delayed jobs.
    pub delayed_job_poll_interval: Duration,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://127.0.0.1:6379".to_string(),
            default_queue: "default".to_string(),
            prefix: "kit_queue".to_string(),
            block_timeout: Duration::from_secs(5),
            max_concurrent_jobs: 10,
            delayed_job_poll_interval: Duration::from_secs(1),
        }
    }
}

impl QueueConfig {
    /// Create a new configuration with a Redis URL.
    pub fn new(redis_url: impl Into<String>) -> Self {
        Self {
            redis_url: redis_url.into(),
            ..Default::default()
        }
    }

    /// Set the default queue name.
    pub fn default_queue(mut self, queue: impl Into<String>) -> Self {
        self.default_queue = queue.into();
        self
    }

    /// Set the key prefix.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    /// Set the block timeout.
    pub fn block_timeout(mut self, timeout: Duration) -> Self {
        self.block_timeout = timeout;
        self
    }

    /// Set max concurrent jobs.
    pub fn max_concurrent_jobs(mut self, count: usize) -> Self {
        self.max_concurrent_jobs = count;
        self
    }

    /// Get the Redis key for a queue.
    pub fn queue_key(&self, queue: &str) -> String {
        format!("{}:{}", self.prefix, queue)
    }

    /// Get the Redis key for delayed jobs.
    pub fn delayed_key(&self, queue: &str) -> String {
        format!("{}:{}:delayed", self.prefix, queue)
    }

    /// Get the Redis key for reserved jobs.
    pub fn reserved_key(&self, queue: &str) -> String {
        format!("{}:{}:reserved", self.prefix, queue)
    }

    /// Get the Redis key for failed jobs.
    pub fn failed_key(&self) -> String {
        format!("{}:failed", self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = QueueConfig::default();
        assert_eq!(config.default_queue, "default");
        assert_eq!(config.prefix, "kit_queue");
    }

    #[test]
    fn test_queue_key() {
        let config = QueueConfig::default();
        assert_eq!(config.queue_key("emails"), "kit_queue:emails");
        assert_eq!(config.delayed_key("emails"), "kit_queue:emails:delayed");
    }

    #[test]
    fn test_builder_pattern() {
        let config = QueueConfig::new("redis://localhost:6380")
            .default_queue("high-priority")
            .prefix("myapp")
            .max_concurrent_jobs(5);

        assert_eq!(config.redis_url, "redis://localhost:6380");
        assert_eq!(config.default_queue, "high-priority");
        assert_eq!(config.prefix, "myapp");
        assert_eq!(config.max_concurrent_jobs, 5);
    }
}
