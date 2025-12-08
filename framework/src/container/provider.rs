//! Service auto-registration for Kit framework
//!
//! This module provides automatic service registration via the `#[service(ConcreteType)]` macro.
//! Services marked with this macro are automatically registered at application startup.
//!
//! # Example
//!
//! ```rust,ignore
//! use kit::service;
//!
//! // Auto-register: dyn CacheStore → RedisCache
//! #[service(RedisCache)]
//! pub trait CacheStore: Send + Sync + 'static {
//!     fn get(&self, key: &str) -> Option<String>;
//!     fn set(&self, key: &str, value: &str);
//! }
//!
//! pub struct RedisCache;
//! impl Default for RedisCache {
//!     fn default() -> Self { Self }
//! }
//! impl CacheStore for RedisCache { ... }
//! ```

/// Entry for inventory-collected service bindings
///
/// Used internally by the `#[service(ConcreteType)]` macro to register
/// service bindings at compile time.
pub struct ServiceBindingEntry {
    /// Function to register the service binding
    pub register: fn(),
    /// Service name for debugging/logging
    pub name: &'static str,
}

// Inventory collection for auto-registered service bindings
inventory::collect!(ServiceBindingEntry);

/// Register all service bindings from inventory
///
/// This is called automatically by `Server::from_config()`.
/// It registers all services marked with `#[service(ConcreteType)]`.
pub fn register_service_bindings() {
    for entry in inventory::iter::<ServiceBindingEntry> {
        (entry.register)();
    }
}

/// Full bootstrap sequence for services
///
/// Called automatically by `Server::from_config()`.
pub fn bootstrap() {
    register_service_bindings();
}
