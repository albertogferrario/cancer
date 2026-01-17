# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-16)

**Core value:** Agents can go from "I want an app that does X" to a working, deployed application with minimal friction.
**Current focus:** v2.0.2 Type Generator Fixes — Fix type generation issues from adotta-animali port

## Current Position

Phase: 22.8 (Nested Types Generation)
Plan: 22.8-01 (complete)
Status: Phase 22.8-01 complete — v2.0.2 Type Generator Fixes milestone complete
Last activity: 2026-01-17 — Phase 22.8-01 complete (nested type generation)

Progress: ▓▓▓▓▓▓▓▓▓▓ 100%

## Milestone Summary

| Milestone | Phases | Plans | Status | Shipped |
|-----------|--------|-------|--------|---------|
| v1.0 DX Overhaul | 1-12 | 18 | ✅ Complete | 2026-01-16 |
| v2.0 Rebrand | 13-22 | 13 | ✅ Complete | 2026-01-16 |
| v2.0.1 Macro Fix | 22.1-22.3 | 6 | ✅ Complete | 2026-01-17 |
| v2.0.2 Type Generator Fixes | 22.4-22.8 | 2/5 | ✅ Complete | 2026-01-17 |
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

None — v2.0.2 Type Generator Fixes milestone complete.

### Blockers/Concerns

**Pre-existing (unrelated to milestones):**
1. ferro-storage has unimplemented trait methods
2. Flaky shared state in test_different_methods_tracked_separately
3. test_globals_css_not_empty expects tailwind in CSS

### Roadmap Evolution

- v1.0 DX Overhaul complete: 12 phases, 18 plans (2026-01-15 to 2026-01-16)
- v2.0 Rebrand complete: 10 phases, 13 plans (2026-01-16)
- v2.0.1 Macro Fix complete: 3 phases (Phase 22.1-22.3) (2026-01-17)
- v2.0.2 Type Generator Fixes complete: 2 plans (Phase 22.4, 22.8) - 22.4-01, 22.8-01 complete
- v2.1 JSON-UI deferred: 10 phases (Phase 23-32) - awaiting v2.0.2

## Session Continuity

Last session: 2026-01-17
Stopped at: Completed Phase 22.8-01 (nested type generation) — v2.0.2 milestone complete
Resume file: None
