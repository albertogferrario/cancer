//! Framework-wide error types
//!
//! Provides a unified error type that can be used throughout the framework
//! and automatically converts to appropriate HTTP responses.

use std::fmt;

/// Framework-wide error type
///
/// This enum represents all possible errors that can occur in the framework.
/// It implements `From<FrameworkError> for Response` so errors can be propagated
/// using the `?` operator in controller handlers.
///
/// # Example
///
/// ```rust,ignore
/// use kit::{App, FrameworkError, Response};
///
/// pub async fn index(_req: Request) -> Response {
///     let service = App::resolve::<MyService>()?;  // Returns FrameworkError on failure
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub enum FrameworkError {
    /// Service not found in the dependency injection container
    ServiceNotFound {
        /// The type name of the service that was not found
        type_name: &'static str,
    },

    /// Parameter extraction failed (missing or invalid parameter)
    ParamError {
        /// The name of the parameter that failed extraction
        param_name: String,
    },

    /// Validation error
    ValidationError {
        /// The field that failed validation
        field: String,
        /// The validation error message
        message: String,
    },

    /// Database error
    DatabaseError {
        /// The error message
        message: String,
    },

    /// Generic internal server error
    Internal {
        /// The error message
        message: String,
    },
}

impl FrameworkError {
    /// Create a ServiceNotFound error for a given type
    pub fn service_not_found<T: ?Sized>() -> Self {
        Self::ServiceNotFound {
            type_name: std::any::type_name::<T>(),
        }
    }

    /// Create a ParamError for a missing parameter
    pub fn param(name: impl Into<String>) -> Self {
        Self::ParamError {
            param_name: name.into(),
        }
    }

    /// Create a ValidationError
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a DatabaseError
    pub fn database(message: impl Into<String>) -> Self {
        Self::DatabaseError {
            message: message.into(),
        }
    }

    /// Create an Internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            Self::ServiceNotFound { .. } => 500,
            Self::ParamError { .. } => 400,
            Self::ValidationError { .. } => 422,
            Self::DatabaseError { .. } => 500,
            Self::Internal { .. } => 500,
        }
    }
}

impl fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceNotFound { type_name } => {
                write!(f, "Service '{}' not registered in container", type_name)
            }
            Self::ParamError { param_name } => {
                write!(f, "Missing required parameter: {}", param_name)
            }
            Self::ValidationError { field, message } => {
                write!(f, "Validation error for '{}': {}", field, message)
            }
            Self::DatabaseError { message } => {
                write!(f, "Database error: {}", message)
            }
            Self::Internal { message } => {
                write!(f, "Internal server error: {}", message)
            }
        }
    }
}

impl std::error::Error for FrameworkError {}
