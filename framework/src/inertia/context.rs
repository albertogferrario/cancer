//! Inertia.js integration - async-safe implementation
//!
//! This module provides the main `Inertia` struct for rendering Inertia responses.
//! Unlike the previous thread-local implementation, this is safe for async Rust.

use super::config::InertiaConfig;
use super::response::InertiaResponse;
use crate::http::{HttpResponse, Request};
use crate::Response;
use serde::Serialize;

/// Shared props that are merged into every Inertia response
///
/// Set via middleware and automatically included in all responses.
#[derive(Clone, Default, Serialize)]
pub struct InertiaShared {
    /// Authenticated user data (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<serde_json::Value>,
    /// Flash messages from the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash: Option<serde_json::Value>,
    /// CSRF token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf: Option<String>,
}

impl InertiaShared {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn auth(mut self, auth: impl Serialize) -> Self {
        self.auth = Some(serde_json::to_value(auth).unwrap_or_default());
        self
    }

    pub fn flash(mut self, flash: impl Serialize) -> Self {
        self.flash = Some(serde_json::to_value(flash).unwrap_or_default());
        self
    }

    pub fn csrf(mut self, token: impl Into<String>) -> Self {
        self.csrf = Some(token.into());
        self
    }
}

/// Main Inertia integration struct
///
/// Provides methods for rendering Inertia responses in an async-safe manner.
/// All state is derived from the Request, not thread-local storage.
pub struct Inertia;

impl Inertia {
    /// Render an Inertia response
    ///
    /// This is the primary method for returning Inertia responses from controllers.
    /// It automatically:
    /// - Detects XHR vs initial page load
    /// - Merges shared props from middleware
    /// - Filters props for partial reloads
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

    /// Render an Inertia response with custom configuration
    pub fn render_with_config<P: Serialize>(
        req: &Request,
        component: &str,
        props: P,
        config: InertiaConfig,
    ) -> Response {
        let url = req.path().to_string();
        let is_inertia = req.is_inertia();
        let partial_data = req.inertia_partial_data();
        let partial_component = req.inertia_partial_component();

        // Serialize props
        let mut props_value = serde_json::to_value(&props)
            .map_err(|e| HttpResponse::text(format!("Failed to serialize props: {}", e)).status(500))?;

        // Merge shared props from middleware (if set)
        if let Some(shared) = req.get::<InertiaShared>() {
            if let serde_json::Value::Object(ref mut map) = props_value {
                if let Some(auth) = &shared.auth {
                    map.insert("auth".to_string(), auth.clone());
                }
                if let Some(flash) = &shared.flash {
                    map.insert("flash".to_string(), flash.clone());
                }
                if let Some(csrf) = &shared.csrf {
                    map.insert("csrf".to_string(), serde_json::Value::String(csrf.clone()));
                }
            }
        }

        // Filter props for partial reloads
        if is_inertia {
            if let Some(partial_keys) = partial_data {
                // Only filter if this is the same component
                let should_filter = partial_component
                    .map(|pc| pc == component)
                    .unwrap_or(false);

                if should_filter {
                    props_value = Self::filter_partial_props(props_value, &partial_keys);
                }
            }
        }

        let response = InertiaResponse::new(component, props_value, url)
            .with_config(config);

        if is_inertia {
            Ok(response.to_json_response())
        } else {
            Ok(response.to_html_response())
        }
    }

    /// Filter props to only include those requested in partial reload
    fn filter_partial_props(
        props: serde_json::Value,
        partial_keys: &[&str],
    ) -> serde_json::Value {
        match props {
            serde_json::Value::Object(map) => {
                let filtered: serde_json::Map<String, serde_json::Value> = map
                    .into_iter()
                    .filter(|(k, _)| partial_keys.contains(&k.as_str()))
                    .collect();
                serde_json::Value::Object(filtered)
            }
            other => other,
        }
    }

    /// Check if the current request is an Inertia XHR request
    pub fn is_inertia_request(req: &Request) -> bool {
        req.is_inertia()
    }

    /// Get the current URL from the request
    pub fn current_url(req: &Request) -> String {
        req.path().to_string()
    }
}

// Keep deprecated InertiaContext for backward compatibility during migration
#[deprecated(since = "0.2.0", note = "Use Inertia::render() instead - thread-local storage is async-unsafe")]
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

/// Legacy context data - kept for migration compatibility
#[deprecated(since = "0.2.0", note = "Use Request methods instead")]
#[derive(Clone, Default)]
pub struct InertiaContextData {
    pub path: String,
    pub is_inertia: bool,
    pub version: Option<String>,
}
