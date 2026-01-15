# Technical Concerns

**Analysis Date:** 2026-01-15

## Error Handling Risks

**Unhandled Panics:**

| File | Pattern | Risk | Recommendation |
|------|---------|------|----------------|
| `app/src/main.rs` | Multiple `.expect()` calls | Panic on startup failure | Convert to Result propagation |
| `app/src/bootstrap.rs` | `.expect()` calls | Panic during bootstrap | Use `?` with context |
| `framework/src/schedule/expression.rs` | `.unwrap()` calls | Panic on cron parse | Return Result<T, ParseError> |

**Specific Locations:**
```
app/src/main.rs:15-30 - Service initialization expects
app/src/bootstrap.rs:45-80 - Database and cache setup expects
framework/src/schedule/expression.rs:100-150 - Cron parsing unwraps
```

**Impact:** Application crashes instead of graceful error handling

## Missing Configuration

**Environment File:**
- `.env.example` file not present
- Developers must guess required environment variables
- Risk: Misconfiguration in deployment

**Required Variables (undocumented):**
- `DATABASE_URL` - Required
- `REDIS_URL` - Optional but affects functionality
- `MAIL_*` - Mail configuration
- `APP_URL` - Application URL
- `CANCER_DEBUG_ENDPOINTS` - Debug routes toggle

**Recommendation:** Create `.env.example` with all variables and defaults

## Incomplete Features

**TODO Items:**

| Location | Description | Priority |
|----------|-------------|----------|
| `app/src/middleware/share_inertia.rs` | Incomplete Inertia data sharing | Medium |
| `framework/src/testing/http.rs:52` | Router integration for test client | High |

**Pattern:** Search codebase with `// TODO:`

## Code Quality Concerns

**Large Files:**

| File | Lines | Concern |
|------|-------|---------|
| `cancer-cli/src/templates/mod.rs` | 2,713 | Large, consider splitting |
| `framework/src/testing/factory.rs` | 1,274 | Complex test factories |
| `framework/src/testing/http.rs` | 739 | Could be modularized |

**Recommendation:** Split large files into focused modules

**Documentation Gaps:**
- `cancer-cli/src/templates/mod.rs` - Minimal documentation for template system
- Internal framework modules lack comprehensive doc comments

## Security Considerations

**Session Security:**
- Session storage in database or memory
- No documented session rotation on auth events
- Cookie security depends on configuration

**Input Validation:**
- Validation framework exists and is comprehensive
- Ensure all user input paths use validation

**Dependency Security:**
- Regular `cargo audit` recommended
- Pin dependency versions for reproducibility

## Performance Considerations

**Database:**
- No connection pool size optimization documented
- `DB_MAX_CONNECTIONS` default may not suit production

**Caching:**
- Redis connection pooling via `deadpool-redis`
- Consider cache warming strategies for frequently accessed data

**Async Operations:**
- Proper use of `tokio::spawn` for background tasks
- Event dispatcher uses async handlers

## Testing Gaps

**Coverage Areas:**
- No E2E test suite (HTTP test client has TODO for router integration)
- Integration tests limited to utility functions
- No load/stress testing framework

**Test Infrastructure:**
- `framework/src/testing/` provides good foundation
- Missing: Database fixtures, factory patterns beyond basics

## Technical Debt Tracking

**Priority Matrix:**

| Item | Effort | Impact | Priority |
|------|--------|--------|----------|
| Add `.env.example` | Low | High | P1 |
| Replace `.expect()` with Results | Medium | High | P1 |
| Split large template file | Medium | Medium | P2 |
| Complete HTTP test client | High | Medium | P2 |
| Document all public APIs | High | Medium | P3 |

## Monitoring Recommendations

**Add:**
- Structured error logging with context
- Request tracing across service boundaries
- Database query performance metrics
- Cache hit/miss ratios

**Current State:**
- Basic tracing infrastructure exists
- Debug endpoints at `/_cancer/metrics`
- No external monitoring integration

---

*Concern analysis: 2026-01-15*
*Review quarterly and update priorities*
