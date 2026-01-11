//! S3 storage driver (placeholder).
//!
//! This module is only compiled when the `s3` feature is enabled.

use crate::{StorageDriver, StorageError};
use async_trait::async_trait;
use bytes::Bytes;

/// S3-compatible storage driver.
pub struct S3Driver;

#[async_trait]
impl StorageDriver for S3Driver {
    async fn get(&self, _path: &str) -> Result<Bytes, StorageError> {
        todo!("S3 driver not implemented yet")
    }

    async fn put(&self, _path: &str, _contents: Bytes) -> Result<(), StorageError> {
        todo!("S3 driver not implemented yet")
    }

    async fn delete(&self, _path: &str) -> Result<(), StorageError> {
        todo!("S3 driver not implemented yet")
    }

    async fn exists(&self, _path: &str) -> Result<bool, StorageError> {
        todo!("S3 driver not implemented yet")
    }

    fn url(&self, _path: &str) -> String {
        todo!("S3 driver not implemented yet")
    }
}
