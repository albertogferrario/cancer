# Cancer Framework

A Laravel-inspired web framework for Rust.

Cancer brings the developer experience of Laravel to Rust, providing familiar patterns and conventions while leveraging Rust's safety and performance.

## Features

- **Routing** - Expressive route definitions with middleware support
- **Database** - SeaORM integration with migrations and models
- **Validation** - Laravel-style validation with declarative rules
- **Authentication** - Session-based auth with guards
- **Inertia.js** - Full-stack React/TypeScript with compile-time validation
- **Events** - Event dispatcher with sync/async listeners
- **Queues** - Background job processing with Redis
- **Notifications** - Multi-channel notifications (mail, database, slack)
- **Broadcasting** - WebSocket channels with authorization
- **Storage** - File storage abstraction (local, S3)
- **Caching** - Cache with tags support
- **Testing** - Test utilities and factories

## Quick Example

```rust
use cancer_rs::*;

#[handler]
pub async fn index(req: Request) -> Response {
    let users = User::find().all(&db).await?;

    Inertia::render(&req, "Users/Index", UsersProps { users })
}

pub fn routes() -> Router {
    Router::new()
        .get("/users", index)
        .middleware(AuthMiddleware)
}
```

## Philosophy

Cancer aims to be the "Laravel of Rust" - a batteries-included framework that lets you build web applications quickly without sacrificing Rust's guarantees.

**Convention over configuration** - Sensible defaults that work out of the box.

**Developer experience** - Clear error messages, helpful CLI, and comprehensive documentation.

**Type safety** - Compile-time validation of routes, components, and queries.

**Performance** - Async-first design built on Tokio.

## Getting Started

Ready to start building? Head to the [Installation](getting-started/installation.md) guide.
