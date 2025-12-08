//! Route definition macros and helpers for Laravel-like routing syntax
//!
//! This module provides a clean, declarative way to define routes:
//!
//! ```rust,ignore
//! use kit::{routes, get, post, put, delete};
//!
//! routes! {
//!     get("/", controllers::home::index).name("home"),
//!     get("/users", controllers::user::index).name("users.index"),
//!     post("/users", controllers::user::store).name("users.store"),
//!     get("/protected", controllers::home::index).middleware(AuthMiddleware),
//! }
//! ```

use crate::http::{Request, Response};
use crate::middleware::{into_boxed, BoxedMiddleware, Middleware};
use crate::routing::router::Router;
use std::future::Future;

/// HTTP method for route definitions
#[derive(Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// Builder for route definitions that supports `.name()` and `.middleware()` chaining
pub struct RouteDefBuilder<H> {
    method: HttpMethod,
    path: &'static str,
    handler: H,
    name: Option<&'static str>,
    middlewares: Vec<BoxedMiddleware>,
}

impl<H, Fut> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    /// Create a new route definition builder
    pub fn new(method: HttpMethod, path: &'static str, handler: H) -> Self {
        Self {
            method,
            path,
            handler,
            name: None,
            middlewares: Vec::new(),
        }
    }

    /// Name this route for URL generation
    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    /// Add middleware to this route
    pub fn middleware<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(into_boxed(middleware));
        self
    }

    /// Register this route definition with a router
    pub fn register(self, router: Router) -> Router {
        // First, register the route based on method
        let builder = match self.method {
            HttpMethod::Get => router.get(self.path, self.handler),
            HttpMethod::Post => router.post(self.path, self.handler),
            HttpMethod::Put => router.put(self.path, self.handler),
            HttpMethod::Delete => router.delete(self.path, self.handler),
        };

        // Apply any middleware
        let builder = self
            .middlewares
            .into_iter()
            .fold(builder, |b, m| b.middleware_boxed(m));

        // Apply name if present, otherwise convert to Router
        if let Some(name) = self.name {
            builder.name(name)
        } else {
            builder.into()
        }
    }
}

/// Create a GET route definition
///
/// # Example
/// ```rust,ignore
/// get("/users", controllers::user::index).name("users.index")
/// ```
pub fn get<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Get, path, handler)
}

/// Create a POST route definition
///
/// # Example
/// ```rust,ignore
/// post("/users", controllers::user::store).name("users.store")
/// ```
pub fn post<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Post, path, handler)
}

/// Create a PUT route definition
///
/// # Example
/// ```rust,ignore
/// put("/users/{id}", controllers::user::update).name("users.update")
/// ```
pub fn put<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Put, path, handler)
}

/// Create a DELETE route definition
///
/// # Example
/// ```rust,ignore
/// delete("/users/{id}", controllers::user::destroy).name("users.destroy")
/// ```
pub fn delete<H, Fut>(path: &'static str, handler: H) -> RouteDefBuilder<H>
where
    H: Fn(Request) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Response> + Send + 'static,
{
    RouteDefBuilder::new(HttpMethod::Delete, path, handler)
}

/// Define routes with a clean, Laravel-like syntax
///
/// This macro generates a `pub fn register() -> Router` function automatically.
/// Place it at the top level of your `routes.rs` file.
///
/// # Example
/// ```rust,ignore
/// use kit::{routes, get, post, put, delete};
/// use crate::controllers;
/// use crate::middleware::AuthMiddleware;
///
/// routes! {
///     get("/", controllers::home::index).name("home"),
///     get("/users", controllers::user::index).name("users.index"),
///     get("/users/{id}", controllers::user::show).name("users.show"),
///     post("/users", controllers::user::store).name("users.store"),
///     put("/users/{id}", controllers::user::update).name("users.update"),
///     delete("/users/{id}", controllers::user::destroy).name("users.destroy"),
///     get("/protected", controllers::home::index).middleware(AuthMiddleware),
/// }
/// ```
#[macro_export]
macro_rules! routes {
    ( $( $route:expr ),* $(,)? ) => {
        pub fn register() -> $crate::Router {
            let mut router = $crate::Router::new();
            $(
                router = $route.register(router);
            )*
            router
        }
    };
}
