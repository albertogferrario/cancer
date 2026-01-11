# Middleware

Middleware filter HTTP requests before they reach your handlers.

## Creating Middleware

```bash
cancer make:middleware Auth
```

This creates `src/middleware/auth_middleware.rs`:

```rust
use cancer_rs::*;

pub struct AuthMiddleware;

#[async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(&self, request: Request, next: Next) -> HttpResponse {
        // Check authentication
        if !request.is_authenticated() {
            return HttpResponse::redirect("/login");
        }

        // Continue to next middleware/handler
        next(request).await
    }
}
```

## Global Middleware

Register middleware that runs on every request in `src/bootstrap.rs`:

```rust
use cancer_rs::*;
use crate::middleware::*;

pub fn register() {
    global_middleware!(LoggingMiddleware);
    global_middleware!(SessionMiddleware::new(session_config()));
    global_middleware!(CsrfMiddleware);
}
```

Global middleware runs in registration order.

## Route Middleware

Apply middleware to specific routes:

```rust
Router::new()
    .get("/dashboard", dashboard)
    .middleware(AuthMiddleware)
```

Or to route groups:

```rust
Router::new()
    .group("/admin", |group| {
        group
            .get("/users", admin::users)
            .get("/settings", admin::settings)
            .middleware(AdminMiddleware)
    })
```

## Middleware Parameters

Pass configuration to middleware:

```rust
pub struct RateLimitMiddleware {
    max_requests: u32,
    window_seconds: u64,
}

impl RateLimitMiddleware {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self { max_requests, window_seconds }
    }
}
```

Use in routes:

```rust
Router::new()
    .get("/api/users", api::users)
    .middleware(RateLimitMiddleware::new(100, 60))
```

## Built-in Middleware

Cancer provides common middleware:

### AuthMiddleware

Ensures user is authenticated:

```rust
use cancer_rs::AuthMiddleware;

Router::new()
    .get("/dashboard", dashboard)
    .middleware(AuthMiddleware)
```

### GuestMiddleware

Ensures user is NOT authenticated (for login pages):

```rust
use cancer_rs::GuestMiddleware;

Router::new()
    .get("/login", login_form)
    .middleware(GuestMiddleware)
```

### SessionMiddleware

Manages session data:

```rust
use cancer_rs::{SessionMiddleware, SessionConfig};

global_middleware!(SessionMiddleware::new(SessionConfig {
    cookie_name: "session".to_string(),
    lifetime: Duration::from_secs(3600 * 24),
    ..Default::default()
}));
```

### CsrfMiddleware

CSRF protection for forms:

```rust
use cancer_rs::CsrfMiddleware;

global_middleware!(CsrfMiddleware);
```

### Throttle (Rate Limiting)

```rust
use cancer_rs::{Throttle, RateLimitConfig};

Router::new()
    .get("/api/users", api::users)
    .middleware(Throttle::new("api", RateLimitConfig {
        max_attempts: 60,
        decay_seconds: 60,
    }))
```

## Modifying Requests

Middleware can modify requests before passing them on:

```rust
#[async_trait]
impl Middleware for AddHeaderMiddleware {
    async fn handle(&self, mut request: Request, next: Next) -> HttpResponse {
        request.headers_mut().insert("X-Custom", "value");
        next(request).await
    }
}
```

## Modifying Responses

Middleware can also modify responses:

```rust
#[async_trait]
impl Middleware for CorsMiddleware {
    async fn handle(&self, request: Request, next: Next) -> HttpResponse {
        let mut response = next(request).await;

        response.headers_mut().insert(
            "Access-Control-Allow-Origin",
            "*".parse().unwrap(),
        );

        response
    }
}
```

## Terminating Middleware

Return early without calling `next()`:

```rust
#[async_trait]
impl Middleware for MaintenanceMiddleware {
    async fn handle(&self, request: Request, next: Next) -> HttpResponse {
        if self.is_maintenance_mode() {
            return HttpResponse::service_unavailable("Under maintenance");
        }

        next(request).await
    }
}
```

## Middleware Order

1. Global middleware (in registration order)
2. Route group middleware
3. Route-specific middleware
4. Handler

Responses travel back through the stack in reverse order.
