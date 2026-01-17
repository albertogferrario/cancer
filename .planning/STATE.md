# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-16)

**Core value:** Agents can go from "I want an app that does X" to a working, deployed application with minimal friction.
**Current focus:** v2.0.3 DO Apps Deploy — Enable one-click deployment to DigitalOcean App Platform

## Current Position

Phase: 22.10 (DigitalOcean Apps One-Click Deploy)
Plan: 22.10-01 (DigitalOcean App Platform CLI Command)
Status: Phase complete
Last activity: 2026-01-17 — Completed 22.10-01-PLAN.md

Progress: ██████████ 100%

## Milestone Summary

| Milestone | Phases | Plans | Status | Shipped |
|-----------|--------|-------|--------|---------|
| v1.0 DX Overhaul | 1-12 | 18 | ✅ Complete | 2026-01-16 |
| v2.0 Rebrand | 13-22 | 13 | ✅ Complete | 2026-01-16 |
| v2.0.1 Macro Fix | 22.1-22.3 | 6 | ✅ Complete | 2026-01-17 |
| v2.0.2 Type Generator Fixes | 22.4-22.9 | 6 | ✅ Complete | 2026-01-17 |
| v2.0.3 DO Apps Deploy | 22.10 | 1/1 | ✅ Complete | 2026-01-17 |
| v2.1 JSON-UI | 23-32 | 0/? | 📋 Planned | - |

## Accumulated Context

### Key Decisions (v2.0.3)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Require --repo flag | Explicit over implicit | Simpler than git remote detection or interactive prompts |
| Follow docker_init pattern | Consistency | Familiar error handling and messaging style |

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
| DateTime variant structure | No inner type | All chrono types serialize to ISO8601 strings regardless of timezone |
| DateTime type recognition | Before `other` fallback | Catch datetime types before they become Custom() |

### Pending Todos

v2.0.3 complete. Milestone ready for archive.

### Blockers/Concerns

**Pre-existing (unrelated to milestones):**
1. ferro-storage has unimplemented trait methods
2. Flaky shared state in test_different_methods_tracked_separately
3. test_globals_css_not_empty expects tailwind in CSS

### Roadmap Evolution

- v1.0 DX Overhaul complete: 12 phases, 18 plans (2026-01-15 to 2026-01-16)
- v2.0 Rebrand complete: 10 phases, 13 plans (2026-01-16)
- v2.0.1 Macro Fix complete: 3 phases (Phase 22.1-22.3) (2026-01-17)
- v2.0.2 Type Generator Fixes complete: 6 phases, 6 plans (Phase 22.4-22.9) (2026-01-17)
- v2.0.3 DO Apps Deploy complete: 1 phase, 1 plan (Phase 22.10) (2026-01-17)
- v2.1 JSON-UI deferred: 10 phases (Phase 23-32) - awaiting v2.0.3

## Session Continuity

Last session: 2026-01-17
Stopped at: Completed 22.10-01-PLAN.md (v2.0.3 milestone complete)
Resume file: None
