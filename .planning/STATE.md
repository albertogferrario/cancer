# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-16)

**Core value:** Agents can go from "I want an app that does X" to a working, deployed application with minimal friction.
**Current focus:** v2.0.1 Macro Fix — Complete Cancer→Ferro rebrand

## Current Position

Phase: 22.3 (Complete Cancer→Ferro Rebrand)
Plan: 0 of ? in current phase
Status: Not planned yet
Last activity: 2026-01-17 — Added Phase 22.3

Progress: ░░░░░░░░░░ 0%

## Milestone Summary

| Milestone | Phases | Plans | Status | Shipped |
|-----------|--------|-------|--------|---------|
| v1.0 DX Overhaul | 1-12 | 18 | ✅ Complete | 2026-01-16 |
| v2.0 Rebrand | 13-22 | 13 | ✅ Complete | 2026-01-16 |
| v2.0.1 Macro Fix | 22.1-22.3 | 4/? | 🚧 In Progress | - |
| v2.1 JSON-UI | 23-32 | 0/? | 📋 Planned | - |

## Accumulated Context

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

Phase 22.3 needs planning — fix remaining Cancer references from FRAMEWORK_ISSUES.md.

### Blockers/Concerns

**Pre-existing (unrelated to milestones):**
1. ferro-storage has unimplemented trait methods
2. Flaky shared state in test_different_methods_tracked_separately
3. test_globals_css_not_empty expects tailwind in CSS

### Roadmap Evolution

- v1.0 DX Overhaul complete: 12 phases, 18 plans (2026-01-15 to 2026-01-16)
- v2.0 Rebrand complete: 10 phases, 13 plans (2026-01-16)
- v2.0.1 Macro Fix inserted: 2 phases (Phase 22.1-22.2) - urgent fix before v2.1
- Phase 22.2 added: Simplify macro crate paths (undo proc-macro-crate workaround)
- v2.1 JSON-UI deferred: 10 phases (Phase 23-32) - awaiting v2.0.1
- Phase 22.3 added: Complete Cancer→Ferro rebrand (fix remaining references)

## Session Continuity

Last session: 2026-01-17
Stopped at: Added Phase 22.3 for branding cleanup
Resume file: None
