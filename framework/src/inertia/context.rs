//! Inertia.js integration - async-safe implementation.
//!
//! This module provides the main `Inertia` struct for rendering Inertia responses.
//! It wraps the framework-agnostic `inertia-rs` crate with Cancer-specific features.

use crate::csrf::csrf_token;
use crate::http::{HttpResponse, Request};
use crate::Response;
use inertia_rs::{InertiaConfig, InertiaRequest as InertiaRequestTrait};
use serde::Serialize;

// Re-export InertiaShared from inertia-rs
pub use inertia_rs::InertiaShared;

/// Implement the framework-agnostic InertiaRequest trait for Cancer's Request type.
impl InertiaRequestTrait for Request {
    fn inertia_header(&self, name: &str) -> Option<&str> {
        self.header(name)
    }

    fn path(&self) -> &str {
        Request::path(self)
    }
}

/// Main Inertia integration struct for Cancer framework.
///
/// Provides methods for rendering Inertia responses in an async-safe manner.
/// All state is derived from the Request, not thread-local storage.
pub struct Inertia;

impl Inertia {
    /// Render an Inertia response.
    ///
    /// This is the primary method for returning Inertia responses from controllers.
    /// It automatically:
    /// - Detects XHR vs initial page load
    /// - Merges shared props from middleware
    /// - Filters props for partial reloads
    /// - Includes CSRF token in HTML responses
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use cancer::{Inertia, Request, Response};
    ///
    /// pub async fn index(req: Request) -> Response {
    ///     Inertia::render(&req, "Home", HomeProps {
    ///         title: "Welcome".into(),
    ///     })
    /// }
    /// ```
    pub fn render<P: Serialize>(req: &Request, component: &str, props: P) -> Response {
        Self::render_with_config(req, component, props, InertiaConfig::default())
    }

    /// Render an Inertia response with custom configuration.
    pub fn render_with_config<P: Serialize>(
        req: &Request,
        component: &str,
        props: P,
        config: InertiaConfig,
    ) -> Response {
        // Get shared props from middleware (if set)
        let shared = req.get::<InertiaShared>();

        // Get CSRF token for HTML responses
        let csrf = csrf_token().unwrap_or_default();

        // Build shared props with CSRF included
        let effective_shared = if let Some(existing) = shared {
            // Clone and add CSRF if not already set
            let mut shared_clone = existing.clone();
            if shared_clone.csrf.is_none() {
                shared_clone.csrf = Some(csrf.clone());
            }
            Some(shared_clone)
        } else {
            Some(InertiaShared::new().csrf(csrf.clone()))
        };

        // Use inertia-rs for the core rendering logic
        let http_response = inertia_rs::Inertia::render_with_options(
            req,
            component,
            props,
            effective_shared.as_ref(),
            config,
        );

        // Convert InertiaHttpResponse to Cancer's Response
        Ok(Self::convert_response(http_response))
    }

    /// Convert an InertiaHttpResponse to Cancer's HttpResponse.
    fn convert_response(inertia_response: inertia_rs::InertiaHttpResponse) -> HttpResponse {
        let mut response = match inertia_response.content_type {
            "application/json" => HttpResponse::text(inertia_response.body),
            "text/html; charset=utf-8" => HttpResponse::text(inertia_response.body),
            _ => HttpResponse::text(inertia_response.body),
        };

        response = response.status(inertia_response.status);
        response = response.header("Content-Type", inertia_response.content_type);

        for (name, value) in inertia_response.headers {
            response = response.header(name, value);
        }

        response
    }

    /// Check if the current request is an Inertia XHR request.
    pub fn is_inertia_request(req: &Request) -> bool {
        req.is_inertia()
    }

    /// Get the current URL from the request.
    pub fn current_url(req: &Request) -> String {
        req.path().to_string()
    }

    /// Check for version mismatch and return 409 Conflict if needed.
    ///
    /// Call this in middleware to handle asset version changes.
    pub fn check_version(
        req: &Request,
        current_version: &str,
        redirect_url: &str,
    ) -> Option<Response> {
        inertia_rs::Inertia::check_version(req, current_version, redirect_url)
            .map(|http_response| Ok(Self::convert_response(http_response)))
    }
}

// Keep deprecated InertiaContext for backward compatibility during migration
#[deprecated(
    since = "0.2.0",
    note = "Use Inertia::render() instead - thread-local storage is async-unsafe"
)]
pub struct InertiaContext;

#[allow(deprecated)]
impl InertiaContext {
    #[deprecated(note = "Use Inertia::render() instead")]
    pub fn set(_ctx: InertiaContextData) {
        // No-op - kept for compilation compatibility during migration
    }

    #[deprecated(note = "Use Inertia::is_inertia_request(&req) instead")]
    pub fn is_inertia_request() -> bool {
        false
    }

    #[deprecated(note = "Use req.path() instead")]
    pub fn current_path() -> String {
        String::new()
    }

    #[deprecated(note = "No longer needed")]
    pub fn clear() {
        // No-op
    }

    #[deprecated(note = "Use req methods instead")]
    pub fn get() -> Option<InertiaContextData> {
        None
    }
}

/// Legacy context data - kept for migration compatibility.
#[deprecated(since = "0.2.0", note = "Use Request methods instead")]
#[derive(Clone, Default)]
pub struct InertiaContextData {
    pub path: String,
    pub is_inertia: bool,
    pub version: Option<String>,
}
