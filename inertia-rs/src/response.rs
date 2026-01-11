//! Inertia response generation.

use crate::config::InertiaConfig;
use crate::request::InertiaRequest;
use crate::shared::InertiaShared;
use serde::Serialize;

/// Framework-agnostic HTTP response.
///
/// Convert this to your framework's response type.
#[derive(Debug, Clone)]
pub struct InertiaHttpResponse {
    /// HTTP status code
    pub status: u16,
    /// Response headers as (name, value) pairs
    pub headers: Vec<(String, String)>,
    /// Response body
    pub body: String,
    /// Content type
    pub content_type: &'static str,
}

impl InertiaHttpResponse {
    /// Create a JSON response.
    pub fn json(body: impl Into<String>) -> Self {
        Self {
            status: 200,
            headers: vec![
                ("X-Inertia".to_string(), "true".to_string()),
                ("Vary".to_string(), "X-Inertia".to_string()),
            ],
            body: body.into(),
            content_type: "application/json",
        }
    }

    /// Create an HTML response.
    pub fn html(body: impl Into<String>) -> Self {
        Self {
            status: 200,
            headers: vec![("Vary".to_string(), "X-Inertia".to_string())],
            body: body.into(),
            content_type: "text/html; charset=utf-8",
        }
    }

    /// Create a 409 Conflict response for version mismatch.
    pub fn conflict(location: impl Into<String>) -> Self {
        Self {
            status: 409,
            headers: vec![("X-Inertia-Location".to_string(), location.into())],
            body: String::new(),
            content_type: "text/plain",
        }
    }

    /// Set the HTTP status code.
    pub fn status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }

    /// Add a header to the response.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}

/// Main Inertia integration struct.
///
/// Provides methods for rendering Inertia responses in a framework-agnostic way.
pub struct Inertia;

impl Inertia {
    /// Render an Inertia response.
    ///
    /// This is the primary method for returning Inertia responses from handlers.
    /// It automatically:
    /// - Detects XHR vs initial page load
    /// - Filters props for partial reloads
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use inertia_rs::Inertia;
    /// use serde_json::json;
    ///
    /// let response = Inertia::render(&req, "Home", json!({
    ///     "title": "Welcome",
    ///     "user": { "name": "John" }
    /// }));
    /// ```
    pub fn render<R, P>(req: &R, component: &str, props: P) -> InertiaHttpResponse
    where
        R: InertiaRequest,
        P: Serialize,
    {
        Self::render_with_options(req, component, props, None, InertiaConfig::default())
    }

    /// Render an Inertia response with shared props.
    pub fn render_with_shared<R, P>(
        req: &R,
        component: &str,
        props: P,
        shared: &InertiaShared,
    ) -> InertiaHttpResponse
    where
        R: InertiaRequest,
        P: Serialize,
    {
        Self::render_with_options(
            req,
            component,
            props,
            Some(shared),
            InertiaConfig::default(),
        )
    }

    /// Render an Inertia response with custom configuration.
    pub fn render_with_config<R, P>(
        req: &R,
        component: &str,
        props: P,
        config: InertiaConfig,
    ) -> InertiaHttpResponse
    where
        R: InertiaRequest,
        P: Serialize,
    {
        Self::render_with_options(req, component, props, None, config)
    }

    /// Render an Inertia response with all options.
    pub fn render_with_options<R, P>(
        req: &R,
        component: &str,
        props: P,
        shared: Option<&InertiaShared>,
        config: InertiaConfig,
    ) -> InertiaHttpResponse
    where
        R: InertiaRequest,
        P: Serialize,
    {
        let url = req.path().to_string();
        let is_inertia = req.is_inertia();
        let partial_data = req.inertia_partial_data();
        let partial_component = req.inertia_partial_component();

        // Serialize props
        let mut props_value = match serde_json::to_value(&props) {
            Ok(v) => v,
            Err(e) => {
                return InertiaHttpResponse::html(format!("Failed to serialize props: {}", e))
                    .status(500);
            }
        };

        // Merge shared props
        if let Some(shared) = shared {
            shared.merge_into(&mut props_value);
        }

        // Filter props for partial reloads
        if is_inertia {
            if let Some(partial_keys) = partial_data {
                let should_filter = partial_component.map(|pc| pc == component).unwrap_or(false);

                if should_filter {
                    props_value = Self::filter_partial_props(props_value, &partial_keys);
                }
            }
        }

        let response = InertiaResponse::new(component, props_value, url).with_config(config);

        if is_inertia {
            response.to_json_response()
        } else {
            response.to_html_response(None)
        }
    }

    /// Check if a version conflict should trigger a full reload.
    ///
    /// Returns `Some(response)` with a 409 Conflict if versions don't match.
    pub fn check_version<R: InertiaRequest>(
        req: &R,
        current_version: &str,
        redirect_url: &str,
    ) -> Option<InertiaHttpResponse> {
        if !req.is_inertia() {
            return None;
        }

        if let Some(client_version) = req.inertia_version() {
            if client_version != current_version {
                return Some(InertiaHttpResponse::conflict(redirect_url));
            }
        }

        None
    }

    /// Filter props to only include those requested in partial reload.
    fn filter_partial_props(props: serde_json::Value, partial_keys: &[&str]) -> serde_json::Value {
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
}

/// Internal response builder.
pub struct InertiaResponse {
    component: String,
    props: serde_json::Value,
    url: String,
    config: InertiaConfig,
}

impl InertiaResponse {
    /// Create a new Inertia response.
    pub fn new(component: impl Into<String>, props: serde_json::Value, url: String) -> Self {
        Self {
            component: component.into(),
            props,
            url,
            config: InertiaConfig::default(),
        }
    }

    /// Set the configuration.
    pub fn with_config(mut self, config: InertiaConfig) -> Self {
        self.config = config;
        self
    }

    /// Build JSON response for XHR requests.
    pub fn to_json_response(&self) -> InertiaHttpResponse {
        let page = serde_json::json!({
            "component": self.component,
            "props": self.props,
            "url": self.url,
            "version": self.config.version,
        });

        InertiaHttpResponse::json(serde_json::to_string(&page).unwrap_or_default())
    }

    /// Build HTML response for initial page loads.
    pub fn to_html_response(&self, csrf_token: Option<&str>) -> InertiaHttpResponse {
        let page_data = serde_json::json!({
            "component": self.component,
            "props": self.props,
            "url": self.url,
            "version": self.config.version,
        });

        // Escape JSON for HTML attribute
        let page_json = serde_json::to_string(&page_data)
            .unwrap_or_default()
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;");

        let csrf = csrf_token.unwrap_or("");

        // Use custom template if provided
        if let Some(template) = &self.config.html_template {
            let html = template
                .replace("{page}", &page_json)
                .replace("{csrf}", csrf);
            return InertiaHttpResponse::html(html);
        }

        // Default template
        let html = if self.config.development {
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="csrf-token" content="{}">
    <title>Inertia App</title>
    <script type="module">
        import RefreshRuntime from '{}/@react-refresh'
        RefreshRuntime.injectIntoGlobalHook(window)
        window.$RefreshReg$ = () => {{}}
        window.$RefreshSig$ = () => (type) => type
        window.__vite_plugin_react_preamble_installed__ = true
    </script>
    <script type="module" src="{}/@vite/client"></script>
    <script type="module" src="{}/{}"></script>
</head>
<body>
    <div id="app" data-page="{}"></div>
</body>
</html>"#,
                csrf,
                self.config.vite_dev_server,
                self.config.vite_dev_server,
                self.config.vite_dev_server,
                self.config.entry_point,
                page_json
            )
        } else {
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="csrf-token" content="{}">
    <title>Inertia App</title>
    <script type="module" src="/assets/main.js"></script>
    <link rel="stylesheet" href="/assets/main.css">
</head>
<body>
    <div id="app" data-page="{}"></div>
</body>
</html>"#,
                csrf, page_json
            )
        };

        InertiaHttpResponse::html(html)
    }
}
