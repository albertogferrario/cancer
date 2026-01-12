//! Queue status tool - show pending, delayed, and failed jobs
//!
//! This tool fetches queue information from the running application via
//! the `/_cancer/queue/jobs` and `/_cancer/queue/stats` debug endpoints.

use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Queue status information
#[derive(Debug, Serialize)]
pub struct QueueStatusInfo {
    /// Jobs information (pending, delayed, failed)
    pub jobs: Option<QueueJobsSnapshot>,
    /// Queue statistics
    pub stats: Option<QueueStatsSnapshot>,
    /// Data source
    pub source: QueueSource,
}

/// Source of queue data
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueSource {
    /// Data fetched from running application via HTTP endpoint
    Runtime,
    /// App not running or endpoint unavailable
    Unavailable,
    /// Queue not initialized (sync mode)
    NotInitialized,
}

/// Response format from the `/_cancer/queue/jobs` endpoint
#[derive(Debug, Deserialize)]
struct DebugJobsResponse {
    success: bool,
    data: QueueJobsSnapshot,
}

/// Response format from the `/_cancer/queue/stats` endpoint
#[derive(Debug, Deserialize)]
struct DebugStatsResponse {
    success: bool,
    data: QueueStatsSnapshot,
}

/// Jobs snapshot from the runtime
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueueJobsSnapshot {
    /// Pending jobs (ready to process)
    pub pending: Vec<JobInfo>,
    /// Delayed jobs (waiting for available_at)
    pub delayed: Vec<JobInfo>,
    /// Failed jobs
    pub failed: Vec<FailedJobInfo>,
}

/// Queue statistics snapshot
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueueStatsSnapshot {
    /// Stats per queue
    pub queues: Vec<SingleQueueStats>,
    /// Total failed jobs count
    pub total_failed: usize,
}

/// Statistics for a single queue
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SingleQueueStats {
    /// Queue name
    pub name: String,
    /// Number of pending jobs
    pub pending: usize,
    /// Number of delayed jobs
    pub delayed: usize,
}

/// Job information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobInfo {
    /// Unique job ID
    pub id: String,
    /// Job type name
    pub job_type: String,
    /// Queue name
    pub queue: String,
    /// Number of attempts made
    pub attempts: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// When the job was created
    pub created_at: DateTime<Utc>,
    /// When the job should be available for processing
    pub available_at: DateTime<Utc>,
    /// Job state
    pub state: String,
}

/// Failed job information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FailedJobInfo {
    /// Job info
    pub job: JobInfo,
    /// Error message
    pub error: String,
    /// When the job failed
    pub failed_at: DateTime<Utc>,
}

/// Try to fetch jobs from the running application
async fn fetch_runtime_jobs(base_url: &str) -> Option<QueueJobsSnapshot> {
    let url = format!("{}/_cancer/queue/jobs", base_url);

    let response = reqwest::get(&url).await.ok()?;

    if !response.status().is_success() {
        return None;
    }

    let debug_response: DebugJobsResponse = response.json().await.ok()?;

    if !debug_response.success {
        return None;
    }

    Some(debug_response.data)
}

/// Try to fetch stats from the running application
async fn fetch_runtime_stats(base_url: &str) -> Option<QueueStatsSnapshot> {
    let url = format!("{}/_cancer/queue/stats", base_url);

    let response = reqwest::get(&url).await.ok()?;

    if !response.status().is_success() {
        return None;
    }

    let debug_response: DebugStatsResponse = response.json().await.ok()?;

    if !debug_response.success {
        return None;
    }

    Some(debug_response.data)
}

pub fn execute() -> Result<QueueStatusInfo> {
    // Try runtime endpoints (synchronously block on async)
    let rt = tokio::runtime::Handle::try_current();
    if let Ok(handle) = rt {
        let (jobs, stats) = handle.block_on(async {
            // Try common development ports
            for base_url in ["http://localhost:8000", "http://127.0.0.1:8000"] {
                let jobs = fetch_runtime_jobs(base_url).await;
                let stats = fetch_runtime_stats(base_url).await;

                // If we got either, we connected to the app
                if jobs.is_some() || stats.is_some() {
                    return (jobs, stats);
                }
            }
            (None, None)
        });

        if jobs.is_some() || stats.is_some() {
            return Ok(QueueStatusInfo {
                jobs,
                stats,
                source: QueueSource::Runtime,
            });
        }
    }

    // No data available - app not running or queue not initialized
    Ok(QueueStatusInfo {
        jobs: None,
        stats: None,
        source: QueueSource::Unavailable,
    })
}
