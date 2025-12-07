mod request;
mod response;

pub use request::Request;
pub use response::{HttpResponse, Redirect, RedirectRouteBuilder, Response, ResponseExt};

/// Error type for missing route parameters
#[derive(Debug)]
pub struct ParamError {
    pub param_name: String,
}

impl From<ParamError> for HttpResponse {
    fn from(err: ParamError) -> HttpResponse {
        HttpResponse::json(serde_json::json!({
            "error": format!("Missing required parameter: {}", err.param_name)
        }))
        .status(400)
    }
}

/// Create a text response
pub fn text(body: impl Into<String>) -> Response {
    Ok(HttpResponse::text(body))
}

/// Create a JSON response from a serde_json::Value
pub fn json(body: serde_json::Value) -> Response {
    Ok(HttpResponse::json(body))
}
