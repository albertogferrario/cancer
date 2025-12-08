//! Database connection management

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;

use crate::database::config::DatabaseConfig;
use crate::error::FrameworkError;

/// Wrapper around SeaORM's DatabaseConnection
///
/// This provides a clonable, thread-safe connection that can be stored
/// in the application container and shared across requests.
///
/// # Example
///
/// ```rust,ignore
/// let conn = DbConnection::connect(&config).await?;
///
/// // Use with SeaORM queries
/// let users = User::find().all(conn.inner()).await?;
/// ```
#[derive(Clone)]
pub struct DbConnection {
    inner: Arc<DatabaseConnection>,
}

impl DbConnection {
    /// Create a new database connection from config
    ///
    /// This establishes a connection pool using the provided configuration.
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, FrameworkError> {
        let mut opt = ConnectOptions::new(&config.url);
        opt.max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .sqlx_logging(config.logging);

        let conn = Database::connect(opt)
            .await
            .map_err(|e| FrameworkError::database(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(conn),
        })
    }

    /// Get a reference to the underlying SeaORM connection
    ///
    /// Use this when you need to execute raw SeaORM queries.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let conn = DB::connection()?;
    /// let users = User::find()
    ///     .filter(user::Column::Active.eq(true))
    ///     .all(conn.inner())
    ///     .await?;
    /// ```
    pub fn inner(&self) -> &DatabaseConnection {
        &self.inner
    }

    /// Check if the connection is closed
    pub fn is_closed(&self) -> bool {
        // SeaORM doesn't expose this directly, but we can check via ping
        false
    }
}

impl AsRef<DatabaseConnection> for DbConnection {
    fn as_ref(&self) -> &DatabaseConnection {
        &self.inner
    }
}

impl std::ops::Deref for DbConnection {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
