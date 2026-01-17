# Roadmap: Ferro Framework

## Milestones

- ✅ [**v1.0 DX Overhaul**](milestones/v1.0-ROADMAP.md) — Phases 1-12 (shipped 2026-01-16)
- ✅ [**v2.0 Rebrand**](milestones/v2.0-ROADMAP.md) — Phases 13-22 (shipped 2026-01-16)
- 🚧 **v2.0.1 Macro Fix** — Phase 22.1-22.2 (in progress)
- 📋 **v2.1 JSON-UI** — Phases 23-32 (planned)

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

### 🚧 v2.0.1 Macro Fix (In Progress)

**Milestone Goal:** Fix hardcoded `::ferro_rs::` paths in proc macros to use canonical `ferro::` name.

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 22.1 Macro Crate Paths | 3/3 | ✅ Complete | 2026-01-17 |
| 22.2 Simplify Macro Crate Paths | 0/1 | 📋 Planned | - |

#### Phase 22.2: Simplify Macro Crate Paths

**Goal:** Remove over-engineered `proc-macro-crate` solution and hardcode `ferro::` directly
**Depends on:** Phase 22.1

Tasks:
- Remove `proc-macro-crate` dependency from ferro-macros
- Replace all `ferro_crate()` calls with hardcoded `quote!(ferro)`
- Delete `crate_path.rs` module
- Update documentation to remove "flexible naming" section

Plans:
- [ ] 22.2-01: Remove proc-macro-crate and hardcode ferro:: paths

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
| v2.0.1 Macro Fix | 22.1-22.2 | 3/4 | 🚧 In Progress | - |
| v2.1 JSON-UI | 23-32 | 0/? | 📋 Planned | - |

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
