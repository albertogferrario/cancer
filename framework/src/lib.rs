pub mod http;
pub mod routing;
pub mod server;

pub use http::{json, text, HttpResponse, Request, Response};
pub use routing::Router;
pub use server::Server;

// Re-export for macro usage
#[doc(hidden)]
pub use serde_json;

/// Creates a JSON response directly from a JSON literal.
/// Returns Ok(HttpResponse) for use as Response type.
///
/// # Examples
/// ```ignore
/// json_response!({
///     "users": [{"id": 1, "name": "John"}]
/// })
///
/// // With status code (chain on the Result)
/// json_response!({"error": "Not found"}).map(|r| r.status(404))
/// ```
#[macro_export]
macro_rules! json_response {
    ($($json:tt)+) => {
        Ok($crate::HttpResponse::json($crate::serde_json::json!($($json)+)))
    };
}

/// Creates a text response directly.
/// Returns Ok(HttpResponse) for use as Response type.
#[macro_export]
macro_rules! text_response {
    ($text:expr) => {
        Ok($crate::HttpResponse::text($text))
    };
}
