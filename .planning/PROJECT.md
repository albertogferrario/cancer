# Cancer Framework DX Overhaul

## What This Is

A developer experience and agent-friendliness overhaul for the Cancer web framework. The goal is to enable AI agents to build complete web applications from natural language descriptions for non-technical users — reducing boilerplate, expanding introspection, and improving scaffolding.

## Core Value

Agents can go from "I want an app that does X" to a working, deployed application with minimal friction. Every framework decision optimizes for agent comprehension and generation capability.

## Requirements

### Validated

<!-- Shipped and confirmed valuable — existing framework capabilities. -->

- ✓ Laravel-inspired architecture with handlers, middleware, routing — existing
- ✓ SeaORM database layer with migrations and model abstraction — existing
- ✓ Session-based authentication with pluggable providers — existing
- ✓ Policy-based authorization with Gate abstraction — existing
- ✓ Validation builder with field-level error messages — existing
- ✓ React/Inertia.js full-stack integration with compile-time validation — existing
- ✓ CLI with make:controller, make:model, make:migration scaffolding — existing
- ✓ MCP server with 30+ introspection tools (routes, models, schema, events, etc.) — existing
- ✓ Auto-generated TypeScript types from Rust models — existing
- ✓ Event dispatcher with async listeners — existing
- ✓ Redis-backed job queue with workers — existing
- ✓ Multi-channel notifications (email, database) — existing
- ✓ WebSocket broadcasting support — existing
- ✓ File storage abstraction (local, S3) — existing
- ✓ Tag-based caching — existing

### Active

<!-- Current scope. Building toward these. -->

**Boilerplate Reduction:**
- [ ] Simplify handler definitions — less ceremony for common patterns
- [ ] Reduce model boilerplate — derive more, write less
- [ ] Streamline validation syntax — more concise rule definitions
- [ ] Convention-over-configuration for common scenarios

**MCP Introspection Expansion:**
- [ ] Agent can understand full app intent, not just structure
- [ ] Better error context for agent diagnosis
- [ ] Relationship and data flow visibility
- [ ] Generation hints embedded in introspection responses

**CLI Scaffolding Improvements:**
- [ ] Higher-level scaffolding (full features, not just files)
- [ ] Smarter defaults based on existing patterns
- [ ] Interactive scaffolding with context awareness
- [ ] Better integration between generated components

**Agent-First Design:**
- [ ] Clear, predictable patterns agents can learn and apply
- [ ] Self-documenting code structure
- [ ] Errors that explain what to do, not just what went wrong

### Out of Scope

<!-- Explicit boundaries. Includes reasoning to prevent re-adding. -->

- New major features (payments, subscriptions, etc.) — focus is DX, not feature expansion
- Frontend framework changes — React/Inertia stack stays as-is
- Database driver changes — SeaORM works, no need to replace

## Context

This is a brownfield improvement of an existing framework. The Cancer framework already works and has a sample application demonstrating its capabilities.

The primary use case shift: from "developer builds apps" to "agent builds apps for non-technical users." This requires:
- Patterns simple enough for agents to reliably generate
- Introspection deep enough for agents to understand existing code
- Error messages clear enough for agents to self-correct

Reference codebase documentation in `.planning/codebase/`:
- ARCHITECTURE.md — Layer breakdown and request lifecycle
- STACK.md — Dependencies and tooling
- PATTERNS.md, HOTSPOTS.md, TESTING.md, CONVENTIONS.md, DOCUMENTATION.md

## Constraints

- **Compatibility**: Existing sample app should work after changes (may need migration)
- **Rust Edition**: 2021 edition, no nightly-only features

## Key Decisions

<!-- Decisions that constrain future work. Add throughout project lifecycle. -->

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Agent-first over developer-first | Non-technical users via agents is the target market | — Pending |
| Breaking changes acceptable | No backwards compatibility constraint allows cleaner APIs | — Pending |

---
*Last updated: 2026-01-15 after initialization*
