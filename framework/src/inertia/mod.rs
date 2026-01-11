//! Inertia.js integration for Cancer framework.
//!
//! This module provides the integration between the framework-agnostic
//! `inertia-rs` crate and the Cancer framework's HTTP types.
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
pub use context::{Inertia, InertiaShared, SavedInertiaContext};
pub use response::InertiaResponse;

// Re-export core types from inertia-rs for advanced usage
pub use inertia_rs::{InertiaHttpResponse, InertiaRequest as InertiaRequestTrait};

// Deprecated exports for backward compatibility
#[allow(deprecated)]
pub use context::{InertiaContext, InertiaContextData};
