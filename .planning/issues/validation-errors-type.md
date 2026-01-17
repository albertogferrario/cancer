# Issue: ValidationErrors should have a dedicated type

## Summary

Validation errors in Inertia props use `serde_json::Value` which maps to `unknown` in TypeScript. This requires manual type casts in frontend code since validation errors always have the structure `Record<string, string[]>`.

## Current Behavior

In Rust controllers:
```rust
#[derive(InertiaProps)]
pub struct LoginProps {
    pub errors: Option<serde_json::Value>,
}
```

Generated TypeScript:
```typescript
export interface LoginProps {
  errors: unknown | null;
}
```

Frontend code requires cast:
```typescript
const [errors, setErrors] = useState<Record<string, string[]>>(
  (serverErrors as Record<string, string[]>) ?? {}
);
```

## Expected Behavior

Ferro should provide a `ValidationErrors` type that generates proper TypeScript:

```rust
use ferro::ValidationErrors;

#[derive(InertiaProps)]
pub struct LoginProps {
    pub errors: Option<ValidationErrors>,
}
```

Generated TypeScript:
```typescript
export interface LoginProps {
  errors: Record<string, string[]> | null;
}
```

## Why This Matters

1. Validation errors are a common pattern in web frameworks
2. The structure is always `field_name -> [error_messages]`
3. Avoids unsafe type casts in every form component
4. Better developer experience with proper autocomplete

## Proposed Solution

1. Add `ferro::ValidationErrors` type alias:
   ```rust
   pub type ValidationErrors = std::collections::HashMap<String, Vec<String>>;
   ```

2. Update type generator to recognize `ValidationErrors` and map to `Record<string, string[]>`

3. Update existing props to use the new type

## Files Affected

- `ferro/src/lib.rs` or `ferro/src/types.rs` - add ValidationErrors type
- `ferro-cli/src/commands/generate_types.rs` - recognize the type
- All controller props that use `Option<serde_json::Value>` for errors

## Priority

Medium - improves developer experience and type safety
