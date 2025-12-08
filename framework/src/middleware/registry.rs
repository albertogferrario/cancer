//! Middleware registry for global middleware
//!
//! Configure global middleware using the `Server::middleware()` builder method.

use super::{into_boxed, BoxedMiddleware, Middleware};

/// Registry for global middleware that runs on every request
///
/// # Example
///
/// ```rust,ignore
/// Server::from_config(router)
///     .middleware(LoggingMiddleware)  // Global middleware
///     .middleware(CorsMiddleware)
///     .run()
///     .await;
/// ```
pub struct MiddlewareRegistry {
    /// Middleware that runs on every request (in order)
    global: Vec<BoxedMiddleware>,
}

impl MiddlewareRegistry {
    /// Create a new empty middleware registry
    pub fn new() -> Self {
        Self {
            global: Vec::new(),
        }
    }

    /// Append global middleware that runs on every request
    ///
    /// Global middleware runs in the order they are added, before any
    /// route-specific middleware.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// m.append(LoggingMiddleware)
    ///  .append(CorsMiddleware)
    /// ```
    pub fn append<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.global.push(into_boxed(middleware));
        self
    }

    /// Get the list of global middleware
    pub fn global_middleware(&self) -> &[BoxedMiddleware] {
        &self.global
    }
}

impl Default for MiddlewareRegistry {
    fn default() -> Self {
        Self::new()
    }
}
