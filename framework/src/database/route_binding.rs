//! Route model binding support
//!
//! Provides automatic model resolution from route parameters.
//!
//! # Example
//!
//! ```rust,ignore
//! use kit::{handler, json_response, Response, route_binding};
//! use crate::models::user;
//!
//! // Enable route binding for the model
//! route_binding!(user::Entity, user::Model, "user");
//!
//! // Use in handlers - user is automatically fetched
//! #[handler]
//! pub async fn show(user: user::Model) -> Response {
//!     json_response!({ "name": user.name })
//! }
//! ```

use crate::error::FrameworkError;
use async_trait::async_trait;

/// Trait for models that can be automatically resolved from route parameters
///
/// Implement this trait on your SeaORM Model types to enable automatic
/// route model binding in handlers. When a route parameter matches the
/// `param_name()`, the model will be automatically fetched from the database.
///
/// If the model is not found, a 404 Not Found response is returned.
///
/// # Example
///
/// ```rust,ignore
/// use kit::database::RouteBinding;
/// use kit::FrameworkError;
///
/// #[async_trait]
/// impl RouteBinding for user::Model {
///     fn param_name() -> &'static str {
///         "user"  // matches {user} in route like /users/{user}
///     }
///
///     async fn from_route_param(value: &str) -> Result<Self, FrameworkError> {
///         let id: i32 = value.parse()
///             .map_err(|_| FrameworkError::param_parse(value, "i32"))?;
///
///         user::Entity::find_by_pk(id)
///             .await?
///             .ok_or_else(|| FrameworkError::model_not_found("User"))
///     }
/// }
/// ```
#[async_trait]
pub trait RouteBinding: Sized + Send {
    /// The route parameter name to bind from
    ///
    /// This should match the parameter placeholder in your route definition.
    /// For example, if your route is `/users/{user}`, this should return `"user"`.
    fn param_name() -> &'static str;

    /// Fetch the model from the database using the route parameter value
    ///
    /// This method is called automatically by the `#[handler]` macro when
    /// a parameter of this type is declared in the handler function.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` - The model was found
    /// - `Err(FrameworkError::ModelNotFound)` - Model not found (returns 404)
    /// - `Err(FrameworkError::ParamParse)` - Parameter could not be parsed (returns 400)
    async fn from_route_param(value: &str) -> Result<Self, FrameworkError>;
}

/// Convenience macro to implement RouteBinding for a SeaORM model
///
/// This macro implements the `RouteBinding` trait for a model, enabling
/// automatic route model binding with 404 handling.
///
/// # Arguments
///
/// - `$entity` - The SeaORM Entity type (e.g., `user::Entity`)
/// - `$model` - The SeaORM Model type (e.g., `user::Model`)
/// - `$param` - The route parameter name (e.g., `"user"`)
///
/// # Example
///
/// ```rust,ignore
/// use kit::route_binding;
///
/// // In your model file (e.g., models/user.rs)
/// route_binding!(Entity, Model, "user");
///
/// // Now you can use automatic binding in handlers:
/// #[handler]
/// pub async fn show(user: user::Model) -> Response {
///     json_response!({ "id": user.id, "name": user.name })
/// }
/// ```
///
/// # Route Definition
///
/// The parameter name must match your route definition:
///
/// ```rust,ignore
/// routes! {
///     get("/users/{user}", controllers::user::show),
/// }
/// ```
#[macro_export]
macro_rules! route_binding {
    ($entity:ty, $model:ty, $param:literal) => {
        #[async_trait::async_trait]
        impl $crate::RouteBinding for $model {
            fn param_name() -> &'static str {
                $param
            }

            async fn from_route_param(value: &str) -> Result<Self, $crate::FrameworkError> {
                let id: i32 = value
                    .parse()
                    .map_err(|_| $crate::FrameworkError::param_parse(value, "i32"))?;

                <$entity as $crate::Model>::find_by_pk(id)
                    .await?
                    .ok_or_else(|| $crate::FrameworkError::model_not_found(stringify!($model)))
            }
        }
    };
}
