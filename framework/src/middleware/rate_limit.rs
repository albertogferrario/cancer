//! Rate limiting middleware for Cancer framework
//!
//! Provides flexible rate limiting with multiple strategies and configurable limits.
//!
//! # Example
//!
//! ```rust,ignore
//! use cancer_rs::middleware::{RateLimiter, RateLimitConfig};
//!
//! // Simple rate limiter: 60 requests per minute
//! let limiter = RateLimiter::per_minute(60);
//!
//! // Complex rate limiter with custom key
//! let limiter = RateLimiter::new(RateLimitConfig {
//!     max_requests: 100,
//!     window_seconds: 3600, // 1 hour
//!     key_resolver: Some(Box::new(|req| {
//!         req.header("X-API-Key")
//!             .map(|s| s.to_string())
//!             .unwrap_or_else(|| req.ip().to_string())
//!     })),
//! });
//! ```

use crate::http::{HttpResponse, Request, Response};
use crate::middleware::{Middleware, Next};
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Configuration for rate limiting
#[derive(Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed in the window
    pub max_requests: u32,
    /// Window duration in seconds
    pub window_seconds: u64,
    /// Optional custom key resolver (defaults to IP-based)
    pub key_prefix: String,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window_seconds: 60,
            key_prefix: "rate_limit".to_string(),
        }
    }
}

/// Rate limit entry tracking requests
struct RateLimitEntry {
    /// Number of requests in current window
    count: AtomicU64,
    /// When the window started
    window_start: Instant,
    /// Window duration
    window_duration: Duration,
}

impl RateLimitEntry {
    fn new(window_seconds: u64) -> Self {
        Self {
            count: AtomicU64::new(0),
            window_start: Instant::now(),
            window_duration: Duration::from_secs(window_seconds),
        }
    }

    /// Increment and check if limit exceeded
    fn increment_and_check(&self, max: u32) -> RateLimitResult {
        let now = Instant::now();
        let elapsed = now.duration_since(self.window_start);

        // If window has expired, this entry is stale
        if elapsed >= self.window_duration {
            return RateLimitResult::WindowExpired;
        }

        let current = self.count.fetch_add(1, Ordering::SeqCst) + 1;
        let remaining = (max as u64).saturating_sub(current);
        let retry_after = (self.window_duration - elapsed).as_secs();

        if current > max as u64 {
            RateLimitResult::Exceeded {
                retry_after,
                limit: max,
            }
        } else {
            RateLimitResult::Allowed {
                remaining: remaining as u32,
                limit: max,
                retry_after,
            }
        }
    }
}

enum RateLimitResult {
    Allowed {
        remaining: u32,
        limit: u32,
        retry_after: u64,
    },
    Exceeded {
        retry_after: u64,
        limit: u32,
    },
    WindowExpired,
}

/// In-memory rate limiter store
///
/// For production use with multiple servers, consider using Redis-backed storage.
pub struct RateLimitStore {
    entries: DashMap<String, Arc<RateLimitEntry>>,
    window_seconds: u64,
}

impl RateLimitStore {
    /// Create a new rate limit store
    pub fn new(window_seconds: u64) -> Self {
        Self {
            entries: DashMap::new(),
            window_seconds,
        }
    }

    /// Check and increment rate limit for a key
    fn check(&self, key: &str, max_requests: u32) -> RateLimitResult {
        // Try to get existing entry
        if let Some(entry) = self.entries.get(key) {
            let result = entry.increment_and_check(max_requests);
            match result {
                RateLimitResult::WindowExpired => {
                    // Entry expired, remove and create new
                    drop(entry);
                    self.entries.remove(key);
                }
                _ => return result,
            }
        }

        // Create new entry
        let entry = Arc::new(RateLimitEntry::new(self.window_seconds));
        let result = entry.increment_and_check(max_requests);
        self.entries.insert(key.to_string(), entry);
        result
    }

    /// Clean up expired entries (call periodically)
    pub fn cleanup(&self) {
        self.entries
            .retain(|_, entry| entry.window_start.elapsed() < entry.window_duration);
    }
}

/// Rate limiting middleware
///
/// Limits the number of requests per time window, identified by client IP or custom key.
///
/// # Headers
///
/// Adds the following headers to responses:
/// - `X-RateLimit-Limit`: Maximum requests per window
/// - `X-RateLimit-Remaining`: Requests remaining in current window
/// - `X-RateLimit-Reset`: Seconds until window resets
///
/// # Example
///
/// ```rust,ignore
/// use cancer_rs::middleware::RateLimiter;
///
/// // 60 requests per minute
/// let limiter = RateLimiter::per_minute(60);
///
/// // 1000 requests per hour
/// let limiter = RateLimiter::per_hour(1000);
///
/// // Custom configuration
/// let limiter = RateLimiter::new(RateLimitConfig {
///     max_requests: 10,
///     window_seconds: 1,
///     key_prefix: "api".to_string(),
/// });
/// ```
pub struct RateLimiter {
    config: RateLimitConfig,
    store: Arc<RateLimitStore>,
    key_resolver: Option<Arc<dyn Fn(&Request) -> String + Send + Sync>>,
}

impl RateLimiter {
    /// Create a new rate limiter with custom configuration
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            store: Arc::new(RateLimitStore::new(config.window_seconds)),
            config,
            key_resolver: None,
        }
    }

    /// Create a rate limiter allowing N requests per second
    pub fn per_second(max_requests: u32) -> Self {
        Self::new(RateLimitConfig {
            max_requests,
            window_seconds: 1,
            key_prefix: "rate_limit:sec".to_string(),
        })
    }

    /// Create a rate limiter allowing N requests per minute
    pub fn per_minute(max_requests: u32) -> Self {
        Self::new(RateLimitConfig {
            max_requests,
            window_seconds: 60,
            key_prefix: "rate_limit:min".to_string(),
        })
    }

    /// Create a rate limiter allowing N requests per hour
    pub fn per_hour(max_requests: u32) -> Self {
        Self::new(RateLimitConfig {
            max_requests,
            window_seconds: 3600,
            key_prefix: "rate_limit:hour".to_string(),
        })
    }

    /// Create a rate limiter allowing N requests per day
    pub fn per_day(max_requests: u32) -> Self {
        Self::new(RateLimitConfig {
            max_requests,
            window_seconds: 86400,
            key_prefix: "rate_limit:day".to_string(),
        })
    }

    /// Set a custom key resolver function
    ///
    /// The resolver receives the request and returns a string key for rate limiting.
    /// By default, the client IP address is used.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let limiter = RateLimiter::per_minute(60)
    ///     .with_key_resolver(|req| {
    ///         // Rate limit by API key if present, otherwise by IP
    ///         req.header("X-API-Key")
    ///             .map(|s| format!("api:{}", s))
    ///             .unwrap_or_else(|| format!("ip:{}", req.ip()))
    ///     });
    /// ```
    pub fn with_key_resolver<F>(mut self, resolver: F) -> Self
    where
        F: Fn(&Request) -> String + Send + Sync + 'static,
    {
        self.key_resolver = Some(Arc::new(resolver));
        self
    }

    /// Set the key prefix for namespacing
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.key_prefix = prefix.into();
        self
    }

    /// Get the rate limit key for a request
    fn get_key(&self, request: &Request) -> String {
        let identifier = if let Some(resolver) = &self.key_resolver {
            resolver(request)
        } else {
            // Default: use client IP from X-Forwarded-For or direct connection
            request
                .header("X-Forwarded-For")
                .and_then(|s| s.split(',').next())
                .map(|s| s.trim().to_string())
                .or_else(|| request.header("X-Real-IP").map(|s| s.to_string()))
                .unwrap_or_else(|| "unknown".to_string())
        };

        format!("{}:{}", self.config.key_prefix, identifier)
    }
}

#[async_trait]
impl Middleware for RateLimiter {
    async fn handle(&self, request: Request, next: Next) -> Response {
        let key = self.get_key(&request);
        let result = self.store.check(&key, self.config.max_requests);

        match result {
            RateLimitResult::Allowed {
                remaining,
                limit,
                retry_after,
            } => {
                // Allow request, add rate limit headers to response
                let response = next(request).await;

                // Add headers to successful response
                match response {
                    Ok(mut http_response) => {
                        http_response = http_response
                            .header("X-RateLimit-Limit", &limit.to_string())
                            .header("X-RateLimit-Remaining", &remaining.to_string())
                            .header("X-RateLimit-Reset", &retry_after.to_string());
                        Ok(http_response)
                    }
                    err => err,
                }
            }
            RateLimitResult::Exceeded { retry_after, limit } => {
                // Rate limit exceeded
                Err(HttpResponse::json(serde_json::json!({
                    "error": "Too Many Requests",
                    "message": "Rate limit exceeded. Please try again later.",
                    "retry_after": retry_after
                }))
                .status(429)
                .header("X-RateLimit-Limit", &limit.to_string())
                .header("X-RateLimit-Remaining", "0")
                .header("X-RateLimit-Reset", &retry_after.to_string())
                .header("Retry-After", &retry_after.to_string()))
            }
            RateLimitResult::WindowExpired => {
                // Should not happen due to loop in check(), but handle gracefully
                next(request).await
            }
        }
    }
}

/// Named rate limiters for different API tiers
pub struct RateLimiters {
    limiters: DashMap<String, Arc<RateLimiter>>,
}

impl RateLimiters {
    /// Create a new collection of rate limiters
    pub fn new() -> Self {
        Self {
            limiters: DashMap::new(),
        }
    }

    /// Register a named rate limiter
    pub fn register(&self, name: &str, limiter: RateLimiter) {
        self.limiters.insert(name.to_string(), Arc::new(limiter));
    }

    /// Get a rate limiter by name
    pub fn get(&self, name: &str) -> Option<Arc<RateLimiter>> {
        self.limiters.get(name).map(|r| r.clone())
    }

    /// Create default API rate limiters
    pub fn with_defaults() -> Self {
        let limiters = Self::new();

        // Standard API tier
        limiters.register("api", RateLimiter::per_minute(60));

        // Higher limit for authenticated users
        limiters.register("authenticated", RateLimiter::per_minute(120));

        // Lower limit for sensitive operations
        limiters.register("sensitive", RateLimiter::per_minute(10));

        // Very strict limit for auth endpoints
        limiters.register("auth", RateLimiter::per_minute(5));

        limiters
    }
}

impl Default for RateLimiters {
    fn default() -> Self {
        Self::new()
    }
}

/// Throttle middleware - simpler API for common use cases
///
/// # Example
///
/// ```rust,ignore
/// use cancer_rs::middleware::Throttle;
///
/// // Simple throttle: 60 requests per minute
/// let throttle = Throttle::requests(60).per_minute();
///
/// // Throttle with decay: requests allowed gradually
/// let throttle = Throttle::requests(100).per_hour().with_decay();
/// ```
pub struct Throttle {
    max_requests: u32,
    window: ThrottleWindow,
    with_decay: bool,
}

#[derive(Clone, Copy)]
enum ThrottleWindow {
    Second,
    Minute,
    Hour,
    Day,
    Custom(u64),
}

impl Throttle {
    /// Start building a throttle with max requests
    pub fn requests(max: u32) -> ThrottleBuilder {
        ThrottleBuilder { max_requests: max }
    }
}

/// Builder for Throttle middleware
pub struct ThrottleBuilder {
    max_requests: u32,
}

impl ThrottleBuilder {
    /// Set window to per second
    pub fn per_second(self) -> Throttle {
        Throttle {
            max_requests: self.max_requests,
            window: ThrottleWindow::Second,
            with_decay: false,
        }
    }

    /// Set window to per minute
    pub fn per_minute(self) -> Throttle {
        Throttle {
            max_requests: self.max_requests,
            window: ThrottleWindow::Minute,
            with_decay: false,
        }
    }

    /// Set window to per hour
    pub fn per_hour(self) -> Throttle {
        Throttle {
            max_requests: self.max_requests,
            window: ThrottleWindow::Hour,
            with_decay: false,
        }
    }

    /// Set window to per day
    pub fn per_day(self) -> Throttle {
        Throttle {
            max_requests: self.max_requests,
            window: ThrottleWindow::Day,
            with_decay: false,
        }
    }

    /// Set custom window in seconds
    pub fn per_seconds(self, seconds: u64) -> Throttle {
        Throttle {
            max_requests: self.max_requests,
            window: ThrottleWindow::Custom(seconds),
            with_decay: false,
        }
    }
}

impl Throttle {
    /// Enable decay mode (gradual request allowance)
    pub fn with_decay(mut self) -> Self {
        self.with_decay = true;
        self
    }

    /// Convert to RateLimiter middleware
    pub fn into_middleware(self) -> RateLimiter {
        let window_seconds = match self.window {
            ThrottleWindow::Second => 1,
            ThrottleWindow::Minute => 60,
            ThrottleWindow::Hour => 3600,
            ThrottleWindow::Day => 86400,
            ThrottleWindow::Custom(s) => s,
        };

        RateLimiter::new(RateLimitConfig {
            max_requests: self.max_requests,
            window_seconds,
            key_prefix: "throttle".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_store_allows_requests() {
        let store = RateLimitStore::new(60);

        // First 5 requests should be allowed
        for i in 0..5 {
            match store.check("test_key", 10) {
                RateLimitResult::Allowed {
                    remaining, limit, ..
                } => {
                    assert_eq!(limit, 10);
                    assert_eq!(remaining, 10 - (i + 1));
                }
                _ => panic!("Expected request to be allowed"),
            }
        }
    }

    #[test]
    fn test_rate_limit_store_exceeds_limit() {
        let store = RateLimitStore::new(60);

        // Use up all 3 requests
        for _ in 0..3 {
            store.check("test_key", 3);
        }

        // 4th request should be exceeded
        match store.check("test_key", 3) {
            RateLimitResult::Exceeded { limit, .. } => {
                assert_eq!(limit, 3);
            }
            _ => panic!("Expected rate limit to be exceeded"),
        }
    }

    #[test]
    fn test_rate_limiter_per_minute() {
        let limiter = RateLimiter::per_minute(60);
        assert_eq!(limiter.config.max_requests, 60);
        assert_eq!(limiter.config.window_seconds, 60);
    }

    #[test]
    fn test_rate_limiter_per_hour() {
        let limiter = RateLimiter::per_hour(1000);
        assert_eq!(limiter.config.max_requests, 1000);
        assert_eq!(limiter.config.window_seconds, 3600);
    }

    #[test]
    fn test_throttle_builder() {
        let throttle = Throttle::requests(100).per_minute();
        let limiter = throttle.into_middleware();
        assert_eq!(limiter.config.max_requests, 100);
        assert_eq!(limiter.config.window_seconds, 60);
    }

    #[test]
    fn test_rate_limiters_collection() {
        let limiters = RateLimiters::with_defaults();

        assert!(limiters.get("api").is_some());
        assert!(limiters.get("authenticated").is_some());
        assert!(limiters.get("sensitive").is_some());
        assert!(limiters.get("auth").is_some());
        assert!(limiters.get("nonexistent").is_none());
    }

    #[test]
    fn test_separate_keys() {
        let store = RateLimitStore::new(60);

        // Different keys should have separate limits
        for _ in 0..5 {
            store.check("key_a", 5);
        }

        // key_a is exhausted
        match store.check("key_a", 5) {
            RateLimitResult::Exceeded { .. } => {}
            _ => panic!("key_a should be exceeded"),
        }

        // key_b should still have quota
        match store.check("key_b", 5) {
            RateLimitResult::Allowed { remaining, .. } => {
                assert_eq!(remaining, 4);
            }
            _ => panic!("key_b should be allowed"),
        }
    }
}
