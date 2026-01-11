# Cancer

**A Laravel-inspired web framework for Rust**

[![Crates.io](https://img.shields.io/crates/v/cancer-rs.svg)](https://crates.io/crates/cancer-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Build web applications in Rust with the developer experience you love from Laravel and Rails. Cancer gives you expressive routing, powerful tooling, and batteries-included features—without sacrificing Rust's performance.

[Website](https://cancer-rs.dev/) | [Documentation](https://cancer-rs.dev/)

## Quick Start

```bash
cargo install cancer-cli
cancer new myapp
cd myapp
cancer serve
```

Your app is now running at `http://localhost:8000`

## Example

If you've used Laravel or Rails, this will feel familiar:

```rust
use cancer::{get, post, routes, json_response, Request, Response};

routes! {
    get("/", index),
    get("/users/{id}", show),
    post("/users", store),
}

async fn index(_req: Request) -> Response {
    json_response!({ "message": "Welcome to Cancer!" })
}

async fn show(req: Request) -> Response {
    let id = req.param("id")?;
    json_response!({ "user": { "id": id } })
}

async fn store(_req: Request) -> Response {
    // Your logic here
    json_response!({ "created": true })
}
```

## Why Cancer?

- **Familiar patterns** — Routes, controllers, middleware, service container
- **CLI generators** — `cancer make:controller`, `cancer make:model`, `cancer migrate`
- **Database built-in** — Migrations, ORM, query builder
- **Modern frontend** — First-class Inertia.js + React with automatic TypeScript types
- **Rust performance** — All the safety and speed, none of the ceremony

## End-to-End Type Safety

Cancer provides automatic TypeScript type generation from your Rust structs. Define your props once in Rust, and use them with full type safety in React.

**Define props in Rust:**

```rust
use cancer::{InertiaProps, inertia_response, Request, Response};

#[derive(InertiaProps)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(InertiaProps)]
pub struct HomeProps {
    pub title: String,
    pub user: User,
}

pub async fn index(_req: Request) -> Response {
    inertia_response!("Home", HomeProps {
        title: "Welcome!".to_string(),
        user: User {
            name: "John".to_string(),
            email: "john@example.com".to_string(),
        },
    })
}
```

**Run type generation:**

```bash
cancer generate-types
```

**TypeScript types are auto-generated:**

```typescript
// frontend/src/types/inertia-props.ts (auto-generated)
export interface HomeProps {
  title: string;
  user: User;
}

export interface User {
  name: string;
  email: string;
}
```

**Use in your React components with full autocomplete:**

```tsx
import { HomeProps } from "../types/inertia-props";

export default function Home({ title, user }: HomeProps) {
  return (
    <div>
      <h1>{title}</h1>
      <p>Welcome, {user.name}!</p>
      <p>Email: {user.email}</p>
    </div>
  );
}
```

Change a field in Rust, regenerate types, and TypeScript will catch any mismatches at compile time.

## Documentation

Ready to build something? Check out the [full documentation](https://cancer-rs.dev/) to get started.

## License

MIT
