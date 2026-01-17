# Ferro Framework Issues Report

Issues discovered during the adotta-animali port. For Ferro framework developers.

---

## 1. Type Generator: Missing Imports from shared.ts

**Severity:** High
**Component:** `ferro generate-types`

### Problem

The `ferro generate-types` command generates `inertia-props.ts` but doesn't import types that are defined in `shared.ts`. This causes TypeScript compilation errors when components use types like `Value`, `UserProfile`, `DiscoverAnimal`, etc.

### Current Behavior

```typescript
// Generated inertia-props.ts - missing imports
export interface DiscoverProps {
  animals: DiscoverAnimal[];  // Error: DiscoverAnimal not found
}
```

### Expected Behavior

```typescript
import type {
  Value,
  UserProfile,
  DiscoverAnimal,
  // ... other shared types
} from './shared';

export interface DiscoverProps {
  animals: DiscoverAnimal[];  // Works
}
```

### Suggested Fix

The type generator should:
1. Analyze which types are referenced in generated interfaces
2. Check if those types exist in `shared.ts`
3. Generate appropriate import statements

---

## 2. Type Generator: Duplicate Properties in routes.ts

**Severity:** Medium
**Component:** `ferro generate-types`

### Problem

The generated `routes.ts` file contained duplicate controller entries, causing TypeScript errors.

### Example

```typescript
export const controllers = {
  dashboard: {
    index: (): RouteConfig => ({ url: '/dashboard', method: 'get' })
  },
  dashboard: {  // DUPLICATE - TypeScript error
    index: (): RouteConfig => ({ url: '/rifugio/dashboard', method: 'get' })
  },
  // ...
}
```

### Suggested Fix

1. Namespace controllers by their full path (e.g., `shelter_dashboard` vs `adopter_dashboard`)
2. Or group under parent namespaces: `controllers.shelter.dashboard`, `controllers.adopter.dashboard`

---

## 3. InertiaProps Naming Collisions

**Severity:** High
**Component:** `InertiaProps` derive macro / type generator

### Problem

Multiple controllers often have structs named `ShowProps`, `IndexProps`, `CreateProps`. When the type generator runs, these collide.

### Example

```rust
// In shelter/applications.rs
#[derive(InertiaProps)]
pub struct ShowProps { ... }

// In adopter/applications.rs
#[derive(InertiaProps)]
pub struct ShowProps { ... }  // Collision!
```

### Current Workaround

Manually rename to unique names: `ShelterApplicationDetailProps`, `AdopterApplicationDetailProps`

### Suggested Fix

1. Auto-namespace based on module path: `ShelterApplicationsShowProps`
2. Or require explicit `#[inertia(name = "ShelterApplicationDetail")]` attribute
3. Detect collisions at compile time and error with helpful message

---

## 4. No Props Contract Validation

**Severity:** Medium
**Component:** Framework tooling

### Problem

When backend handler props don't match frontend component expectations, the error only surfaces at runtime in the browser. This session's bug was caused by:
- Backend sending flat fields: `applicant_name`, `applicant_email`
- Frontend expecting nested: `application.user.name`, `application.user.email`

### Suggested Fix

Add a `ferro validate-contracts` command that:
1. Parses Rust `InertiaProps` structs
2. Parses TypeScript component prop types
3. Compares structure and reports mismatches
4. Runs as part of CI/pre-commit

The MCP server already has `validate_contracts` tool - consider exposing this as a CLI command.

---

## 5. Type Re-exports Not Generated

**Severity:** Medium
**Component:** `ferro generate-types`

### Problem

Components import types from `inertia-props.ts`, but many types are only defined in `shared.ts`. The generator doesn't create re-exports.

### Current Behavior

```typescript
// Component imports
import type { ShelterApplicationDetailProps } from '../types/inertia-props';
// Error: not exported from inertia-props.ts, only in shared.ts
```

### Expected Behavior

`inertia-props.ts` should re-export relevant types:

```typescript
export type {
  User,
  Animal,
  Shelter,
  ShelterApplicationDetailProps,
  // ... types components commonly need
} from './shared';
```

### Suggested Fix

Generate re-export statements for all types that components might need, or consolidate to a single types entry point.

---

## 6. Inconsistent Date/Time Field Handling

**Severity:** Low
**Component:** Model generation / SeaORM integration

### Problem

Date/time fields are stored as `String` in models rather than proper datetime types. This leads to:
- Manual parsing in handlers
- Potential format inconsistencies
- Runtime "Invalid time value" errors in frontend

### Example

```rust
// Current - all strings
pub created_at: String,
pub updated_at: String,
pub home_check_scheduled: Option<String>,

// Better - typed
pub created_at: DateTime<Utc>,
pub updated_at: DateTime<Utc>,
pub home_check_scheduled: Option<DateTime<Utc>>,
```

### Suggested Fix

1. Use `chrono::DateTime` or `time` crate types in models
2. Configure SeaORM to handle SQLite datetime columns properly
3. Serialize to ISO8601 strings automatically for JSON

---

## 7. Missing Animal Images Relationship

**Severity:** Low
**Component:** Application-specific, but reveals pattern gap

### Problem

The `show` handler has a TODO for loading animal images:

```rust
primary_image_url: None, // TODO: Load from animal_images table
```

Loading related data (images, tags, etc.) requires manual joins. A Laravel-like eager loading pattern would help.

### Suggested Feature

```rust
// Desired API
let animal = AnimalEntity::find_by_id(id)
    .with("images")  // Eager load relation
    .one(conn)
    .await?;

animal.images  // Vec<AnimalImage> already loaded
```

---

## Summary

| Issue | Severity | Quick Fix Available |
|-------|----------|---------------------|
| Missing shared.ts imports | High | Yes - parse and generate imports |
| Duplicate routes.ts entries | Medium | Yes - namespace by path |
| Props naming collisions | High | Medium - needs macro changes |
| No contract validation CLI | Medium | Yes - expose MCP tool as CLI |
| Missing type re-exports | Medium | Yes - generate re-exports |
| String datetime fields | Low | Medium - SeaORM config |
| No eager loading | Low | No - significant feature |

---

*Report generated during adotta-animali v1.0 development*
