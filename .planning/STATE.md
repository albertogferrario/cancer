# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-16)

**Core value:** Agents can go from "I want an app that does X" to a working, deployed application with minimal friction.
**Current focus:** v2.0.2 Type Generator Fixes — Fix type generation issues from adotta-animali port

## Current Position

Phase: 22.6 (Contract Validation CLI)
Plan: 22.6-01 (Expose MCP validate_contracts as CLI command)
Status: Complete
Last activity: 2026-01-17 — Phase 22.6-01 complete

Progress: ▓▓▓▓▓▓▓▓▓░ 90%

## Milestone Summary

| Milestone | Phases | Plans | Status | Shipped |
|-----------|--------|-------|--------|---------|
| v1.0 DX Overhaul | 1-12 | 18 | ✅ Complete | 2026-01-16 |
| v2.0 Rebrand | 13-22 | 13 | ✅ Complete | 2026-01-16 |
| v2.0.1 Macro Fix | 22.1-22.3 | 6 | ✅ Complete | 2026-01-17 |
| v2.0.2 Type Generator Fixes | 22.4-22.9 | 5/? | 🔄 In Progress | - |
| v2.1 JSON-UI | 23-32 | 0/? | 📋 Planned | - |

## Accumulated Context

### Key Decisions (v2.0.2)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Serde case handling | Enum with `apply()` method | Type-safe, exhaustive pattern matching |
| Per-field vs struct-level | Field rename takes precedence | Matches serde's actual behavior |
| Import generation | Only referenced types | Keeps imports minimal and relevant |
| Re-export generation | All shared.ts types | Convenience for component imports |
| Re-export control | `--no-reexports` flag | Flexibility for projects that don't want re-exports |
| Nested type scanning | SerializeStructVisitor | Finds #[derive(Serialize)] structs by name |
| Nested resolution | Fixed-point iteration | Recursively finds all referenced types |
| ValidationErrors mapping | Direct to Record | Maps to Record<string, string[]> for proper TS types |
| Module path format | `::` separator | Consistent with Rust module paths (shelter::applications) |
| Namespaced names | PascalCase | ShelterApplicationsShowProps - clean TypeScript interface names |
| Controller hierarchy | Underscore join | shelter_dashboard preserves hierarchy without nesting |

### Key Decisions (v2.0.1)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Resolution approach | Hardcode `ferro::` | Simple, direct - no runtime detection needed |
| Previous approach | ~~proc-macro-crate~~ | Over-engineered; being removed in 22.2 |

### Key Decisions (v2.1 - deferred)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| CSS approach | Tailwind classes in HTML | No extra CSS file, inherits project theme |
| Render location | Server-side (Rust) | No JS build required |
| Component style | Beautiful by default | shadcn/ui quality via predefined components |
| Coexistence | Alongside Inertia | Use JSON-UI for simple pages, Inertia for custom |

### Pending Todos

Phase 22.6-01 complete. Ready for next phase.

### Blockers/Concerns

**Pre-existing (unrelated to milestones):**
1. ferro-storage has unimplemented trait methods
2. Flaky shared state in test_different_methods_tracked_separately
3. test_globals_css_not_empty expects tailwind in CSS

### Roadmap Evolution

- v1.0 DX Overhaul complete: 12 phases, 18 plans (2026-01-15 to 2026-01-16)
- v2.0 Rebrand complete: 10 phases, 13 plans (2026-01-16)
- v2.0.1 Macro Fix complete: 3 phases (Phase 22.1-22.3) (2026-01-17)
- v2.0.2 Type Generator Fixes in progress: 5 plans (22.4-01, 22.5-01, 22.6-01, 22.8-01, 22.9-01) (2026-01-17)
- v2.1 JSON-UI deferred: 10 phases (Phase 23-32) - awaiting v2.0.2

## Session Continuity

Last session: 2026-01-17
Stopped at: Completed Phase 22.6-01 (Contract Validation CLI)
Resume file: None
