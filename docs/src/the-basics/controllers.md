# Controllers

Controllers group related request handling logic.

## Creating Controllers

```bash
cancer make:controller Users
```

This creates `src/controllers/users_controller.rs`:

```rust
use cancer_rs::*;

#[handler]
pub async fn index(req: Request) -> Response {
    // List users
    Ok(json!({"users": []}))
}

#[handler]
pub async fn show(req: Request, id: i64) -> Response {
    // Show single user
    Ok(json!({"id": id}))
}

#[handler]
pub async fn store(req: Request) -> Response {
    // Create user
    Ok(json!({"created": true}))
}

#[handler]
pub async fn update(req: Request, id: i64) -> Response {
    // Update user
    Ok(json!({"updated": id}))
}

#[handler]
pub async fn destroy(req: Request, id: i64) -> Response {
    // Delete user
    Ok(json!({"deleted": id}))
}
```

## The Handler Macro

The `#[handler]` macro provides:

1. **Automatic parameter extraction** from path, query, body
2. **Dependency injection** for services
3. **Error handling** conversion

```rust
#[handler]
pub async fn show(
    req: Request,           // Always available
    id: i64,                // From path parameter
    user_service: Arc<dyn UserService>,  // Injected service
) -> Response {
    let user = user_service.find(id).await?;
    Ok(json!(user))
}
```

## Route Registration

Register controller methods in `src/routes.rs`:

```rust
use crate::controllers::users_controller;

pub fn routes() -> Router {
    Router::new()
        .get("/users", users_controller::index)
        .get("/users/:id", users_controller::show)
        .post("/users", users_controller::store)
        .put("/users/:id", users_controller::update)
        .delete("/users/:id", users_controller::destroy)
}
```

## Inertia Controllers

For Inertia.js responses:

```rust
use cancer_rs::*;
use crate::models::users::Entity as User;

#[handler]
pub async fn index(req: Request) -> Response {
    let db = req.db();
    let users = User::find().all(db).await?;

    Inertia::render(&req, "Users/Index", UsersIndexProps { users })
}

#[derive(InertiaProps)]
pub struct UsersIndexProps {
    pub users: Vec<crate::models::users::Model>,
}
```

## Form Validation

Use form requests for validation:

```rust
use cancer_rs::*;

#[derive(FormRequest)]
pub struct CreateUserRequest {
    #[validate(required, email)]
    pub email: String,

    #[validate(required, min(8))]
    pub password: String,
}

#[handler]
pub async fn store(req: Request, form: CreateUserRequest) -> Response {
    // form is already validated
    let user = User::create(form.email, form.password).await?;
    Ok(Redirect::to("/users"))
}
```

## Service Injection

Inject services via the `#[service]` system:

```rust
#[handler]
pub async fn store(
    req: Request,
    form: CreateUserRequest,
    user_service: Arc<dyn UserService>,
    mailer: Arc<dyn Mailer>,
) -> Response {
    let user = user_service.create(form).await?;
    mailer.send_welcome(user.email).await?;

    Ok(Redirect::to("/users"))
}
```

## Actions

For complex operations, use Actions:

```bash
cancer make:action CreateUser
```

```rust
// src/actions/create_user.rs
use cancer_rs::*;

#[derive(Action)]
pub struct CreateUser {
    user_service: Arc<dyn UserService>,
    mailer: Arc<dyn Mailer>,
}

impl CreateUser {
    pub async fn execute(&self, data: CreateUserData) -> Result<User> {
        let user = self.user_service.create(data).await?;
        self.mailer.send_welcome(&user).await?;
        Ok(user)
    }
}
```

Use in controller:

```rust
#[handler]
pub async fn store(req: Request, form: CreateUserRequest, action: CreateUser) -> Response {
    let user = action.execute(form.into()).await?;
    Ok(Redirect::to(format!("/users/{}", user.id)))
}
```

## Resource Controllers

A resource controller handles all CRUD operations:

| Method | Handler | Description |
|--------|---------|-------------|
| GET /resources | index | List all |
| GET /resources/create | create | Show create form |
| POST /resources | store | Create new |
| GET /resources/:id | show | Show single |
| GET /resources/:id/edit | edit | Show edit form |
| PUT /resources/:id | update | Update |
| DELETE /resources/:id | destroy | Delete |

## API Controllers

For JSON APIs:

```rust
#[handler]
pub async fn index(req: Request) -> Response {
    let users = User::find().all(&req.db()).await?;

    Ok(json!({
        "data": users,
        "meta": {
            "total": users.len()
        }
    }))
}
```

With pagination:

```rust
#[handler]
pub async fn index(req: Request, page: Option<i64>, per_page: Option<i64>) -> Response {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(15);

    let paginator = User::find()
        .paginate(&req.db(), per_page as u64);

    let users = paginator.fetch_page(page as u64 - 1).await?;
    let total = paginator.num_items().await?;

    Ok(json!({
        "data": users,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total
        }
    }))
}
```
