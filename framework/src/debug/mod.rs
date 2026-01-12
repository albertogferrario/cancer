//! Debug introspection endpoints for development
//!
//! These endpoints expose runtime application state for AI-assisted development
//! and debugging. They are automatically disabled in production.

use crate::config::Config;
use crate::middleware::get_global_middleware_info;
use crate::routing::get_registered_routes;
use bytes::Bytes;
use chrono::Utc;
use http_body_util::Full;
use serde::Serialize;

/// Response wrapper for debug endpoints
#[derive(Debug, Serialize)]
pub struct DebugResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub timestamp: String,
}

/// Error response for debug endpoints
#[derive(Debug, Serialize)]
pub struct DebugErrorResponse {
    pub success: bool,
    pub error: String,
    pub timestamp: String,
}

/// Check if debug endpoints should be enabled
pub fn is_debug_enabled() -> bool {
    // Disabled in production unless explicitly enabled
    if Config::is_production() {
        return std::env::var("CANCER_DEBUG_ENDPOINTS")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
    }
    true
}

/// Build a JSON response for debug endpoints
fn json_response<T: Serialize>(data: T, status: u16) -> hyper::Response<Full<Bytes>> {
    let body = serde_json::to_string_pretty(&data).unwrap_or_else(|_| "{}".to_string());
    hyper::Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(body)))
        .unwrap()
}

/// Handle /_cancer/routes endpoint
pub fn handle_routes() -> hyper::Response<Full<Bytes>> {
    if !is_debug_enabled() {
        return json_response(
            DebugErrorResponse {
                success: false,
                error: "Debug endpoints disabled in production".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
            403,
        );
    }

    let routes = get_registered_routes();
    json_response(
        DebugResponse {
            success: true,
            data: routes,
            timestamp: Utc::now().to_rfc3339(),
        },
        200,
    )
}

/// Global middleware info for introspection
#[derive(Debug, Serialize)]
pub struct MiddlewareInfo {
    pub global: Vec<String>,
}

/// Handle /_cancer/middleware endpoint
pub fn handle_middleware() -> hyper::Response<Full<Bytes>> {
    if !is_debug_enabled() {
        return json_response(
            DebugErrorResponse {
                success: false,
                error: "Debug endpoints disabled in production".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            },
            403,
        );
    }

    let global = get_global_middleware_info();
    json_response(
        DebugResponse {
            success: true,
            data: MiddlewareInfo { global },
            timestamp: Utc::now().to_rfc3339(),
        },
        200,
    )
}
