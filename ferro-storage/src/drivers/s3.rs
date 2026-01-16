//! S3 storage driver (placeholder).
//!
//! This module is only compiled when the `s3` feature is enabled.

use crate::storage::{FileMetadata, PutOptions, StorageDriver};
use crate::Error;
use async_trait::async_trait;
use bytes::Bytes;

/// S3-compatible storage driver.
pub struct S3Driver;

#[async_trait]
impl StorageDriver for S3Driver {
    async fn exists(&self, _path: &str) -> Result<bool, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn get(&self, _path: &str) -> Result<Bytes, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn put(&self, _path: &str, _contents: Bytes, _options: PutOptions) -> Result<(), Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn delete(&self, _path: &str) -> Result<(), Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn copy(&self, _from: &str, _to: &str) -> Result<(), Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn size(&self, _path: &str) -> Result<u64, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn metadata(&self, _path: &str) -> Result<FileMetadata, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn url(&self, _path: &str) -> Result<String, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn temporary_url(
        &self,
        _path: &str,
        _expiration: std::time::Duration,
    ) -> Result<String, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn files(&self, _directory: &str) -> Result<Vec<String>, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn all_files(&self, _directory: &str) -> Result<Vec<String>, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn directories(&self, _directory: &str) -> Result<Vec<String>, Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn make_directory(&self, _path: &str) -> Result<(), Error> {
        todo!("S3 driver not implemented yet")
    }

    async fn delete_directory(&self, _path: &str) -> Result<(), Error> {
        todo!("S3 driver not implemented yet")
    }
}
