# cancer-macros

Procedural macros for the Cancer framework.

## Macros

### `#[handler]`

Transform functions into HTTP handlers with automatic parameter extraction:

```rust
use cancer::{handler, Request, Response};

#[handler]
pub async fn show(req: Request, id: i64) -> Response {
    // id is extracted from path parameter
    Ok(json!({"id": id}))
}
```

### `#[request]`

Define validated request data:

```rust
use cancer::request;

#[request]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,
}
```

### `#[service]`

Mark traits for dependency injection:

```rust
use cancer::service;

#[service]
pub trait UserService: Send + Sync {
    async fn find(&self, id: i64) -> Option<User>;
}
```

### `#[injectable]`

Auto-register implementations as singletons:

```rust
use cancer::injectable;

#[injectable]
pub struct AppState {
    pub counter: u32,
}
```

### `#[domain_error]`

Define domain errors with HTTP response conversion:

```rust
use cancer::domain_error;

#[domain_error(status = 404, message = "User not found")]
pub struct UserNotFoundError {
    pub user_id: i64,
}
```

### `inertia_response!`

Create Inertia.js responses with compile-time component validation:

```rust
use cancer::inertia_response;

inertia_response!("Users/Index", UsersProps { users })
```

### `describe!` and `test!`

Jest-like testing macros:

```rust
use cancer::{describe, test, expect};

describe!("UserService", {
    test!("finds user by id", async fn(db: TestDatabase) {
        let user = UserService::find(1).await;
        expect!(user).to_be_some();
    });
});
```

## License

MIT
