//! Authentication module for Kit framework
//!
//! Provides Laravel-like authentication with guards and middleware.
//!
//! # Overview
//!
//! Kit provides a simple, session-based authentication system:
//!
//! - `Auth` facade for login/logout operations
//! - `AuthMiddleware` for protecting routes
//! - `GuestMiddleware` for guest-only routes
//!
//! # Example
//!
//! ```rust,ignore
//! use kit::{Auth, AuthMiddleware, GuestMiddleware};
//!
//! // In a controller
//! if Auth::check() {
//!     let user_id = Auth::id().unwrap();
//! }
//!
//! // Login
//! Auth::login(user.id);
//!
//! // Logout
//! Auth::logout();
//!
//! // In routes
//! group!("/dashboard")
//!     .middleware(AuthMiddleware::redirect_to("/login"))
//!     .routes([...]);
//!
//! group!("/")
//!     .middleware(GuestMiddleware::redirect_to("/dashboard"))
//!     .routes([
//!         get!("/login", auth::show_login),
//!     ]);
//! ```

pub mod guard;
pub mod middleware;

pub use guard::Auth;
pub use middleware::{AuthMiddleware, GuestMiddleware};
