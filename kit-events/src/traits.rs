//! Core traits for the event system.

use crate::Error;
use async_trait::async_trait;
use std::any::Any;

/// Marker trait for events that can be dispatched.
///
/// Events are simple data structures that represent something that happened
/// in your application. They should be cheap to clone and contain all the
/// data needed by listeners.
///
/// # Example
///
/// ```rust
/// use kit_events::Event;
///
/// #[derive(Clone)]
/// struct OrderPlaced {
///     order_id: i64,
///     user_id: i64,
///     total: f64,
/// }
///
/// impl Event for OrderPlaced {
///     fn name(&self) -> &'static str {
///         "OrderPlaced"
///     }
/// }
/// ```
pub trait Event: Clone + Send + Sync + 'static {
    /// Returns the name of the event for logging and debugging.
    fn name(&self) -> &'static str;

    /// Returns the event as Any for type erasure.
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
}

/// A listener that handles events of type `E`.
///
/// Listeners contain the logic that should run when an event is dispatched.
/// They can be synchronous or asynchronous.
///
/// # Example
///
/// ```rust
/// use kit_events::{Event, Listener, Error, async_trait};
///
/// #[derive(Clone)]
/// struct UserRegistered { email: String }
///
/// impl Event for UserRegistered {
///     fn name(&self) -> &'static str { "UserRegistered" }
/// }
///
/// struct SendWelcomeEmail;
///
/// #[async_trait]
/// impl Listener<UserRegistered> for SendWelcomeEmail {
///     async fn handle(&self, event: &UserRegistered) -> Result<(), Error> {
///         println!("Welcome, {}!", event.email);
///         Ok(())
///     }
/// }
/// ```
#[async_trait]
pub trait Listener<E: Event>: Send + Sync + 'static {
    /// Handle the event.
    ///
    /// This method is called when the event is dispatched. It receives
    /// an immutable reference to the event data.
    async fn handle(&self, event: &E) -> Result<(), Error>;

    /// Returns the name of the listener for logging and debugging.
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Whether this listener should stop propagation to other listeners.
    ///
    /// If this returns `true` after handling, no further listeners will
    /// be called for this event.
    fn should_stop_propagation(&self) -> bool {
        false
    }
}

/// Marker trait for listeners that should be queued for background processing.
///
/// Listeners implementing this trait will not be executed immediately.
/// Instead, they will be pushed to a job queue and processed asynchronously
/// by a worker.
///
/// # Example
///
/// ```rust
/// use kit_events::{Event, Listener, ShouldQueue, Error, async_trait};
///
/// #[derive(Clone)]
/// struct LargeFileUploaded { path: String }
///
/// impl Event for LargeFileUploaded {
///     fn name(&self) -> &'static str { "LargeFileUploaded" }
/// }
///
/// struct ProcessUploadedFile;
///
/// impl ShouldQueue for ProcessUploadedFile {
///     fn queue(&self) -> &'static str {
///         "file-processing"
///     }
/// }
///
/// #[async_trait]
/// impl Listener<LargeFileUploaded> for ProcessUploadedFile {
///     async fn handle(&self, event: &LargeFileUploaded) -> Result<(), Error> {
///         // This will run in a background worker
///         println!("Processing file: {}", event.path);
///         Ok(())
///     }
/// }
/// ```
pub trait ShouldQueue {
    /// The queue name to dispatch this listener to.
    fn queue(&self) -> &'static str {
        "default"
    }

    /// The number of seconds to delay before processing.
    fn delay(&self) -> Option<u64> {
        None
    }

    /// The number of times to retry on failure.
    fn max_retries(&self) -> u32 {
        3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct TestEvent {
        message: String,
    }

    impl Event for TestEvent {
        fn name(&self) -> &'static str {
            "TestEvent"
        }
    }

    struct TestListener;

    #[async_trait]
    impl Listener<TestEvent> for TestListener {
        async fn handle(&self, event: &TestEvent) -> Result<(), Error> {
            assert_eq!(event.message, "hello");
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_listener_handle() {
        let listener = TestListener;
        let event = TestEvent {
            message: "hello".into(),
        };
        let result = listener.handle(&event).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_event_name() {
        let event = TestEvent {
            message: "test".into(),
        };
        assert_eq!(event.name(), "TestEvent");
    }
}
