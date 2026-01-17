# Roadmap: Ferro Framework

## Milestones

- ✅ [**v1.0 DX Overhaul**](milestones/v1.0-ROADMAP.md) — Phases 1-12 (shipped 2026-01-16)
- ✅ [**v2.0 Rebrand**](milestones/v2.0-ROADMAP.md) — Phases 13-22 (shipped 2026-01-16)
- 🚧 **v2.0.1 Macro Fix** — Phase 22.1 (in progress)
- 📋 **v2.1 JSON-UI** — Phases 23-32 (planned)

---

### 🚧 v2.0.1 Macro Fix (In Progress)

**Milestone Goal:** Fix hardcoded `::ferro_rs::` paths in proc macros using `proc-macro-crate` for dynamic resolution.

#### Phase 22.1: Macro Crate Path Resolution (INSERTED)

**Goal**: Replace hardcoded `::ferro_rs::` paths with dynamic crate name resolution
**Depends on**: Phase 22 (v2.0 complete)
**Research**: Likely (proc-macro-crate usage patterns)
**Research topics**: proc-macro-crate API, FoundCrate handling, fallback strategies

**Problem**: Macros generate code with hardcoded `::ferro_rs::` paths, forcing users to name their dependency exactly `ferro_rs` in Cargo.toml. This breaks natural import conventions (`ferro = ...`).

**Affected files**:
- `ferro-macros/src/test_macro.rs` - 8 occurrences
- `ferro-macros/src/request.rs` - 2 occurrences
- `ferro-macros/src/redirect.rs` - 2 occurrences
- `ferro-macros/src/inertia.rs` - 5+ occurrences

Plans:
- [ ] [22.1-01](phases/22.1-macro-crate-paths/PLAN-22.1-01.md): Add proc-macro-crate dependency and helper function
- [ ] [22.1-02](phases/22.1-macro-crate-paths/PLAN-22.1-02.md): Update all macros to use dynamic crate resolution
- [ ] [22.1-03](phases/22.1-macro-crate-paths/PLAN-22.1-03.md): Test with alternate crate names and document

---

### 📋 v2.1 JSON-UI (Planned)

**Milestone Goal:** Add JSON-based UI rendering as an alternative to Inertia for rapid, beautiful UI without frontend builds.

#### Phase 23: JSON-UI Schema

**Goal**: Define core JSON schema for UI elements (components, props, visibility rules, actions)
**Depends on**: Previous milestone complete
**Research**: Likely (study json-render patterns, JSON schema design)
**Research topics**: json-render by Vercel, component catalog patterns, action declaration

Plans:
- [ ] 23-01: TBD (run /gsd:plan-phase 23 to break down)

#### Phase 24: Component Catalog

**Goal**: Implement default components: Table, Form, Card, Input, Button, Alert, Badge, Modal, etc.
**Depends on**: Phase 23
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 24-01: TBD

#### Phase 25: Data Binding

**Goal**: JSONPath-based data binding to handler props and responses
**Depends on**: Phase 24
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 25-01: TBD

#### Phase 26: Action System

**Goal**: Map declared actions to Ferro handlers with form submissions and confirmations
**Depends on**: Phase 25
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 26-01: TBD

#### Phase 27: Validation Integration

**Goal**: Connect to existing Ferro validation system, display errors in components
**Depends on**: Phase 26
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 27-01: TBD

#### Phase 28: HTML Renderer

**Goal**: Rust-based JSON→HTML renderer outputting Tailwind classes
**Depends on**: Phase 27
**Research**: Likely (templating approaches, HTML generation in Rust)
**Research topics**: maud, askama, or custom builder patterns

Plans:
- [ ] 28-01: TBD

#### Phase 29: Layout System

**Goal**: Layouts, partials, and slots for page structure
**Depends on**: Phase 28
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 29-01: TBD

#### Phase 30: CLI Scaffolding

**Goal**: `ferro make:json-view` command to generate JSON view files
**Depends on**: Phase 29
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 30-01: TBD

#### Phase 31: MCP UI Tools

**Goal**: MCP tools to generate and inspect JSON-UI specs from models/routes
**Depends on**: Phase 30
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 31-01: TBD

#### Phase 32: Documentation

**Goal**: Guides, component reference, and examples for JSON-UI
**Depends on**: Phase 31
**Research**: Unlikely (internal patterns)

Plans:
- [ ] 32-01: TBD

---

## Completed Milestones

<details>
<summary>✅ v2.0 Rebrand (Phases 13-22) — SHIPPED 2026-01-16</summary>

**Milestone Goal:** Rename the framework from "cancer" to "ferro" for crates.io publication and public release.

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 13. Rebrand Audit | 1/1 | Complete | 2026-01-16 |
| 14. Core Framework Rename | 1/1 | Complete | 2026-01-16 |
| 15. Supporting Crates Rename | 1/1 | Complete | 2026-01-16 |
| 16. CLI Rebrand | 1/1 | Complete | 2026-01-16 |
| 17. MCP Server Rebrand | 1/1 | Complete | 2026-01-16 |
| 18. Documentation Update | 3/3 | Complete | 2026-01-16 |
| 19. Sample App Migration | 1/1 | Complete | 2026-01-16 |
| 20. Templates & Scaffolding | 1/1 | Complete | 2026-01-16 |
| 21. Repository & CI | 1/1 | Complete | 2026-01-16 |
| 22. Publishing & Announcement | 2/2 | Complete | 2026-01-16 |

**Total:** 10 phases, 13 plans

[Full details →](milestones/v2.0-ROADMAP.md)

</details>

<details>
<summary>✅ v1.0 DX Overhaul (Phases 1-12) — SHIPPED 2026-01-16</summary>

**Milestone Goal:** Transform the framework from developer-centric to agent-first.

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 1. Handler Simplification | 1/1 | Complete | 2026-01-15 |
| 2. Model Boilerplate Reduction | 1/1 | Complete | 2026-01-15 |
| 3. Validation Syntax Streamlining | 1/1 | Complete | 2026-01-15 |
| 4. Convention-over-Configuration | 1/1 | Complete | 2026-01-15 |
| 5. MCP Intent Understanding | 1/1 | Complete | 2026-01-15 |
| 6. MCP Error Context | 1/1 | Complete | 2026-01-15 |
| 7. MCP Relationship Visibility | 1/1 | Complete | 2026-01-15 |
| 8. MCP Generation Hints | 1/1 | Complete | 2026-01-15 |
| 9. CLI Feature Scaffolding | 1/1 | Complete | 2026-01-15 |
| 10. CLI Smart Defaults | 1/1 | Complete | 2026-01-15 |
| 11. CLI Component Integration | 3/3 | Complete | 2026-01-15 |
| 12. Agent-First Polish | 5/5 | Complete | 2026-01-16 |

**Total:** 12 phases, 18 plans

[Full details →](milestones/v1.0-ROADMAP.md)

</details>

---

## Progress Summary

| Milestone | Phases | Plans | Status | Shipped |
|-----------|--------|-------|--------|---------|
| v1.0 DX Overhaul | 1-12 | 18 | ✅ Complete | 2026-01-16 |
| v2.0 Rebrand | 13-22 | 13 | ✅ Complete | 2026-01-16 |
| v2.0.1 Macro Fix | 22.1 | 0/3 | 🚧 In Progress | - |
| v2.1 JSON-UI | 23-32 | 0/? | 📋 Planned | - |

## Progress (v2.0.1 Macro Fix)

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 22.1 Macro Crate Paths (INSERTED) | 3/3 | Planned | - |

## Progress (v2.1 JSON-UI)

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 23. JSON-UI Schema | 0/? | Not started | - |
| 24. Component Catalog | 0/? | Not started | - |
| 25. Data Binding | 0/? | Not started | - |
| 26. Action System | 0/? | Not started | - |
| 27. Validation Integration | 0/? | Not started | - |
| 28. HTML Renderer | 0/? | Not started | - |
| 29. Layout System | 0/? | Not started | - |
| 30. CLI Scaffolding | 0/? | Not started | - |
| 31. MCP UI Tools | 0/? | Not started | - |
| 32. Documentation | 0/? | Not started | - |
