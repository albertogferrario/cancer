//! Application middleware
//!
//! Each middleware has its own dedicated file following the framework convention.

mod logging;

pub use logging::LoggingMiddleware;
