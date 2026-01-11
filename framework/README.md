# Cancer-RS

A Laravel-inspired web framework for Rust.

## Installation

Add Cancer to your `Cargo.toml`:

```toml
[dependencies]
cancer = { package = "cancer-rs", version = "0.1" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use cancer::{json_response, text, Router, Server, Request, Response};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .get("/", index)
        .get("/users/{id}", show_user);

    Server::new(router)
        .port(8080)
        .run()
        .await
        .expect("Failed to start server");
}

async fn index(_req: Request) -> Response {
    text("Welcome to Cancer!")
}

async fn show_user(req: Request) -> Response {
    let id = req.param("id")?;  // Returns 400 if missing
    json_response!({
        "id": id,
        "name": format!("User {}", id)
    })
}
```

## Features

- **Simple routing** - GET, POST, PUT, DELETE with route parameters
- **Async handlers** - Built on Tokio for high performance
- **Response builders** - Text, JSON, and custom responses
- **Error handling** - Use `?` operator for automatic 400 responses
- **Laravel-inspired** - Familiar patterns for Laravel developers

## CLI Tool

Use the Cancer CLI to scaffold new projects:

```bash
cargo install cancer-cli
cancer new myapp
```

## License

MIT
