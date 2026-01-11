pub mod app;
pub mod auth;
pub mod cache;
pub mod config;
pub mod container;
pub mod csrf;
pub mod database;
pub mod error;
pub mod hashing;
pub mod http;
pub mod inertia;
pub mod middleware;
pub mod routing;
pub mod schedule;
pub mod server;
pub mod session;
pub mod testing;
pub mod validation;

pub use app::Application;
pub use auth::{Auth, AuthMiddleware, Authenticatable, GuestMiddleware, UserProvider};
pub use cache::{Cache, CacheConfig, CacheStore, InMemoryCache, RedisCache};
pub use config::{env, env_optional, env_required, AppConfig, Config, Environment, ServerConfig};
pub use container::{App, Container};
pub use csrf::{csrf_field, csrf_meta_tag, csrf_token, CsrfMiddleware};
pub use database::{
    AutoRouteBinding, Database, DatabaseConfig, DatabaseType, DbConnection, Model, ModelMut,
    RouteBinding, DB,
};
pub use error::{AppError, FrameworkError, HttpError, ValidationErrors};
pub use hashing::{hash, needs_rehash, verify, DEFAULT_COST as HASH_DEFAULT_COST};
pub use http::{
    json, text, Cookie, CookieOptions, FormRequest, FromParam, FromRequest, HttpResponse, Redirect,
    Request, Response, ResponseExt, SameSite,
};
pub use inertia::{Inertia, InertiaConfig, InertiaResponse, InertiaShared};
pub use session::{
    session, session_mut, SessionConfig, SessionData, SessionMiddleware, SessionStore,
};
// Deprecated - kept for backward compatibility
#[allow(deprecated)]
pub use inertia::InertiaContext;
pub use middleware::{
    register_global_middleware, Middleware, MiddlewareFuture, MiddlewareRegistry, Next,
    RateLimitConfig, RateLimiter, RateLimiters, Throttle,
};
pub use routing::{
    // Internal functions used by macros (hidden from docs)
    __delete_impl,
    __fallback_impl,
    __get_impl,
    __post_impl,
    __put_impl,
    route,
    validate_route_path,
    FallbackDefBuilder,
    GroupBuilder,
    GroupDef,
    GroupItem,
    GroupRoute,
    GroupRouter,
    IntoGroupItem,
    RouteBuilder,
    RouteDefBuilder,
    Router,
};
pub use schedule::{CronExpression, DayOfWeek, Schedule, Task, TaskBuilder, TaskEntry, TaskResult};
pub use server::Server;

// Re-export cancer-events for event-driven architecture
pub use cancer_events::{
    dispatch as dispatch_event, dispatch_sync, Error as EventError, Event, EventDispatcher,
    Listener, ShouldQueue,
};

// Re-export cancer-queue for background job processing
pub use cancer_queue::{
    dispatch as queue_dispatch, dispatch_later, dispatch_to, Error as QueueError, Job, JobPayload,
    PendingDispatch, Queue, QueueConfig, QueueConnection, Queueable, Worker, WorkerConfig,
};

// Re-export cancer-notifications for multi-channel notifications
pub use cancer_notifications::{
    Channel as NotificationChannel, ChannelResult, DatabaseMessage, DatabaseNotificationStore,
    Error as NotificationError, MailConfig, MailMessage, Notifiable, Notification,
    NotificationConfig, NotificationDispatcher, SlackAttachment, SlackField, SlackMessage,
    StoredNotification,
};

// Re-export cancer-broadcast for real-time WebSocket channels
pub use cancer_broadcast::{
    AuthData, Broadcast, BroadcastBuilder, BroadcastMessage, Broadcaster, ChannelAuthorizer,
    ChannelInfo, ChannelType, Client as BroadcastClient, ClientMessage, Error as BroadcastError,
    PresenceMember, ServerMessage,
};

// Re-export cancer-storage for file storage abstraction
pub use cancer_storage::{
    Disk, DiskConfig, DiskDriver, Error as StorageError, FileMetadata, LocalDriver,
    MemoryDriver as StorageMemoryDriver, PutOptions, Storage, StorageDriver, Visibility,
};

// Re-export cancer-cache for caching with tags
pub use cancer_cache::{
    Cache as TaggableCache, CacheConfig as TaggableCacheConfig, CacheStore as TaggableCacheStore,
    Error as TaggableCacheError, MemoryStore as TaggableCacheMemoryStore, TaggedCache,
};

// Re-export async_trait for middleware implementations
pub use async_trait::async_trait;

// Re-export inventory for #[service(ConcreteType)] macro
#[doc(hidden)]
pub use inventory;

// Re-export for macro usage
#[doc(hidden)]
pub use serde_json;

// Re-export serde for InertiaProps derive macro
pub use serde;

// Re-export validator crate for derive-based validation
pub use validator;
pub use validator::Validate;

// Re-export our Laravel-style validation module
pub use validation::{
    // Rules
    accepted,
    alpha,
    alpha_dash,
    alpha_num,
    array,
    between,
    boolean,
    confirmed,
    date,
    different,
    email,
    in_array,
    integer,
    max,
    min,
    not_in,
    nullable,
    numeric,
    regex,
    required,
    required_if,
    same,
    string,
    url,
    validate,
    Rule,
    ValidationError,
    Validator,
};

// Re-export the proc-macros for compile-time component validation and type safety
pub use cancer_macros::cancer_test;
pub use cancer_macros::domain_error;
pub use cancer_macros::handler;
pub use cancer_macros::inertia_response;
pub use cancer_macros::injectable;
pub use cancer_macros::redirect;
pub use cancer_macros::request;
pub use cancer_macros::service;
pub use cancer_macros::FormRequest as FormRequestDerive;
pub use cancer_macros::InertiaProps;

// Re-export Jest-like testing macros
pub use cancer_macros::describe;
pub use cancer_macros::test;

// Re-export testing utilities
pub use testing::{
    Factory, FactoryBuilder, Fake, Sequence, TestClient, TestContainer, TestContainerGuard,
    TestDatabase, TestRequestBuilder, TestResponse,
};

#[macro_export]
macro_rules! json_response {
    ($($json:tt)+) => {
        Ok($crate::HttpResponse::json($crate::serde_json::json!($($json)+)))
    };
}

#[macro_export]
macro_rules! text_response {
    ($text:expr) => {
        Ok($crate::HttpResponse::text($text))
    };
}

/// Register global middleware that runs on every request
///
/// Global middleware is registered in `bootstrap.rs` and runs in registration order,
/// before any route-specific middleware.
///
/// # Example
///
/// ```rust,ignore
/// // In bootstrap.rs
/// use cancer_rs::global_middleware;
/// use cancer_rs::middleware;
///
/// pub fn register() {
///     global_middleware!(middleware::LoggingMiddleware);
///     global_middleware!(middleware::CorsMiddleware);
/// }
/// ```
#[macro_export]
macro_rules! global_middleware {
    ($middleware:expr) => {
        $crate::register_global_middleware($middleware)
    };
}

/// Create an expectation for fluent assertions
///
/// # Example
///
/// ```rust,ignore
/// use cancer_rs::expect;
///
/// expect!(actual).to_equal(expected);
/// expect!(result).to_be_ok();
/// expect!(vec).to_have_length(3);
/// ```
///
/// On failure, shows clear output:
/// ```text
/// Test: "returns all todos"
///   at src/actions/todo_action.rs:25
///
///   expect!(actual).to_equal(expected)
///
///   Expected: 0
///   Received: 3
/// ```
#[macro_export]
macro_rules! expect {
    ($value:expr) => {
        $crate::testing::Expect::new($value, concat!(file!(), ":", line!()))
    };
}
