# Roadmap: Cancer Framework DX Overhaul

## Overview

Transform the Cancer web framework from developer-centric to agent-first. The journey starts with reducing boilerplate in core patterns (handlers, models, validation), expands MCP introspection for agent comprehension, improves CLI scaffolding for feature-level generation, and concludes with polish that makes the entire framework self-documenting and agent-friendly.

## Domain Expertise

None

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Handler Simplification** - Reduce ceremony in handler definitions
- [x] **Phase 2: Model Boilerplate Reduction** - Derive more from models, write less
- [x] **Phase 3: Validation Syntax Streamlining** - More concise rule definitions
- [x] **Phase 4: Convention-over-Configuration** - Smart defaults for common patterns
- [x] **Phase 5: MCP Intent Understanding** - Agent comprehension of app purpose
- [x] **Phase 6: MCP Error Context** - Better diagnostic information for agents
- [x] **Phase 7: MCP Relationship Visibility** - Data flow and relationship introspection
- [x] **Phase 8: MCP Generation Hints** - Embedded hints in introspection responses
- [x] **Phase 9: CLI Feature Scaffolding** - Higher-level scaffolding (full features)
- [x] **Phase 10: CLI Smart Defaults** - Context-aware pattern detection
- [ ] **Phase 11: CLI Component Integration** - Better integration between generated parts
- [ ] **Phase 12: Agent-First Polish** - Self-documenting patterns and actionable errors

## Phase Details

### Phase 1: Handler Simplification
**Goal**: Reduce boilerplate in handler function definitions
**Depends on**: Nothing (first phase)
**Research**: Unlikely (existing Rust macro patterns)
**Plans**: 1 (Sample App Handler Modernization)

### Phase 2: Model Boilerplate Reduction
**Goal**: Derive more from SeaORM models, write less boilerplate
**Depends on**: Phase 1
**Research**: Unlikely (existing SeaORM patterns in codebase)
**Plans**: 1 (CancerModel Derive Macro Integration)

### Phase 3: Validation Syntax Streamlining
**Goal**: More concise validation rule definitions
**Depends on**: Phase 2
**Research**: Unlikely (internal validation patterns)
**Plans**: 1 (Validate Derive Macro)

### Phase 4: Convention-over-Configuration
**Goal**: Smart defaults reduce explicit configuration for common scenarios
**Depends on**: Phase 3
**Research**: Unlikely (pattern analysis of existing conventions)
**Plans**: TBD

### Phase 5: MCP Intent Understanding
**Goal**: Agent can understand full app intent, not just structure
**Depends on**: Phase 4
**Research**: Likely (new MCP capabilities)
**Research topics**: MCP protocol extensions, intent representation patterns, semantic understanding approaches
**Plans**: TBD

### Phase 6: MCP Error Context
**Goal**: Better error context for agent diagnosis
**Depends on**: Phase 5
**Research**: Unlikely (existing error handling patterns)
**Plans**: 1 (Error Context Enhancement)

### Phase 7: MCP Relationship Visibility
**Goal**: Relationship and data flow visibility through MCP
**Depends on**: Phase 6
**Research**: Unlikely (straightforward graph representation using existing patterns)
**Plans**: 1 (Relationship Visibility Tools)

### Phase 8: MCP Generation Hints
**Goal**: Generation hints embedded in introspection responses
**Depends on**: Phase 7
**Research**: Likely (agent generation patterns)
**Research topics**: Code generation hint formats, agent-friendly metadata, generation context requirements
**Plans**: TBD

### Phase 9: CLI Feature Scaffolding
**Goal**: Higher-level scaffolding that generates full features, not just files
**Depends on**: Phase 8
**Research**: Likely (scaffolding patterns)
**Research topics**: Feature scaffolding approaches, multi-file generation coordination, template systems
**Plans**: TBD

### Phase 10: CLI Smart Defaults
**Goal**: Smarter defaults based on existing codebase patterns
**Depends on**: Phase 9
**Research**: Unlikely (codebase analysis patterns)
**Plans**: TBD

### Phase 11: CLI Component Integration
**Goal**: Better integration between generated components
**Depends on**: Phase 10
**Research**: Unlikely (existing CLI patterns)
**Plans**: 3 (FK Detection & Test-Factory, FK-Aware Generation, Controllers & Pages)

### Phase 12: Agent-First Polish
**Goal**: Clear, predictable patterns agents can learn; self-documenting code; actionable errors
**Depends on**: Phase 11
**Research**: Unlikely (consolidation of earlier work)
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10 → 11 → 12

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
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
| 11. CLI Component Integration | 0/3 | Planned | - |
| 12. Agent-First Polish | 0/TBD | Not started | - |
