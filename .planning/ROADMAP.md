# Roadmap: Ferro Framework

## Milestones

- ✅ [**v1.0 DX Overhaul**](milestones/v1.0-ROADMAP.md) — Phases 1-12 (shipped 2026-01-16)
- ✅ [**v2.0 Rebrand**](milestones/v2.0-ROADMAP.md) — Phases 13-22 (shipped 2026-01-16)
- ✅ **v2.0.1 Macro Fix** — Phase 22.1-22.3 (shipped 2026-01-17)
- 🚧 **v2.0.2 Type Generator Fixes** — Phase 22.4-22.9 (in progress)
- 📋 **v2.1 JSON-UI** — Phases 23-32 (planned)

---

### 🚧 v2.0.2 Type Generator Fixes (In Progress)

**Milestone Goal:** Fix type generation issues discovered during adotta-animali port to improve TypeScript integration reliability.

**Source:** [.planning/backlog/adotta-animali-issues.md](backlog/adotta-animali-issues.md)

#### Phase 22.4: Type Generator Fixes

**Goal**: Fix missing shared.ts imports, type re-exports, and serde attribute handling in generated TypeScript files
**Depends on**: v2.0.1 complete
**Research**: Unlikely (internal patterns)

Issues addressed:
- #1 Type Generator: Missing Imports from shared.ts (High)
- #5 Type Re-exports Not Generated (Medium)
- #8 Serde rename_all Causes Silent Frontend Failures (High)

Plans:
- [x] 22.4-01: Fix serde attributes, shared.ts imports, type re-exports

#### Phase 22.5: Prop Naming Collisions

**Goal**: Resolve InertiaProps naming collisions and duplicate routes.ts entries
**Depends on**: Phase 22.4
**Research**: Unlikely (internal patterns)

Issues addressed:
- #3 InertiaProps Naming Collisions (High)
- #2 Type Generator: Duplicate Properties in routes.ts (Medium)

Plans:
- [x] 22.5-01: Module path tracking and namespaced type generation

#### Phase 22.6: Contract Validation CLI

**Goal**: Expose MCP validate_contracts tool as a CLI command with structural nesting validation for CI integration
**Depends on**: Phase 22.5
**Research**: Unlikely (MCP tool already exists)

Issues addressed:
- #4 No Props Contract Validation CLI (Medium)
- #9 Nested vs Flat Props Structure Mismatch (High)

Plans:
- [x] 22.6-01: Expose MCP validate_contracts as CLI command with structural nesting validation

#### Phase 22.7: DateTime Handling

**Goal**: Improve datetime field handling with proper types instead of strings
**Depends on**: Phase 22.6
**Research**: Unlikely (type generator patterns established)

Issues addressed:
- #6 Inconsistent Date/Time Field Handling (Low)

Plans:
- [ ] 22.7-01: Add chrono datetime type support to TypeScript generator

#### Phase 22.8: Nested Types Generation

**Goal**: Generate TypeScript interfaces for all nested/referenced types used in InertiaProps, not just the page props themselves
**Depends on**: Phase 22.7
**Research**: Unlikely (internal patterns)

Issues addressed:
- Type Generator Missing Nested Types (High) - Props reference undefined types like UserInfo, MenuSummary, TenantInfo

Plans:
- [x] 22.8-01: Recursive nested type resolution and generation

#### Phase 22.9: ValidationErrors Type

**Goal**: Add dedicated ValidationErrors type that generates proper TypeScript Record<string, string[]> instead of unknown
**Depends on**: Phase 22.8
**Research**: Unlikely (internal patterns)

Issues addressed:
- ValidationErrors should have a dedicated type (Medium) - Avoids manual type casts in frontend forms

Plans:
- [ ] 22.9-01: ValidationErrors type handling

**Deferred to future milestone:**
- #7 Missing Animal Images Relationship (Eager Loading) - Significant feature requiring architectural work

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

### ✅ v2.0.1 Macro Fix (Complete)

**Milestone Goal:** Fix hardcoded `::ferro_rs::` paths in proc macros to use canonical `ferro::` name.

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 22.1 Macro Crate Paths | 3/3 | ✅ Complete | 2026-01-17 |
| 22.2 Simplify Macro Crate Paths | 1/1 | ✅ Complete | 2026-01-17 |
| 22.3 Complete Rebrand | 2/2 | ✅ Complete | 2026-01-17 |

**Total:** 3 phases, 6 plans

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
| v2.0.1 Macro Fix | 22.1-22.3 | 6 | ✅ Complete | 2026-01-17 |
| v2.0.2 Type Generator Fixes | 22.4-22.9 | 4/7 | 🚧 In Progress | - |
| v2.1 JSON-UI | 23-32 | 0/? | 📋 Planned | - |

## Progress (v2.0.2 Type Generator Fixes)

| Phase | Plans | Status | Completed |
|-------|-------|--------|-----------|
| 22.4. Type Generator Fixes | 1/1 | Complete | 2026-01-17 |
| 22.5. Prop Naming Collisions | 1/1 | Complete | 2026-01-17 |
| 22.6. Contract Validation CLI | 0/1 | Planned | - |
| 22.7. DateTime Handling | 0/? | Not started | - |
| 22.8. Nested Types Generation | 1/1 | Complete | 2026-01-17 |
| 22.9. ValidationErrors Type | 0/1 | Planned | - |

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
