//! Request metrics tool - show performance data per route
//!
//! This tool fetches metrics from the running application via
//! the `/_cancer/metrics` debug endpoint.

use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MetricsInfo {
    pub metrics: MetricsSnapshot,
    /// Indicates whether metrics came from runtime
    pub source: MetricsSource,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricsSource {
    /// Metrics fetched from running application via HTTP endpoint
    Runtime,
    /// App not running or endpoint unavailable
    Unavailable,
}

/// Response format from the `/_cancer/metrics` endpoint
#[derive(Debug, Deserialize, Serialize)]
pub struct DebugResponse {
    pub success: bool,
    pub data: MetricsSnapshot,
}

/// Metrics snapshot from the runtime
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MetricsSnapshot {
    pub routes: Vec<RouteMetricsView>,
    pub total_requests: u64,
    pub total_errors: u64,
    pub uptime_seconds: u64,
}

/// Per-route metrics
#[derive(Debug, Clone, Deserialize, Serialize)]
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

/// Try to fetch metrics from the running application
async fn fetch_runtime_metrics(base_url: &str) -> Option<MetricsSnapshot> {
    let url = format!("{}/_cancer/metrics", base_url);

    let response = reqwest::get(&url).await.ok()?;

    if !response.status().is_success() {
        return None;
    }

    let debug_response: DebugResponse = response.json().await.ok()?;

    if !debug_response.success {
        return None;
    }

    Some(debug_response.data)
}

pub fn execute() -> Result<MetricsInfo> {
    // Try runtime endpoint (synchronously block on async)
    let rt = tokio::runtime::Handle::try_current();
    if let Ok(handle) = rt {
        let runtime_metrics = handle.block_on(async {
            // Try common development ports
            for base_url in ["http://localhost:8000", "http://127.0.0.1:8000"] {
                if let Some(metrics) = fetch_runtime_metrics(base_url).await {
                    return Some(metrics);
                }
            }
            None
        });

        if let Some(metrics) = runtime_metrics {
            return Ok(MetricsInfo {
                metrics,
                source: MetricsSource::Runtime,
            });
        }
    }

    // No metrics available - app not running
    Ok(MetricsInfo {
        metrics: MetricsSnapshot::default(),
        source: MetricsSource::Unavailable,
    })
}
