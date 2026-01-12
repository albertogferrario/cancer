# CLI Reference

Cancer provides a powerful CLI tool for project scaffolding, code generation, database management, and development workflow automation.

## Installation

```bash
cargo install cancer-cli
```

Or build from source:

```bash
git clone https://github.com/albertogferrario/cancer
cd cancer/cancer-cli
cargo install --path .
```

## Project Commands

### `cancer new`

Create a new Cancer project with the complete directory structure.

```bash
# Interactive mode (prompts for project name)
cancer new

# Direct creation
cancer new my-app

# Skip git initialization
cancer new my-app --no-git

# Non-interactive mode (uses defaults)
cancer new my-app --no-interaction
```

**Options:**

| Option | Description |
|--------|-------------|
| `--no-interaction` | Skip prompts, use defaults |
| `--no-git` | Don't initialize git repository |

**Generated Structure:**

```
my-app/
├── src/
│   ├── main.rs
│   ├── bootstrap.rs
│   ├── routes.rs
│   ├── controllers/
│   │   └── mod.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── cors.rs
│   ├── models/
│   │   └── mod.rs
│   ├── migrations/
│   │   └── mod.rs
│   ├── events/
│   │   └── mod.rs
│   ├── listeners/
│   │   └── mod.rs
│   ├── jobs/
│   │   └── mod.rs
│   ├── notifications/
│   │   └── mod.rs
│   ├── tasks/
│   │   └── mod.rs
│   ├── seeders/
│   │   └── mod.rs
│   └── factories/
│       └── mod.rs
├── frontend/
│   ├── src/
│   │   ├── pages/
│   │   │   └── Home.tsx
│   │   ├── layouts/
│   │   │   └── Layout.tsx
│   │   ├── types/
│   │   │   └── inertia.d.ts
│   │   ├── app.tsx
│   │   └── main.tsx
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   └── tailwind.config.js
├── Cargo.toml
├── .env
├── .env.example
└── .gitignore
```

## Development Commands

### `cancer serve`

Start the development server with hot reloading for both backend and frontend.

```bash
# Start both backend and frontend
cancer serve

# Custom ports
cancer serve --port 8080 --frontend-port 5173

# Backend only (no frontend dev server)
cancer serve --backend-only

# Frontend only (no Rust compilation)
cancer serve --frontend-only

# Skip TypeScript type generation
cancer serve --skip-types
```

**Options:**

| Option | Default | Description |
|--------|---------|-------------|
| `--port` | `3000` | Backend server port |
| `--frontend-port` | `5173` | Frontend dev server port |
| `--backend-only` | `false` | Run only the backend |
| `--frontend-only` | `false` | Run only the frontend |
| `--skip-types` | `false` | Don't regenerate TypeScript types |

**What it does:**

1. Starts the Rust backend with `cargo watch` for hot reloading
2. Starts the Vite frontend dev server
3. Watches Rust files to regenerate TypeScript types automatically
4. Proxies frontend requests to the backend

### `cancer generate-types`

Generate TypeScript type definitions from Rust `InertiaProps` structs.

```bash
cancer generate-types
```

This scans your Rust code for structs deriving `InertiaProps` and generates corresponding TypeScript interfaces in `frontend/src/types/`.

## Code Generators

All generators follow the pattern `cancer make:<type> <name> [options]`.

### `cancer make:controller`

Generate a controller with handler methods.

```bash
# Basic controller
cancer make:controller UserController

# Resource controller with CRUD methods
cancer make:controller PostController --resource

# API controller (JSON responses)
cancer make:controller Api/ProductController --api
```

**Options:**

| Option | Description |
|--------|-------------|
| `--resource` | Generate index, show, create, store, edit, update, destroy methods |
| `--api` | Generate API-style controller (JSON responses) |

**Generated file:** `src/controllers/user_controller.rs`

```rust
use cancer::{handler, Request, Response, json_response};

#[handler]
pub async fn index(req: Request) -> Response {
    // TODO: Implement
    json_response!({ "message": "index" })
}

#[handler]
pub async fn show(req: Request) -> Response {
    // TODO: Implement
    json_response!({ "message": "show" })
}

// ... additional methods for --resource
```

### `cancer make:middleware`

Generate middleware for request/response processing.

```bash
cancer make:middleware Auth
cancer make:middleware RateLimit
```

**Generated file:** `src/middleware/auth.rs`

```rust
use cancer::{Middleware, Request, Response, Next};
use async_trait::async_trait;

pub struct Auth;

#[async_trait]
impl Middleware for Auth {
    async fn handle(&self, request: Request, next: Next) -> Response {
        // TODO: Implement middleware logic
        next.run(request).await
    }
}
```

### `cancer make:action`

Generate a single-action class for complex business logic.

```bash
cancer make:action CreateOrder
cancer make:action ProcessPayment
```

**Generated file:** `src/actions/create_order.rs`

```rust
use cancer::FrameworkError;

pub struct CreateOrder;

impl CreateOrder {
    pub async fn execute(&self) -> Result<(), FrameworkError> {
        // TODO: Implement action
        Ok(())
    }
}
```

### `cancer make:event`

Generate an event struct for the event dispatcher.

```bash
cancer make:event UserRegistered
cancer make:event OrderPlaced
```

**Generated file:** `src/events/user_registered.rs`

```rust
use cancer_events::Event;

#[derive(Debug, Clone, Event)]
pub struct UserRegistered {
    pub user_id: i64,
}
```

### `cancer make:listener`

Generate a listener that responds to events.

```bash
cancer make:listener SendWelcomeEmail
cancer make:listener NotifyAdmins
```

**Generated file:** `src/listeners/send_welcome_email.rs`

```rust
use cancer_events::{Listener, Event};
use async_trait::async_trait;

pub struct SendWelcomeEmail;

#[async_trait]
impl<E: Event + Send + Sync> Listener<E> for SendWelcomeEmail {
    async fn handle(&self, event: &E) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement listener
        Ok(())
    }
}
```

### `cancer make:job`

Generate a background job for queue processing.

```bash
cancer make:job ProcessImage
cancer make:job SendEmail
```

**Generated file:** `src/jobs/process_image.rs`

```rust
use cancer_queue::{Job, JobContext};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessImage {
    pub image_id: i64,
}

#[async_trait]
impl Job for ProcessImage {
    async fn handle(&self, ctx: &JobContext) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement job
        Ok(())
    }
}
```

### `cancer make:notification`

Generate a multi-channel notification.

```bash
cancer make:notification OrderShipped
cancer make:notification InvoiceGenerated
```

**Generated file:** `src/notifications/order_shipped.rs`

```rust
use cancer_notifications::{Notification, Notifiable, Channel};

pub struct OrderShipped {
    pub order_id: i64,
}

impl Notification for OrderShipped {
    fn via(&self) -> Vec<Channel> {
        vec![Channel::Mail, Channel::Database]
    }
}
```

### `cancer make:migration`

Generate a database migration file.

```bash
cancer make:migration create_posts_table
cancer make:migration add_status_to_orders
```

**Generated file:** `src/migrations/m20240115_143052_create_posts_table.rs`

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Implement migration
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // TODO: Implement rollback
        Ok(())
    }
}
```

### `cancer make:inertia`

Generate an Inertia.js page component with TypeScript types.

```bash
cancer make:inertia Dashboard
cancer make:inertia Users/Profile
```

**Generated files:**
- `frontend/src/pages/Dashboard.tsx`
- `src/controllers/` (props struct)

### `cancer make:task`

Generate a scheduled task.

```bash
cancer make:task CleanupExpiredSessions
cancer make:task SendDailyReport
```

**Generated file:** `src/tasks/cleanup_expired_sessions.rs`

```rust
use cancer::scheduling::{Task, Schedule};
use async_trait::async_trait;

pub struct CleanupExpiredSessions;

#[async_trait]
impl Task for CleanupExpiredSessions {
    fn schedule(&self) -> Schedule {
        Schedule::daily()
    }

    async fn handle(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Implement task
        Ok(())
    }
}
```

### `cancer make:seeder`

Generate a database seeder.

```bash
cancer make:seeder UserSeeder
cancer make:seeder ProductSeeder
```

**Generated file:** `src/seeders/user_seeder.rs`

```rust
use cancer::database::Seeder;
use async_trait::async_trait;

pub struct UserSeeder;

#[async_trait]
impl Seeder for UserSeeder {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: Seed data
        Ok(())
    }
}
```

### `cancer make:factory`

Generate a model factory for testing.

```bash
cancer make:factory UserFactory
cancer make:factory PostFactory
```

**Generated file:** `src/factories/user_factory.rs`

```rust
use cancer::testing::Factory;
use fake::{Fake, Faker};

pub struct UserFactory;

impl Factory for UserFactory {
    type Model = user::Model;

    fn definition(&self) -> Self::Model {
        // TODO: Define factory
        todo!()
    }
}
```

### `cancer make:error`

Generate a custom error type.

```bash
cancer make:error PaymentFailed
cancer make:error ValidationError
```

### `cancer make:scaffold`

Generate a complete CRUD scaffold: model, migration, controller, and Inertia pages.

```bash
# Basic scaffold
cancer make:scaffold Post

# With field definitions
cancer make:scaffold Post title:string content:text published:bool

# Complex example
cancer make:scaffold Product name:string description:text price:float stock:integer
```

**Field Types:**

| Type | Rust Type | Database Type |
|------|-----------|---------------|
| `string` | `String` | `VARCHAR(255)` |
| `text` | `String` | `TEXT` |
| `integer` | `i32` | `INTEGER` |
| `bigint` | `i64` | `BIGINT` |
| `float` | `f64` | `DOUBLE` |
| `bool` | `bool` | `BOOLEAN` |
| `datetime` | `DateTime` | `TIMESTAMP` |
| `date` | `Date` | `DATE` |
| `uuid` | `Uuid` | `UUID` |

**Generated Files:**

```
src/
├── models/post.rs           # SeaORM entity
├── migrations/m*_create_posts_table.rs
├── controllers/post_controller.rs
frontend/src/pages/
├── posts/
│   ├── Index.tsx
│   ├── Show.tsx
│   ├── Create.tsx
│   └── Edit.tsx
```

## Database Commands

### `cancer migrate`

Run all pending migrations.

```bash
cancer migrate
```

### `cancer migrate:rollback`

Rollback the last batch of migrations.

```bash
cancer migrate:rollback
```

### `cancer migrate:status`

Show the status of all migrations.

```bash
cancer migrate:status
```

Output:

```
+------+------------------------------------------------+-------+
| Ran? | Migration                                       | Batch |
+------+------------------------------------------------+-------+
| Yes  | m20240101_000001_create_users_table            | 1     |
| Yes  | m20240101_000002_create_posts_table            | 1     |
| No   | m20240115_143052_add_status_to_posts           |       |
+------+------------------------------------------------+-------+
```

### `cancer migrate:fresh`

Drop all tables and re-run all migrations.

```bash
cancer migrate:fresh
```

**Warning:** This is destructive and will delete all data.

### `cancer db:sync`

Synchronize the database schema and generate entity files.

```bash
# Sync entities from existing database
cancer db:sync

# Run migrations first, then sync
cancer db:sync --migrate
```

**Options:**

| Option | Description |
|--------|-------------|
| `--migrate` | Run pending migrations before syncing |

This command:
1. Discovers the database schema (tables, columns, types)
2. Generates SeaORM entity files in `src/models/entities/`
3. Creates user-friendly model wrappers with the Cancer Model API

### `cancer db:query`

Execute a raw SQL query against the database.

```bash
# Simple SELECT query
cancer db:query "SELECT * FROM users LIMIT 5"

# Query with conditions
cancer db:query "SELECT id, name, email FROM users WHERE active = true"

# Count query
cancer db:query "SELECT COUNT(*) FROM posts"
```

**Features:**

- Reads `DATABASE_URL` from `.env` file
- Supports SQLite and PostgreSQL databases
- Displays results in a formatted table
- Handles NULL values gracefully
- Shows row count after results

**Example Output:**

```
+-----+-------+-------------------+
| 1   | Alice | alice@example.com |
| 2   | Bob   | bob@example.com   |
+-----+-------+-------------------+

→ 2 row(s)
```

**Use Cases:**

- Quick data inspection during development
- Debugging database state
- Verifying migration results
- Ad-hoc queries without external tools

## Docker Commands

### `cancer docker:init`

Initialize Docker configuration files.

```bash
cancer docker:init
```

**Generated files:**
- `Dockerfile`
- `docker-compose.yml`
- `.dockerignore`

### `cancer docker:compose`

Manage Docker Compose services.

```bash
# Start services
cancer docker:compose up

# Stop services
cancer docker:compose down

# Rebuild and start
cancer docker:compose up --build
```

## Scheduling Commands

### `cancer schedule:run`

Run scheduled tasks that are due.

```bash
cancer schedule:run
```

This executes all tasks whose schedule indicates they should run now. Typically called by a system cron job every minute:

```cron
* * * * * cd /path/to/project && cancer schedule:run >> /dev/null 2>&1
```

### `cancer schedule:work`

Start the scheduler worker for continuous task execution.

```bash
cancer schedule:work
```

This runs in the foreground and checks for due tasks every minute. Useful for development or container deployments.

### `cancer schedule:list`

Display all registered scheduled tasks.

```bash
cancer schedule:list
```

Output:

```
+---------------------------+-------------+-------------------+
| Task                      | Schedule    | Next Run          |
+---------------------------+-------------+-------------------+
| CleanupExpiredSessions    | Daily 00:00 | 2024-01-16 00:00  |
| SendDailyReport           | Daily 09:00 | 2024-01-15 09:00  |
| PruneOldNotifications     | Weekly Mon  | 2024-01-22 00:00  |
+---------------------------+-------------+-------------------+
```

## Storage Commands

### `cancer storage:link`

Create a symbolic link from `public/storage` to `storage/app/public`.

```bash
cancer storage:link
```

This allows publicly accessible files stored in `storage/app/public` to be served via the web server.

## AI-Assisted Development

### `cancer mcp`

Start the Model Context Protocol (MCP) server for AI-assisted development.

```bash
cancer mcp
```

The MCP server provides introspection tools that help AI assistants understand your Cancer application structure, including routes, models, controllers, and configuration.

### `cancer boost:install`

Install AI development boost features.

```bash
cancer boost:install
```

This sets up configuration for enhanced AI-assisted development workflows.

## Command Summary

| Command | Description |
|---------|-------------|
| `new` | Create a new Cancer project |
| `serve` | Start development server |
| `generate-types` | Generate TypeScript types |
| `make:controller` | Create a controller |
| `make:middleware` | Create middleware |
| `make:action` | Create an action class |
| `make:event` | Create an event |
| `make:listener` | Create a listener |
| `make:job` | Create a background job |
| `make:notification` | Create a notification |
| `make:migration` | Create a migration |
| `make:inertia` | Create an Inertia page |
| `make:task` | Create a scheduled task |
| `make:seeder` | Create a database seeder |
| `make:factory` | Create a model factory |
| `make:error` | Create a custom error |
| `make:policy` | Create an authorization policy |
| `make:scaffold` | Create complete CRUD scaffold |
| `migrate` | Run migrations |
| `migrate:rollback` | Rollback migrations |
| `migrate:status` | Show migration status |
| `migrate:fresh` | Fresh migrate (drop all) |
| `db:sync` | Sync database schema |
| `db:query` | Execute raw SQL query |
| `docker:init` | Initialize Docker files |
| `docker:compose` | Manage Docker Compose |
| `schedule:run` | Run due scheduled tasks |
| `schedule:work` | Start scheduler worker |
| `schedule:list` | List scheduled tasks |
| `storage:link` | Create storage symlink |
| `mcp` | Start MCP server |
| `boost:install` | Install AI boost features |

## Environment Variables

The CLI respects these environment variables:

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | Database connection string |
| `APP_ENV` | Application environment (development, production) |
| `RUST_LOG` | Logging level |
