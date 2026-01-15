# Codebase Structure

**Analysis Date:** 2026-01-15

## Directory Layout

```
cancer/
├── app/                    # Sample/reference application
│   ├── frontend/          # React + TypeScript frontend
│   ├── src/               # Rust application code
│   └── public/            # Static assets (Vite output)
├── framework/             # Core web framework crate
│   └── src/               # Framework implementation
├── cancer-cli/            # CLI scaffolding tool
├── cancer-macros/         # Procedural macros
├── cancer-events/         # Event dispatcher
├── cancer-queue/          # Background job queue
├── cancer-notifications/  # Multi-channel notifications
├── cancer-broadcast/      # WebSocket broadcasting
├── cancer-storage/        # File storage abstraction
├── cancer-cache/          # Caching layer
├── cancer-mcp/            # MCP introspection server
├── inertia-rs/            # Inertia.js adapter
├── docs/                  # User documentation (mdBook)
├── scripts/               # Helper scripts
├── Cargo.toml             # Workspace definition
├── bacon.toml             # Dev watcher config
└── dev.sh                 # Development launcher
```

## Directory Purposes

**app/src/**
- Purpose: Sample application demonstrating framework usage
- Contains: Controllers, models, migrations, middleware, routes
- Key files: `main.rs` (entry), `routes.rs` (route definitions), `bootstrap.rs` (service registration)
- Subdirectories: `controllers/`, `models/`, `actions/`, `middleware/`, `config/`, `migrations/`, `providers/`

**app/frontend/**
- Purpose: React frontend with Inertia.js
- Contains: TypeScript components, pages, types
- Key files: `vite.config.ts`, `tsconfig.json`, `package.json`
- Subdirectories: `src/pages/`, `src/types/`

**framework/src/**
- Purpose: Core framework implementation
- Contains: HTTP, routing, database, auth, validation, middleware, sessions
- Key files: `lib.rs` (public API), `app.rs` (application builder), `server.rs` (HTTP server)
- Subdirectories: `http/`, `routing/`, `database/`, `auth/`, `authorization/`, `validation/`, `middleware/`, `session/`, `cache/`, `config/`, `container/`, `testing/`

**cancer-cli/src/**
- Purpose: CLI code generators and project scaffolding
- Contains: Commands for creating controllers, models, migrations, etc.
- Key files: `main.rs` (CLI entry), `templates/mod.rs` (code templates)
- Subdirectories: `commands/` (33 generator commands), `templates/`

**cancer-macros/src/**
- Purpose: Procedural macros for framework
- Contains: Handler, routing, service, model macros
- Key files: `lib.rs`, `handler.rs`, `routing.rs`, `service.rs`, `model.rs`, `inertia.rs`

**cancer-events/src/**
- Purpose: Event dispatcher system
- Contains: Event traits, dispatcher, listener registration
- Key files: `lib.rs`, `dispatcher.rs`

**cancer-queue/src/**
- Purpose: Background job queue (Redis-backed)
- Contains: Job definitions, queue management, workers
- Key files: `lib.rs`, `queue.rs`, `job.rs`, `worker.rs`, `dispatcher.rs`

**cancer-notifications/src/**
- Purpose: Multi-channel notification system
- Contains: Mail, database, Slack channels
- Key files: `lib.rs`, `dispatcher.rs`, `notification.rs`
- Subdirectories: `channels/` (database, mail)

**cancer-broadcast/src/**
- Purpose: WebSocket real-time broadcasting
- Contains: Broadcaster, channel management
- Key files: `lib.rs`, `broadcaster.rs`, `channel.rs`, `config.rs`

**cancer-storage/src/**
- Purpose: File storage abstraction
- Contains: Local and S3 drivers
- Key files: `lib.rs`, `facade.rs`
- Subdirectories: `drivers/` (local, s3)

**cancer-cache/src/**
- Purpose: Cache abstraction
- Contains: Memory and Redis stores
- Key files: `lib.rs`, `cache.rs`
- Subdirectories: `stores/` (redis, memory)

**cancer-mcp/src/**
- Purpose: Model Context Protocol introspection server
- Contains: Debug tools for AI-assisted development
- Key files: `lib.rs`
- Subdirectories: `tools/`

**inertia-rs/src/**
- Purpose: Inertia.js server-side adapter
- Contains: Request handling, response generation
- Key files: `lib.rs`, `response.rs`, `config.rs`, `request.rs`

**docs/src/**
- Purpose: User-facing documentation (mdBook)
- Contains: Getting started, features, reference docs
- Subdirectories: `getting-started/`, `features/`, `the-basics/`, `reference/`

## Key File Locations

**Entry Points:**
- `cancer-cli/src/main.rs` - CLI tool entry (binary: `cancer`)
- `app/src/main.rs` - Application entry (binary: `app`)
- `framework/src/lib.rs` - Framework library root

**Configuration:**
- `Cargo.toml` - Workspace and dependency management
- `app/frontend/tsconfig.json` - TypeScript configuration
- `app/frontend/vite.config.ts` - Vite build configuration
- `bacon.toml` - Development watch tool

**Core Logic:**
- `framework/src/http/request.rs` - Request handling
- `framework/src/http/response.rs` - Response building
- `framework/src/routing/router.rs` - Route matching
- `framework/src/database/model.rs` - ORM model trait
- `framework/src/container/mod.rs` - DI container

**Testing:**
- `framework/src/testing/mod.rs` - Test utilities
- `framework/src/testing/factory.rs` - Test data factories
- `framework/src/testing/http.rs` - HTTP test client

**Documentation:**
- `README.md` - Project overview
- `CLAUDE.md` - Development guidelines
- `docs/src/` - Full documentation

## Naming Conventions

**Files:**
- `snake_case.rs` - Rust source files
- `mod.rs` - Module roots
- `lib.rs` - Library crate roots
- `main.rs` - Binary crate roots
- `PascalCase.tsx` - React components

**Directories:**
- `snake_case` - Rust directories
- `kebab-case` - Crate names (cancer-events, cancer-queue)
- Plural for collections: `controllers/`, `models/`, `commands/`

**Special Patterns:**
- `cancer-*` - Framework addon crates
- `make_*.rs` - Generator command files
- `*.test.rs` - Test modules (inline, not separate files)

## Where to Add New Code

**New Feature (framework-level):**
- Primary code: `framework/src/{feature}/`
- Tests: Inline `#[cfg(test)]` modules
- Public API: Export from `framework/src/lib.rs`
- Documentation: `docs/src/features/{feature}.md`

**New Application Feature:**
- Controller: `app/src/controllers/{name}.rs`
- Model: `app/src/models/{name}.rs`
- Action: `app/src/actions/{name}.rs`
- Route: Add to `app/src/routes.rs`
- Frontend page: `app/frontend/src/pages/{Name}.tsx`

**New CLI Command:**
- Implementation: `cancer-cli/src/commands/{command}.rs`
- Registration: Add to `cancer-cli/src/main.rs`
- Templates: Add to `cancer-cli/src/templates/mod.rs`

**New Addon Crate:**
- Create: `cancer-{name}/` directory
- Add to: Workspace members in `Cargo.toml`
- Export: Re-export from `framework/src/lib.rs` if user-facing

**Utilities:**
- Framework utilities: `framework/src/{module}/`
- Shared across crates: Consider new crate or in `framework/`

## Special Directories

**app/public/assets/**
- Purpose: Vite-built frontend assets
- Source: Generated by `npm run build` in `app/frontend/`
- Committed: Yes (production build artifacts)

**target/**
- Purpose: Cargo build output
- Source: Rust compiler
- Committed: No (in .gitignore)

**node_modules/**
- Purpose: npm dependencies
- Source: `npm install` in `app/frontend/`
- Committed: No (in .gitignore)

---

*Structure analysis: 2026-01-15*
*Update when directory structure changes*
