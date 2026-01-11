# Claude Instructions for Cancer Framework

## Quick Start

**Use cancer-mcp first.** The MCP tools provide instant introspection:
- `application_info` - Project state, models, installed crates
- `list_routes` - All endpoints
- `database_schema` - Table structure
- `last_error` - Debug failures

## Workspace Structure

| Crate | Purpose | Key Files |
|-------|---------|-----------|
| `framework` | Core web framework | `src/lib.rs` (public API) |
| `cancer-cli` | CLI tool | `src/commands/` |
| `cancer-events` | Event dispatcher | `src/lib.rs` |
| `cancer-queue` | Background jobs | `src/lib.rs` |
| `cancer-notifications` | Multi-channel notifications | `src/lib.rs` |
| `cancer-broadcast` | WebSocket broadcasting | `src/lib.rs` |
| `cancer-storage` | File storage abstraction | `src/lib.rs` |
| `cancer-cache` | Caching with tags | `src/lib.rs` |
| `cancer-macros` | Proc macros | `src/lib.rs` |
| `cancer-mcp` | MCP introspection server | `src/tools/` |
| `inertia-rs` | Inertia.js adapter | `src/lib.rs` |
| `app` | Sample application | Reference implementation |

## Key Patterns

### Handler Functions
```rust
#[handler]
pub async fn show(req: Request, user: User) -> Response {
    Ok(json!({"user": user}))
}
```
- Return `Response` = `Result<HttpResponse, HttpResponse>`
- Parameters auto-extracted from request

### Services
```rust
#[service(ConcreteType)]
pub trait MyService: Send + Sync { ... }

#[injectable]
pub struct ConcreteType;
```

### Validation
```rust
Validator::new(&data)
    .rules("email", rules![required(), email()])
    .validate()
```

### Inertia
```rust
// Basic render
Inertia::render(&req, "Component", Props { ... })

// Form handlers: save context before consuming request
let ctx = SavedInertiaContext::from(&req);
let form = req.input().await?;  // Consumes req
Inertia::render_ctx(&ctx, "Component", Props { ... })  // Use saved ctx
```
Component paths validated at compile-time.

## Common Operations

### Adding features
1. Implement in appropriate crate
2. Export from `lib.rs`
3. Add to `framework/src/lib.rs` re-exports if user-facing
4. Document in docs/

### Testing
```bash
cargo test --all-features
cargo fmt --check
cargo clippy
```

### Documentation
- User docs: `docs/src/`
- API docs: `cargo doc --no-deps`

## File Locations

| Need | Location |
|------|----------|
| Public API | `framework/src/lib.rs` |
| Route macros | `cancer-macros/src/routing.rs` |
| Handler macro | `cancer-macros/src/handler.rs` |
| Validation rules | `framework/src/validation/rules/` |
| HTTP types | `framework/src/http/` |
| Database | `framework/src/database/` |
| Middleware | `framework/src/middleware/` |
| CLI commands | `cancer-cli/src/commands/` |

## Notes

- Never add co-author attribution to commits
- Run lint/tests before committing
- Prefer editing existing files over creating new ones
- Keep changes focused and minimal
