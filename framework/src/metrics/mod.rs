//! Request metrics collection for performance monitoring
//!
//! Collects request counts, response times, and error rates per route.
//! Metrics are stored in-memory and exposed via `/_cancer/metrics`.

use serde::Serialize;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::time::Duration;

/// Request metrics for a single route
#[derive(Debug, Clone, Serialize)]
pub struct RouteMetrics {
    /// Route pattern (e.g., "/users/{id}")
    pub route: String,
    /// HTTP method
    pub method: String,
    /// Total request count
    pub count: u64,
    /// Total duration in milliseconds (for average calculation)
    pub total_duration_ms: u64,
    /// Number of error responses (4xx and 5xx)
    pub error_count: u64,
    /// Minimum response time in ms
    pub min_duration_ms: u64,
    /// Maximum response time in ms
    pub max_duration_ms: u64,
}

impl RouteMetrics {
    fn new(route: String, method: String) -> Self {
        Self {
            route,
            method,
            count: 0,
            total_duration_ms: 0,
            error_count: 0,
            min_duration_ms: u64::MAX,
            max_duration_ms: 0,
        }
    }

    /// Calculate average response time in milliseconds
    pub fn avg_duration_ms(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_duration_ms as f64 / self.count as f64
        }
    }
}

/// Aggregated metrics response
#[derive(Debug, Serialize)]
pub struct MetricsSnapshot {
    /// Per-route metrics
    pub routes: Vec<RouteMetricsView>,
    /// Total requests across all routes
    pub total_requests: u64,
    /// Total errors across all routes
    pub total_errors: u64,
    /// Uptime since metrics collection started (seconds)
    pub uptime_seconds: u64,
}

/// View of route metrics for serialization (includes computed avg)
#[derive(Debug, Serialize)]
pub struct RouteMetricsView {
    pub route: String,
    pub method: String,
    pub count: u64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: Option<u64>,
    pub max_duration_ms: u64,
    pub error_count: u64,
    pub error_rate: f64,
}

/// Global metrics storage
static METRICS: OnceLock<RwLock<MetricsStore>> = OnceLock::new();

struct MetricsStore {
    routes: HashMap<String, RouteMetrics>,
    start_time: std::time::Instant,
}

impl MetricsStore {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
            start_time: std::time::Instant::now(),
        }
    }
}

fn get_store() -> &'static RwLock<MetricsStore> {
    METRICS.get_or_init(|| RwLock::new(MetricsStore::new()))
}

/// Generate a unique key for route metrics
fn route_key(method: &str, route: &str) -> String {
    format!("{}:{}", method, route)
}

/// Record a request completion
///
/// # Arguments
/// * `route` - Route pattern (e.g., "/users/{id}")
/// * `method` - HTTP method
/// * `duration` - Request duration
/// * `is_error` - Whether response was an error (4xx or 5xx)
pub fn record_request(route: &str, method: &str, duration: Duration, is_error: bool) {
    let key = route_key(method, route);
    let duration_ms = duration.as_millis() as u64;

    if let Ok(mut store) = get_store().write() {
        let metrics = store
            .routes
            .entry(key)
            .or_insert_with(|| RouteMetrics::new(route.to_string(), method.to_string()));

        metrics.count += 1;
        metrics.total_duration_ms += duration_ms;

        if duration_ms < metrics.min_duration_ms {
            metrics.min_duration_ms = duration_ms;
        }
        if duration_ms > metrics.max_duration_ms {
            metrics.max_duration_ms = duration_ms;
        }

        if is_error {
            metrics.error_count += 1;
        }
    }
}

/// Get current metrics snapshot
pub fn get_metrics() -> MetricsSnapshot {
    let store = get_store().read().unwrap();

    let mut total_requests = 0u64;
    let mut total_errors = 0u64;

    let routes: Vec<RouteMetricsView> = store
        .routes
        .values()
        .map(|m| {
            total_requests += m.count;
            total_errors += m.error_count;

            RouteMetricsView {
                route: m.route.clone(),
                method: m.method.clone(),
                count: m.count,
                avg_duration_ms: m.avg_duration_ms(),
                min_duration_ms: if m.min_duration_ms == u64::MAX {
                    None
                } else {
                    Some(m.min_duration_ms)
                },
                max_duration_ms: m.max_duration_ms,
                error_count: m.error_count,
                error_rate: if m.count == 0 {
                    0.0
                } else {
                    m.error_count as f64 / m.count as f64
                },
            }
        })
        .collect();

    MetricsSnapshot {
        routes,
        total_requests,
        total_errors,
        uptime_seconds: store.start_time.elapsed().as_secs(),
    }
}

/// Reset all metrics (useful for testing)
pub fn reset_metrics() {
    if let Ok(mut store) = get_store().write() {
        store.routes.clear();
        store.start_time = std::time::Instant::now();
    }
}

/// Check if metrics collection is enabled
pub fn is_enabled() -> bool {
    std::env::var("CANCER_COLLECT_METRICS")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(true) // Enabled by default in development
}
