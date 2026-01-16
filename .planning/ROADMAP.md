# Roadmap: Ferro Framework

## Milestones

- ✅ **v1.0 DX Overhaul** - Phases 1-12 (completed 2026-01-16)
- 🚧 **v2.0 Rebrand** - Phases 13-22 (in progress)

---

## 🚧 v2.0 Rebrand (In Progress)

**Milestone Goal:** Rename the framework from "cancer" to "ferro" for crates.io publication and public release.

### Overview

Complete rebrand of the framework, including all crates, CLI tools, MCP server, documentation, and repository infrastructure. The name "ferro" (Italian for "iron") reflects the framework's Rust foundation while being appropriate for public distribution.

### Phases

- [ ] **Phase 13: Rebrand Audit** - Document all name occurrences and plan rename strategy
- [ ] **Phase 14: Core Framework Rename** - Rename main `cancer` crate to `ferro`
- [ ] **Phase 15: Supporting Crates Rename** - Rename all `cancer-*` crates to `ferro-*`
- [ ] **Phase 16: CLI Rebrand** - Rename `cancer-cli` to `ferro-cli` and update commands
- [ ] **Phase 17: MCP Server Rebrand** - Rename `cancer-mcp` to `ferro-mcp` and update tools
- [ ] **Phase 18: Documentation Update** - Update all docs, READMEs, and code comments
- [ ] **Phase 19: Sample App Migration** - Update sample app to use new names
- [ ] **Phase 20: Templates & Scaffolding** - Update all CLI templates with new names
- [ ] **Phase 21: Repository & CI** - Update repo name, CI/CD, badges, GitHub config
- [ ] **Phase 22: Publishing & Announcement** - Prepare for crates.io, migration guide

### Phase Details

#### Phase 13: Rebrand Audit
**Goal**: Document every occurrence of "cancer" across the codebase and plan rename strategy
**Depends on**: v1.0 complete
**Research**: Unlikely (grep and documentation)
**Plans**: TBD

#### Phase 14: Core Framework Rename
**Goal**: Rename the main `cancer` crate to `ferro` including Cargo.toml, lib.rs exports
**Depends on**: Phase 13
**Research**: Unlikely (Cargo workspace patterns)
**Plans**: TBD

#### Phase 15: Supporting Crates Rename
**Goal**: Rename all supporting crates: cancer-events→ferro-events, cancer-queue→ferro-queue, etc.
**Depends on**: Phase 14
**Research**: Unlikely (follow Phase 14 pattern)
**Plans**: TBD

#### Phase 16: CLI Rebrand
**Goal**: Rename cancer-cli to ferro-cli, update binary name, command references
**Depends on**: Phase 15
**Research**: Unlikely (CLI patterns established)
**Plans**: TBD

#### Phase 17: MCP Server Rebrand
**Goal**: Rename cancer-mcp to ferro-mcp, update tool prefixes and descriptions
**Depends on**: Phase 16
**Research**: Unlikely (MCP patterns established)
**Plans**: TBD

#### Phase 18: Documentation Update
**Goal**: Update all documentation, README files, and code comments
**Depends on**: Phase 17
**Research**: Unlikely (find and replace with review)
**Plans**: TBD

#### Phase 19: Sample App Migration
**Goal**: Update sample app (app crate) to use ferro imports and patterns
**Depends on**: Phase 18
**Research**: Unlikely (follow new patterns)
**Plans**: TBD

#### Phase 20: Templates & Scaffolding
**Goal**: Update all CLI templates to generate ferro-based code
**Depends on**: Phase 19
**Research**: Unlikely (template updates)
**Plans**: TBD

#### Phase 21: Repository & CI
**Goal**: Rename GitHub repository, update CI/CD pipelines, badges, URLs
**Depends on**: Phase 20
**Research**: Unlikely (GitHub/CI patterns)
**Plans**: TBD

#### Phase 22: Publishing & Announcement
**Goal**: Prepare crates.io metadata, write migration guide, plan announcement
**Depends on**: Phase 21
**Research**: Likely (crates.io publishing requirements, semver strategies)
**Research topics**: crates.io namespace policies, publishing workflow, migration guide best practices
**Plans**: TBD

---

## ✅ v1.0 DX Overhaul (Completed)

**Milestone Goal:** Transform the framework from developer-centric to agent-first.

### Overview

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
- [x] **Phase 11: CLI Component Integration** - Better integration between generated parts
- [x] **Phase 12: Agent-First Polish** - Self-documenting patterns and actionable errors

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
**Plans**: 5 (Actionable Errors, Self-Documenting Config, Tailwind Out-of-Box, Auto-Cleanup, Inertia Redirect Fix)

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → ... → 12 (v1.0) → 13 → ... → 22 (v2.0)

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. Handler Simplification | v1.0 | 1/1 | Complete | 2026-01-15 |
| 2. Model Boilerplate Reduction | v1.0 | 1/1 | Complete | 2026-01-15 |
| 3. Validation Syntax Streamlining | v1.0 | 1/1 | Complete | 2026-01-15 |
| 4. Convention-over-Configuration | v1.0 | 1/1 | Complete | 2026-01-15 |
| 5. MCP Intent Understanding | v1.0 | 1/1 | Complete | 2026-01-15 |
| 6. MCP Error Context | v1.0 | 1/1 | Complete | 2026-01-15 |
| 7. MCP Relationship Visibility | v1.0 | 1/1 | Complete | 2026-01-15 |
| 8. MCP Generation Hints | v1.0 | 1/1 | Complete | 2026-01-15 |
| 9. CLI Feature Scaffolding | v1.0 | 1/1 | Complete | 2026-01-15 |
| 10. CLI Smart Defaults | v1.0 | 1/1 | Complete | 2026-01-15 |
| 11. CLI Component Integration | v1.0 | 3/3 | Complete | 2026-01-15 |
| 12. Agent-First Polish | v1.0 | 5/5 | Complete | 2026-01-16 |
| 13. Rebrand Audit | v2.0 | 1/1 | Complete | 2026-01-16 |
| 14. Core Framework Rename | v2.0 | 1/1 | Complete | 2026-01-16 |
| 15. Supporting Crates Rename | v2.0 | 0/? | Not started | - |
| 16. CLI Rebrand | v2.0 | 0/? | Not started | - |
| 17. MCP Server Rebrand | v2.0 | 0/? | Not started | - |
| 18. Documentation Update | v2.0 | 0/? | Not started | - |
| 19. Sample App Migration | v2.0 | 0/? | Not started | - |
| 20. Templates & Scaffolding | v2.0 | 0/? | Not started | - |
| 21. Repository & CI | v2.0 | 0/? | Not started | - |
| 22. Publishing & Announcement | v2.0 | 0/? | Not started | - |
