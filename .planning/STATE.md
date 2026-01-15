# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-15)

**Core value:** Agents can go from "I want an app that does X" to a working, deployed application with minimal friction.
**Current focus:** Phase 7 Plan 1 complete — Ready for Phase 8

## Current Position

Phase: 7 of 12 (MCP Relationship Visibility) - COMPLETE
Plan: 1 of 1 in phase - COMPLETE
Status: Completed all relationship visibility tools
Last activity: 2026-01-15 — Completed 07-01-PLAN (MCP Relationship Visibility)

Progress: ██████████ 58%

## Performance Metrics

**Velocity:**
- Total plans completed: 7
- Average duration: 30 min
- Total execution time: 3.5 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1 | 1 | 15 min | 15 min |
| 2 | 1 | 30 min | 30 min |
| 3 | 1 | 45 min | 45 min |
| 4 | 1 | 25 min | 25 min |
| 5 | 1 | 35 min | 35 min |
| 6 | 1 | 30 min | 30 min |
| 7 | 1 | 30 min | 30 min |

**Recent Trend:**
- Last 7 plans: All completed successfully
- Trend: Stable velocity with growing codebase complexity

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

1. **Action injection deferred** - Handler macro doesn't support injectable types as parameters. Keep using App::resolve until macro enhancement.
2. **CancerModel derive on entities** - Apply derive to entity files (auto-generated) not model files. Model files remain minimal with only custom code.
3. **Fully qualified trait calls in macro** - Use `<Entity as trait>::method()` syntax to avoid scoping issues.
4. **ValidateRules not Validate** - Named derive macro `ValidateRules` to avoid conflict with validator crate's `Validate` derive.
5. **#[rule(...)] not #[validate(...)]** - Used `rule` attribute name to avoid namespace collision with validator crate.
6. **Float literals for rules** - Numeric rule arguments like `min()` require float literals: `min(8.0)` not `min(8)`.
7. **Declarative macro for resource!** - Used macro_rules! instead of proc macro to stay consistent with existing routing macros.
8. **Module path capture pattern** - Used `$($controller:ident)::+` pattern to properly capture and expand module paths with `::action` suffix.
9. **Tool vs Resource for Glossary** - Implemented domain_glossary as tool rather than MCP resource for simpler agent consumption.
10. **Inference over Annotation** - Domain meaning inferred from naming patterns rather than requiring explicit annotations.
11. **Structured Description Format** - Standardized on When/Returns/Combine format for all tool descriptions.
12. **Regex caching with once_cell** - Used once_cell::sync::Lazy to cache compiled regex patterns for route extraction and model detection.
13. **Category-specific fix prioritization** - Fix suggestions have priority 1-5 where 1 is highest for most actionable fixes.
14. **Static error patterns** - Error patterns are hardcoded rather than dynamic to ensure consistency.
15. **Known models validation** - route_dependencies only reports models that exist in list_models, reducing false positives.
16. **Graph node IDs with prefixes** - Use `route:`, `model:`, `component:` prefixes for unique, type-safe identification.
17. **Bidirectional FK edges** - Both belongs_to and has_many edges created for each FK relationship.

### Pending Todos

None.

### Blockers/Concerns

1. **Pre-existing test failures**: cancer-storage has unimplemented trait methods. Unrelated to current work but blocks full test suite.
2. **Pre-existing metrics test failure**: Flaky shared state issue in test_record_request_increments_count.
3. **Pre-existing clippy warnings**: cancer-macros and cancer-events have clippy warnings unrelated to MCP work.

## Session Continuity

Last session: 2026-01-15
Stopped at: Completed Phase 7 Plan 1
Resume file: None
