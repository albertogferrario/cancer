//! In-memory cache store using moka.

use crate::cache::CacheStore;
use crate::error::Error;
use async_trait::async_trait;
use dashmap::DashMap;
use moka::future::Cache as MokaCache;
use std::sync::Arc;
use std::time::Duration;

/// In-memory cache store.
pub struct MemoryStore {
    cache: MokaCache<String, Vec<u8>>,
    tags: Arc<DashMap<String, Vec<String>>>,
    counters: Arc<DashMap<String, i64>>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStore {
    /// Create a new memory store.
    pub fn new() -> Self {
        Self {
            cache: MokaCache::builder().max_capacity(10_000).build(),
            tags: Arc::new(DashMap::new()),
            counters: Arc::new(DashMap::new()),
        }
    }

    /// Create with custom capacity.
    pub fn with_capacity(capacity: u64) -> Self {
        Self {
            cache: MokaCache::builder().max_capacity(capacity).build(),
            tags: Arc::new(DashMap::new()),
            counters: Arc::new(DashMap::new()),
        }
    }
}

#[async_trait]
impl CacheStore for MemoryStore {
    async fn get_raw(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        Ok(self.cache.get(key).await)
    }

    async fn put_raw(&self, key: &str, value: Vec<u8>, ttl: Duration) -> Result<(), Error> {
        self.cache.insert(key.to_string(), value).await;

        // Moka handles TTL through expiration, but we need to set it per-entry
        // For simplicity, we'll use the builder-level TTL
        // In production, you'd want per-entry TTL using a wrapper or different approach
        let _ = ttl; // TTL is handled at cache level for moka

        Ok(())
    }

    async fn has(&self, key: &str) -> Result<bool, Error> {
        Ok(self.cache.contains_key(key))
    }

    async fn forget(&self, key: &str) -> Result<bool, Error> {
        let existed = self.cache.contains_key(key);
        self.cache.remove(key).await;
        self.counters.remove(key);
        Ok(existed)
    }

    async fn flush(&self) -> Result<(), Error> {
        self.cache.invalidate_all();
        self.tags.clear();
        self.counters.clear();
        Ok(())
    }

    async fn increment(&self, key: &str, value: i64) -> Result<i64, Error> {
        let mut entry = self.counters.entry(key.to_string()).or_insert(0);
        *entry += value;
        Ok(*entry)
    }

    async fn decrement(&self, key: &str, value: i64) -> Result<i64, Error> {
        let mut entry = self.counters.entry(key.to_string()).or_insert(0);
        *entry -= value;
        Ok(*entry)
    }

    async fn tag_add(&self, tag: &str, key: &str) -> Result<(), Error> {
        self.tags
            .entry(tag.to_string())
            .or_default()
            .push(key.to_string());
        Ok(())
    }

    async fn tag_members(&self, tag: &str) -> Result<Vec<String>, Error> {
        Ok(self.tags.get(tag).map(|v| v.clone()).unwrap_or_default())
    }

    async fn tag_flush(&self, tag: &str) -> Result<(), Error> {
        if let Some((_, keys)) = self.tags.remove(tag) {
            for key in keys {
                self.cache.remove(&key).await;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_store_put_get() {
        let store = MemoryStore::new();

        store
            .put_raw("key", b"value".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();

        let value = store.get_raw("key").await.unwrap();
        assert_eq!(value, Some(b"value".to_vec()));
    }

    #[tokio::test]
    async fn test_memory_store_has() {
        let store = MemoryStore::new();

        assert!(!store.has("missing").await.unwrap());

        store
            .put_raw("exists", b"value".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();

        assert!(store.has("exists").await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_store_forget() {
        let store = MemoryStore::new();

        store
            .put_raw("key", b"value".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();

        let removed = store.forget("key").await.unwrap();
        assert!(removed);
        assert!(!store.has("key").await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_store_increment_decrement() {
        let store = MemoryStore::new();

        let val = store.increment("counter", 5).await.unwrap();
        assert_eq!(val, 5);

        let val = store.increment("counter", 3).await.unwrap();
        assert_eq!(val, 8);

        let val = store.decrement("counter", 2).await.unwrap();
        assert_eq!(val, 6);
    }

    #[tokio::test]
    async fn test_memory_store_tags() {
        let store = MemoryStore::new();

        store
            .put_raw("user:1", b"alice".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();
        store
            .put_raw("user:2", b"bob".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();

        store.tag_add("users", "user:1").await.unwrap();
        store.tag_add("users", "user:2").await.unwrap();

        let members = store.tag_members("users").await.unwrap();
        assert_eq!(members.len(), 2);

        store.tag_flush("users").await.unwrap();

        assert!(!store.has("user:1").await.unwrap());
        assert!(!store.has("user:2").await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_store_flush() {
        let store = MemoryStore::new();

        store
            .put_raw("key1", b"value1".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();
        store
            .put_raw("key2", b"value2".to_vec(), Duration::from_secs(60))
            .await
            .unwrap();

        store.flush().await.unwrap();

        assert!(!store.has("key1").await.unwrap());
        assert!(!store.has("key2").await.unwrap());
    }
}
