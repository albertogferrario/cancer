# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-01-15)

**Core value:** Agents can go from "I want an app that does X" to a working, deployed application with minimal friction.
**Current focus:** v2.0 Rebrand — Rename framework from "cancer" to "ferro" for crates.io publication

## Current Position

Phase: 18 of 22 (Documentation Update)
Plan: 03 completed
Status: Ready for plan 18-04
Last activity: 2026-01-16 — Completed 18-03 (PROJECT.md and codebase docs rebranded)

Progress: ██████░░░░░░░░░░░░ 30%

## Performance Metrics

**Velocity:**
- Total plans completed: 20 (v1.0 + 15-01 + 16-01)
- Average duration: 23 min
- Total execution time: 7 hours 45 min

**By Phase (v1.0):**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1 | 1 | 15 min | 15 min |
| 2 | 1 | 30 min | 30 min |
| 3 | 1 | 45 min | 45 min |
| 4 | 1 | 25 min | 25 min |
| 5 | 1 | 35 min | 35 min |
| 6 | 1 | 30 min | 30 min |
| 7 | 1 | 30 min | 30 min |
| 8 | 1 | 25 min | 25 min |
| 9 | 1 | 35 min | 35 min |
| 10 | 1 | 30 min | 30 min |
| 11 | 3 | 75 min | 25 min |
| 12 | 5 | 50 min | 10 min |

**v2.0 Rebrand:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 14 | 1 | (part of prev session) | - |
| 15 | 1 | 15 min | 15 min |
| 16 | 1 | 25 min | 25 min |
| 17 | 1 | 12 min | 12 min |
| 18 | 3 | 21 min | 7 min |

**Recent Trend:**
- Last 24 plans: All completed successfully
- v2.0 rebrand in progress: Phase 18 plan 03 complete

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
18. **Dedicated generation tools** - Created generation_context and code_templates as separate tools rather than adding include_hints param to existing tools.
19. **Template placeholder format** - Used `{{Name}}` double-brace syntax for template placeholders.
20. **dialoguer for prompts** - Used dialoguer::Confirm for interactive route registration prompts.
21. **String manipulation over AST** - Route injection uses simple string matching rather than Rust AST parsing.
22. **Single analyzer call** - Project structure analyzed once per scaffold, conventions passed to all detection functions.
23. **SmartDefaults tracking struct** - Separate tracking of detections vs applied flags enables showing user what was auto-detected.
24. **Field inference returns tuple** - `infer_field_type()` returns `(FieldType, reason)` for display in summary.
25. **Interactive by default** - Smart defaults summary prompts for confirmation unless `--yes` or `--quiet` passed.
26. **ForeignKeyInfo struct** - Contains field_name, target_model, target_table, validated for comprehensive FK information.
27. **Validated FK flag** - FK detection checks if target model exists in project, enabling smart suggestions.
28. **Factory-integrated tests** - Tests conditionally use factory template when both --with-tests and --with-factory are used.
29. **Cascade FK behavior** - FK constraints use ON DELETE CASCADE and ON UPDATE CASCADE as sensible defaults.
30. **Factory with_* methods** - Factories get builder methods for FK fields plus create_with_relations() for auto-creation.
31. **Display field cascade** - Select options use `name ?? title ?? email ?? id` to show meaningful text for any model type.
32. **fail_with helper pattern** - Centralized error formatting in main.rs with context + cause + fix list format.
33. **ENV precedence for ports** - CLI default (8000) triggers env lookup, explicit CLI arg overrides .env values.
34. **Self-documenting .env.example** - All environment variables documented with comments explaining purpose, format, and defaults.
35. **Inertia::redirect() over redirect!()** - For Inertia pages, use Inertia::redirect() which returns 303 for POST-like methods and includes X-Inertia header.
36. **Rebrand to "ferro"** - Framework renamed from "cancer" to "ferro" for crates.io compatibility and public release.
37. **Alias pattern for gradual migration** - Use `cancer = { path = "../framework", package = "ferro" }` to keep code imports working during phased rename.
38. **Doctest crate name updates** - Doctests must use actual crate name (ferro_events) not alias, unlike runtime code which can use aliases.

### Pending Todos

None.

### Blockers/Concerns

1. **Pre-existing test failures**: ferro-storage has unimplemented trait methods. Unrelated to current work but blocks full test suite.
2. **Pre-existing metrics test failure**: Flaky shared state issue in test_different_methods_tracked_separately.
3. **Pre-existing CSS test failure**: test_globals_css_not_empty expects tailwind in CSS.

### Roadmap Evolution

- v1.0 DX Overhaul complete: 12 phases, 18 plans (2026-01-15 to 2026-01-16)
- v2.0 Rebrand in progress: cancer → ferro, 10 phases (Phase 13-22)
- Phase 14 complete: Core framework renamed
- Phase 15 complete: Supporting crates renamed
- Phase 16 complete: CLI binary renamed to ferro
- Phase 17 complete: MCP server rebranded to Ferro
- Phase 18 in progress: Documentation update (plan 03 complete)

## Session Continuity

Last session: 2026-01-16
Stopped at: Phase 18 plan 03 completed
Resume file: .planning/ROADMAP.md (check next phase)
