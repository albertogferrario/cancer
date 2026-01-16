//! List middleware tool - scan for registered middleware
//!
//! This tool tries to fetch middleware from the running application first via
//! the `/_ferro/middleware` debug endpoint, falling back to static file parsing
//! when the app isn't running.

use crate::error::Result;
use crate::introspection::middleware::{self, MiddlewareItem};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct MiddlewareInfo {
    pub middleware: Vec<MiddlewareItem>,
    /// Indicates whether middleware came from runtime or static analysis
    pub source: MiddlewareSource,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MiddlewareSource {
    /// Middleware fetched from running application via HTTP endpoint
    Runtime,
    /// Middleware parsed from source files (fallback when app not running)
    StaticAnalysis,
}

/// Response format from the `/_ferro/middleware` endpoint
#[derive(Debug, Deserialize)]
struct DebugResponse {
    success: bool,
    data: RuntimeMiddlewareInfo,
}

/// Middleware info as returned by the runtime endpoint
#[derive(Debug, Deserialize)]
struct RuntimeMiddlewareInfo {
    global: Vec<String>,
}

/// Try to fetch middleware from the running application
async fn fetch_runtime_middleware(base_url: &str) -> Option<Vec<MiddlewareItem>> {
    let url = format!("{}/_ferro/middleware", base_url);

    let response = reqwest::get(&url).await.ok()?;

    if !response.status().is_success() {
        return None;
    }

    let debug_response: DebugResponse = response.json().await.ok()?;

    if !debug_response.success {
        return None;
    }

    Some(
        debug_response
            .data
            .global
            .into_iter()
            .map(|name| MiddlewareItem {
                name,
                path: String::new(), // Runtime doesn't expose file paths
                global: true,
            })
            .collect(),
    )
}

pub fn execute(project_root: &Path) -> Result<MiddlewareInfo> {
    // Try runtime endpoint first (synchronously block on async)
    let rt = tokio::runtime::Handle::try_current();
    if let Ok(handle) = rt {
        let runtime_middleware = handle.block_on(async {
            // Try common development ports
            for base_url in ["http://localhost:8000", "http://127.0.0.1:8000"] {
                if let Some(middleware) = fetch_runtime_middleware(base_url).await {
                    return Some(middleware);
                }
            }
            None
        });

        if let Some(middleware) = runtime_middleware {
            return Ok(MiddlewareInfo {
                middleware,
                source: MiddlewareSource::Runtime,
            });
        }
    }

    // Fall back to static analysis
    let middleware_items = middleware::scan_middleware(project_root);
    Ok(MiddlewareInfo {
        middleware: middleware_items,
        source: MiddlewareSource::StaticAnalysis,
    })
}
