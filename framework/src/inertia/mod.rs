//! Inertia.js integration for Cancer framework
//!
//! This module provides async-safe Inertia.js integration with:
//! - `Inertia::render()` - Main response rendering
//! - `InertiaShared` - Shared props via middleware
//! - `InertiaConfig` - Configuration options
//! - Automatic partial reload filtering
//!
//! # Example
//!
//! ```rust,ignore
//! use cancer::{Inertia, Request, Response, InertiaProps};
//!
//! #[derive(InertiaProps)]
//! pub struct HomeProps {
//!     pub title: String,
//! }
//!
//! pub async fn index(req: Request) -> Response {
//!     Inertia::render(&req, "Home", HomeProps {
//!         title: "Welcome".into(),
//!     })
//! }
//! ```

mod config;
mod context;
mod response;

pub use config::InertiaConfig;
pub use context::{Inertia, InertiaShared};
pub use response::InertiaResponse;

// Deprecated exports for backward compatibility
#[allow(deprecated)]
pub use context::{InertiaContext, InertiaContextData};
