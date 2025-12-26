use super::config::InertiaConfig;
use crate::csrf::csrf_token;
use crate::http::HttpResponse;

/// Builds Inertia responses based on request type
pub struct InertiaResponse {
    component: String,
    props: serde_json::Value,
    url: String,
    config: InertiaConfig,
}

impl InertiaResponse {
    pub fn new(component: impl Into<String>, props: serde_json::Value, url: String) -> Self {
        Self {
            component: component.into(),
            props,
            url,
            config: InertiaConfig::default(),
        }
    }

    pub fn with_config(mut self, config: InertiaConfig) -> Self {
        self.config = config;
        self
    }

    /// Build JSON response for XHR requests (X-Inertia: true)
    pub fn to_json_response(&self) -> HttpResponse {
        let page = serde_json::json!({
            "component": self.component,
            "props": self.props,
            "url": self.url,
            "version": self.config.version,
        });

        HttpResponse::json(page)
            .header("X-Inertia", "true")
            .header("Vary", "X-Inertia")
    }

    /// Build HTML response for initial page loads
    pub fn to_html_response(&self) -> HttpResponse {
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

        // Get CSRF token for meta tag (empty string if no session)
        let csrf = csrf_token().unwrap_or_default();

        let html = if self.config.development {
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="csrf-token" content="{}">
    <title>Kit App</title>
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
    <title>Kit App</title>
    <script type="module" src="/assets/main.js"></script>
    <link rel="stylesheet" href="/assets/main.css">
</head>
<body>
    <div id="app" data-page="{}"></div>
</body>
</html>"#,
                csrf,
                page_json
            )
        };

        HttpResponse::text(html)
            .header("Content-Type", "text/html; charset=utf-8")
            .header("Vary", "X-Inertia")
    }

    /// Build 409 Conflict response for version mismatch
    pub fn version_conflict(new_url: &str) -> HttpResponse {
        HttpResponse::text("")
            .status(409)
            .header("X-Inertia-Location", new_url)
    }
}
