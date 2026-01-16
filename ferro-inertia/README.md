# inertia-rs

Server-side [Inertia.js](https://inertiajs.com) adapter for Rust web frameworks.

[![Crates.io](https://img.shields.io/crates/v/inertia-rs.svg)](https://crates.io/crates/inertia-rs)
[![Documentation](https://docs.rs/inertia-rs/badge.svg)](https://docs.rs/inertia-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Framework-agnostic** - Works with Axum, Actix-web, Rocket, Hyper, or any Rust web framework
- **Async-safe** - No thread-local storage, safe for async runtimes like Tokio
- **Partial reloads** - Efficient updates via `X-Inertia-Partial-Data` header
- **Shared props** - Easily share auth, flash messages, CSRF tokens across all responses
- **Version detection** - Automatic 409 Conflict responses for asset version mismatches
- **Vite integration** - Development mode with HMR support

## Installation

```toml
[dependencies]
inertia-rs = "0.1"
```

## Quick Start

### 1. Implement the `InertiaRequest` trait

```rust
use inertia_rs::InertiaRequest;

impl InertiaRequest for MyRequest {
    fn inertia_header(&self, name: &str) -> Option<&str> {
        self.headers().get(name).and_then(|v| v.to_str().ok())
    }

    fn path(&self) -> &str {
        self.uri().path()
    }
}
```

### 2. Render Inertia responses

```rust
use inertia_rs::Inertia;
use serde_json::json;

async fn index(req: MyRequest) -> MyResponse {
    let response = Inertia::render(&req, "Home", json!({
        "title": "Welcome",
        "user": {
            "name": "John Doe",
            "email": "john@example.com"
        }
    }));

    // Convert InertiaHttpResponse to your framework's response type
    response.into()
}
```

### 3. Add shared props via middleware

```rust
use inertia_rs::{Inertia, InertiaShared};

async fn handler(req: MyRequest) -> MyResponse {
    let shared = InertiaShared::new()
        .auth(get_current_user())
        .csrf(get_csrf_token())
        .flash(get_flash_messages());

    let response = Inertia::render_with_shared(&req, "Dashboard", props, &shared);
    response.into()
}
```

## Configuration

```rust
use inertia_rs::InertiaConfig;

// Development (default)
let config = InertiaConfig::new()
    .vite_dev_server("http://localhost:5173")
    .entry_point("src/main.tsx");

// Production
let config = InertiaConfig::new()
    .version("1.0.0")
    .production();

// Custom HTML template
let config = InertiaConfig::new()
    .html_template(r#"
        <!DOCTYPE html>
        <html>
        <head><title>My App</title></head>
        <body>
            <div id="app" data-page="{page}"></div>
            <script src="/app.js"></script>
        </body>
        </html>
    "#);
```

## Version Conflict Detection

```rust
// In middleware, check for version mismatch
if let Some(conflict_response) = Inertia::check_version(&req, "1.0.0", "/") {
    return conflict_response.into();
}
```

## Partial Reloads

Partial reloads are handled automatically. When the client sends:

```
X-Inertia-Partial-Data: user,notifications
X-Inertia-Partial-Component: Dashboard
```

Only the requested props (`user`, `notifications`) will be included in the response.

## Framework Examples

### Axum

```rust
use axum::{response::IntoResponse, http::StatusCode};
use inertia_rs::InertiaHttpResponse;

impl IntoResponse for InertiaHttpResponse {
    fn into_response(self) -> axum::response::Response {
        let mut response = axum::response::Response::builder()
            .status(StatusCode::from_u16(self.status).unwrap());

        for (name, value) in self.headers {
            response = response.header(name, value);
        }

        response
            .header("Content-Type", self.content_type)
            .body(self.body.into())
            .unwrap()
    }
}
```

### Actix-web

```rust
use actix_web::HttpResponse;
use inertia_rs::InertiaHttpResponse;

impl From<InertiaHttpResponse> for HttpResponse {
    fn from(res: InertiaHttpResponse) -> Self {
        let mut builder = HttpResponse::build(
            actix_web::http::StatusCode::from_u16(res.status).unwrap()
        );

        for (name, value) in res.headers {
            builder.insert_header((name, value));
        }

        builder
            .content_type(res.content_type)
            .body(res.body)
    }
}
```

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
