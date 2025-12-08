//! Application Bootstrap
//!
//! This is where you manually register services that need runtime configuration.
//! Services that don't need runtime config can use `#[service(ConcreteType)]` instead.
//!
//! # Example
//!
//! ```rust,ignore
//! // For services with no runtime config, use the macro:
//! #[service(RedisCache)]
//! pub trait CacheStore { ... }
//!
//! // For services needing runtime config, register here:
//! pub fn register() {
//!     let db_url = std::env::var("DATABASE_URL").unwrap();
//!     bind!(dyn Database, PostgresDB::connect(&db_url));
//! }
//! ```

#[allow(unused_imports)]
use kit::{bind, singleton, App};

/// Register services that need runtime configuration
///
/// Called from main.rs before `Server::from_config()`.
/// Services registered here can use environment variables, config files, etc.
pub fn register() {
    // Example: Register a trait binding with runtime config
    // let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/app".to_string());
    // bind!(dyn Database, PostgresDB::connect(&db_url));

    // Example: Register a concrete singleton
    // singleton!(CacheService::new());

    // Add your service registrations here
}
