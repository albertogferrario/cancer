# Ferro Framework - Issues Report

**Date:** 2026-01-17
**Reporter:** Adotta Animali development session
**Framework Version:** 0.1.67

---

## Critical Issues

### 1. ✅ FIXED: Redis Connection Hangs Indefinitely

**Status:** Fixed in commit `3a8cccb`

**Problem:** When Redis is unavailable, `ConnectionManager::new()` hangs forever with no timeout, preventing the application from starting.

**Impact:** Applications cannot start when Redis is not running, even though the framework has in-memory fallback.

**Fix Applied:** Added 2-second timeout to Redis connection in `framework/src/cache/redis.rs`:
```rust
let conn = tokio::time::timeout(
    Duration::from_secs(2),
    ConnectionManager::new(client),
)
.await
.map_err(|_| FrameworkError::internal("Redis connection timeout".to_string()))?
```

---

## Branding Issues (Cancer → Ferro rename incomplete)

### 2. Server startup message still says "Cancer"

**File:** `framework/src/server.rs:90`
```rust
println!("Cancer server running on http://{}", addr);
```
**Should be:** `"Ferro server running on http://{}"`

### 3. Debug endpoints use `/_cancer/` prefix

**File:** `framework/src/server.rs:125-135`
- `/_cancer/health`
- `/_cancer/routes`
- `/_cancer/middleware`
- `/_cancer/services`
- `/_cancer/metrics`
- `/_cancer/queue/jobs`
- `/_cancer/queue/stats`

**Should be:** `/_ferro/*`

### 4. Session cookie name defaults to "cancer_session"

**File:** `framework/src/session/config.rs:28, 64`
```rust
cookie_name: "cancer_session".to_string(),
```
**Should be:** `"ferro_session"`

### 5. Cache prefix defaults to "cancer_cache:"

**Files:**
- `framework/src/cache/config.rs:42`
- `framework/src/cache/memory.rs:49`

**Should be:** `"ferro_cache:"`

### 6. Documentation comments reference "Cancer framework"

**Files with outdated comments:**
- `framework/src/session/middleware.rs:1`
- `framework/src/session/mod.rs:1`
- `framework/src/hashing/mod.rs:1`
- `framework/src/seeder/mod.rs:1`
- `framework/src/database/model.rs:1, 61`
- `framework/src/validation/mod.rs:1`
- `framework/src/schedule/mod.rs:1, 71`
- `framework/src/cache/config.rs:1` (implied)

### 7. Test macro still named `cancer_test`

**File:** `framework/src/lib.rs:182`
```rust
pub use ferro_macros::cancer_test;
```
**Should be:** `ferro_test`

### 8. `ferro db:sync` generates `CancerModel` derive

**Problem:** Running `ferro db:sync` generates entity files with `#[derive(CancerModel)]` instead of `#[derive(FerroModel)]`.

**Impact:** Projects scaffolded from templates fail to compile until manually fixed:
```bash
sed -i '' 's/CancerModel/FerroModel/g' src/models/entities/*.rs
```

**Fix needed:** Update CLI templates to use `FerroModel`.

---

## Suggested Improvements

### 8. Add CACHE_DRIVER environment variable

**Problem:** No way to explicitly disable Redis cache attempts. The only way to use in-memory cache is to let Redis connection fail.

**Suggestion:** Add `CACHE_DRIVER` env var with options:
- `redis` (default) - Try Redis, fallback to memory
- `memory` - Use in-memory only, skip Redis attempt
- `none` - Disable caching entirely

### 9. Add connection timeout configuration

**Problem:** The 2-second Redis timeout is hardcoded.

**Suggestion:** Add `REDIS_TIMEOUT` env var to configure connection timeout.

---

## Files Requiring Updates

Run this to find all "Cancer/cancer" references:
```bash
grep -r -i "cancer" framework/src --include="*.rs" | wc -l
# Result: ~50+ occurrences
```

### Quick fix script:
```bash
# Backup first!
find framework/src -name "*.rs" -exec sed -i '' \
  -e 's/Cancer server/Ferro server/g' \
  -e 's/Cancer framework/Ferro framework/g' \
  -e 's/Cancer ORM/Ferro ORM/g' \
  -e 's/cancer_session/ferro_session/g' \
  -e 's/cancer_cache/ferro_cache/g' \
  -e 's/_cancer\//_ferro\//g' \
  -e 's/cancer_test/ferro_test/g' \
  {} \;
```

---

## Priority

| Issue | Priority | Effort |
|-------|----------|--------|
| Redis timeout (FIXED) | Critical | Done |
| Branding rename | High | Low (sed script) |
| CACHE_DRIVER env | Medium | Medium |
| Timeout configuration | Low | Low |
