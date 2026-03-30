use std::time::Duration;

use crate::common::error::AppError;
use redis::{AsyncCommands, Client};

/// Redis cache configuration
#[derive(Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl: Duration,
    pub connection_pool_size: u32,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            ttl: Duration::from_secs(3600), // 1 hour
            connection_pool_size: 10,
        }
    }
}

impl CacheConfig {
    pub fn new(ttl: Duration, connection_pool_size: u32) -> Self {
        Self {
            enabled: true,
            ttl,
            connection_pool_size,
        }
    }

    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }
}

/// Redis cache client
#[derive(Clone)]
pub struct RedisCache {
    config: CacheConfig,
    url: Option<String>,
}

impl RedisCache {
    /// Create a new Redis cache client
    pub fn new(url: Option<String>, config: CacheConfig) -> Self {
        if let Some(ref u) = url {
            if Client::open(u.as_str()).is_ok() {
                tracing::info!("Redis connection pool initialized");
            } else {
                tracing::warn!("Failed to create Redis client. Caching disabled.");
                return Self { config, url: None };
            }
        }

        Self { config, url }
    }

    /// Create a new Redis cache client from URL
    pub fn from_url(url: &str, ttl_seconds: u64) -> Self {
        let config = CacheConfig::new(
            Duration::from_secs(ttl_seconds),
            10, // default connection pool size
        );
        let url_str = url.to_string();
        if Client::open(url_str.as_str()).is_ok() {
            tracing::info!("Redis connection pool initialized");
        } else {
            tracing::warn!("Failed to create Redis client. Caching disabled.");
            return Self { config, url: None };
        }
        Self {
            config,
            url: Some(url_str),
        }
    }

    /// Get cache TTL
    pub fn ttl(&self) -> Duration {
        self.config.ttl
    }

    /// Check if cache is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled && self.url.is_some()
    }

    /// Get a connection from the pool
    async fn get_conn(&self) -> Result<redis::aio::MultiplexedConnection, AppError> {
        match &self.url {
            Some(url) => {
                let client = Client::open(url.as_str())
                    .map_err(|e| AppError::Internal(Some(format!("Redis client error: {}", e))))?;
                client
                    .get_multiplexed_async_connection()
                    .await
                    .map_err(|e| AppError::Internal(Some(format!("Redis connection error: {}", e))))
            }
            None => Err(AppError::Internal(Some("Redis client not initialized".to_string()))),
        }
    }

    /// Get value from cache
    /// Returns None if cache is disabled, key not found, or connection fails
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, AppError> {
        if !self.is_enabled() {
            return Ok(None);
        }

        let mut conn = match self.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!("Redis connection failed, skipping cache get for {}: {}", key, e);
                return Ok(None);
            }
        };

        let result: Option<String> = match conn.get(key).await {
            Ok(r) => r,
            Err(e) => {
                // Check if this is a "key not found" error
                let err_str = e.to_string();
                if err_str.contains("nil") || err_str.contains("key not found") {
                    None
                } else {
                    tracing::debug!("Redis get failed for {}: {}", key, e);
                    return Ok(None);
                }
            }
        };

        match result {
            Some(s) => {
                match serde_json::from_str(&s) {
                    Ok(v) => Ok(Some(v)),
                    Err(e) => {
                        tracing::debug!("Redis deserialization failed for {}: {}", key, e);
                        Ok(None)
                    }
                }
            }
            None => Ok(None),
        }
    }

    /// Set value in cache with default TTL
    pub async fn set<T: serde::Serialize>(&self, key: &str, value: &T) -> Result<(), AppError> {
        self.set_with_ttl(key, value, self.config.ttl).await
    }

    /// Set value in cache with custom TTL
    pub async fn set_with_ttl<T: serde::Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<(), AppError> {
        if !self.is_enabled() {
            return Ok(()); // Skip if Redis not configured
        }

        let mut conn = match self.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!("Redis connection failed, skipping cache set for {}: {}", key, e);
                return Ok(());
            }
        };

        let serialized = match serde_json::to_string(value) {
            Ok(s) => s,
            Err(e) => {
                tracing::debug!("Redis serialization failed for {}: {}", key, e);
                return Ok(());
            }
        };

        let ttl_secs = ttl.as_secs();

        match conn.set_ex(key, &serialized, ttl_secs).await {
            Ok(()) => {
                tracing::debug!("Cached key: {} with TTL: {}s", key, ttl_secs);
                Ok(())
            }
            Err(e) => {
                tracing::debug!("Redis set failed for {}: {}", key, e);
                Ok(()) // Don't fail the request if caching fails
            }
        }
    }

    /// Invalidate key from cache
    pub async fn invalidate(&self, key: &str) -> Result<(), AppError> {
        if !self.is_enabled() {
            return Ok(()); // Skip if Redis not configured
        }

        let mut conn = match self.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!("Redis connection failed, skipping cache invalidate for {}: {}", key, e);
                return Ok(());
            }
        };

        match conn.del(key).await {
            Ok(()) => {
                tracing::debug!("Invalidated key: {}", key);
                Ok(())
            }
            Err(e) => {
                tracing::debug!("Redis invalidate failed for {}: {}", key, e);
                Ok(()) // Don't fail the request if invalidation fails
            }
        }
    }

    /// Invalidate all keys matching pattern using SCAN
    #[allow(dead_code)]
    pub async fn invalidate_pattern(&self, pattern: &str) -> Result<(), AppError> {
        if !self.is_enabled() {
            return Ok(()); // Skip if Redis not configured
        }

        let mut conn = match self.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!("Redis connection failed, skipping cache invalidate pattern {}: {}", pattern, e);
                return Ok(());
            }
        };

        // Use SCAN to find matching keys, then delete them in batches
        let mut cursor = 0u64;
        let pattern_str = pattern.to_string();

        loop {
            let (new_cursor, keys): (u64, Vec<String>) = match redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern_str)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await
            {
                Ok(result) => result,
                Err(e) => {
                    tracing::debug!("Redis scan failed for pattern {}: {}", pattern, e);
                    return Ok(());
                }
            };

            cursor = new_cursor;

            if !keys.is_empty() {
                // Delete the keys found in this batch
                if let Err(e) = conn.del::<_, ()>(keys.as_slice()).await {
                    tracing::debug!("Redis delete failed for pattern {}: {}", pattern, e);
                }
            }

            if cursor == 0 {
                break;
            }
        }

        tracing::debug!("Invalidated keys matching pattern: {}", pattern);
        Ok(())
    }

    /// Check if key exists in cache
    #[allow(dead_code)]
    pub async fn has(&self, key: &str) -> Result<bool, AppError> {
        if !self.is_enabled() {
            return Ok(false);
        }

        let mut conn = match self.get_conn().await {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!("Redis connection failed, skipping cache has for {}: {}", key, e);
                return Ok(false);
            }
        };

        match conn.exists(key).await {
            Ok(exists) => Ok(exists),
            Err(e) => {
                tracing::debug!("Redis exists check failed for {}: {}", key, e);
                Ok(false)
            }
        }
    }

    /// Store user data in cache
    #[allow(dead_code)]
    pub async fn cache_user(&self, user_id: &uuid::Uuid, user_data: &impl serde::Serialize) -> Result<(), AppError> {
        let key = format!("user:{}", user_id);
        self.set(&key, user_data).await
    }

    /// Get user data from cache
    #[allow(dead_code)]
    pub async fn get_user<T: serde::de::DeserializeOwned>(&self, user_id: &uuid::Uuid) -> Result<Option<T>, AppError> {
        let key = format!("user:{}", user_id);
        self.get(&key).await
    }

    /// Invalidate user data from cache
    #[allow(dead_code)]
    pub async fn invalidate_user(&self, user_id: &uuid::Uuid) -> Result<(), AppError> {
        let key = format!("user:{}", user_id);
        self.invalidate(&key).await
    }

    /// Store menu data in cache
    #[allow(dead_code)]
    pub async fn cache_menu(&self, menu_id: &uuid::Uuid, menu_data: &impl serde::Serialize) -> Result<(), AppError> {
        let key = format!("menu:{}", menu_id);
        self.set(&key, menu_data).await
    }

    /// Get menu data from cache
    #[allow(dead_code)]
    pub async fn get_menu<T: serde::de::DeserializeOwned>(&self, menu_id: &uuid::Uuid) -> Result<Option<T>, AppError> {
        let key = format!("menu:{}", menu_id);
        self.get(&key).await
    }

    /// Invalidate menu data from cache
    #[allow(dead_code)]
    pub async fn invalidate_menu(&self, menu_id: &uuid::Uuid) -> Result<(), AppError> {
        let key = format!("menu:{}", menu_id);
        self.invalidate(&key).await
    }

    /// Cache menu tree
    pub async fn cache_menu_tree(&self, data: &impl serde::Serialize) -> Result<(), AppError> {
        // Menu tree has 30 minute TTL
        self.set_with_ttl("menu:tree", data, Duration::from_secs(1800)).await
    }

    /// Get menu tree from cache
    pub async fn get_menu_tree<T: serde::de::DeserializeOwned>(&self) -> Result<Option<T>, AppError> {
        self.get("menu:tree").await
    }

    /// Invalidate menu tree
    pub async fn invalidate_menu_tree(&self) -> Result<(), AppError> {
        self.invalidate("menu:tree").await
    }

    /// Cache department tree
    pub async fn cache_department_tree(&self, data: &impl serde::Serialize) -> Result<(), AppError> {
        // Department tree has 30 minute TTL
        self.set_with_ttl("department:tree", data, Duration::from_secs(1800)).await
    }

    /// Get department tree from cache
    pub async fn get_department_tree<T: serde::de::DeserializeOwned>(&self) -> Result<Option<T>, AppError> {
        self.get("department:tree").await
    }

    /// Invalidate department tree
    pub async fn invalidate_department_tree(&self) -> Result<(), AppError> {
        self.invalidate("department:tree").await
    }

    /// Cache role list
    pub async fn cache_role_list(&self, data: &impl serde::Serialize) -> Result<(), AppError> {
        // Role list has 15 minute TTL
        self.set_with_ttl("role:list", data, Duration::from_secs(900)).await
    }

    /// Get role list from cache
    pub async fn get_role_list<T: serde::de::DeserializeOwned>(&self) -> Result<Option<T>, AppError> {
        self.get("role:list").await
    }

    /// Invalidate role list
    pub async fn invalidate_role_list(&self) -> Result<(), AppError> {
        self.invalidate("role:list").await
    }
}

/// Default cache instance
pub fn create_default_cache(url: &str) -> RedisCache {
    RedisCache::from_url(url, 3600) // 1 hour TTL
}

/// Cache key builders
pub mod keys {
    /// Cache key for user by ID
    pub fn user(user_id: &uuid::Uuid) -> String {
        format!("user:{}", user_id)
    }

    /// Cache key for user by username
    pub fn user_by_username(username: &str) -> String {
        format!("user:username:{}", username)
    }

    /// Cache key for user by email
    pub fn user_by_email(email: &str) -> String {
        format!("user:email:{}", email)
    }

    /// Cache key for menu by ID
    pub fn menu(menu_id: &uuid::Uuid) -> String {
        format!("menu:{}", menu_id)
    }

    /// Cache key for menu tree
    pub fn menu_tree() -> &'static str {
        "menu:tree"
    }

    /// Cache key for role by ID
    pub fn role(role_id: &uuid::Uuid) -> String {
        format!("role:{}", role_id)
    }

    /// Cache key for role list
    pub fn role_list() -> &'static str {
        "role:list"
    }

    /// Cache key for department tree
    pub fn department_tree() -> &'static str {
        "department:tree"
    }

    /// Cache key for dictionary type
    pub fn dict_type(code: &str) -> String {
        format!("dict:type:{}", code)
    }

    /// Cache key for dictionary data
    pub fn dict_data(code: &str) -> String {
        format!("dict:data:{}", code)
    }

    /// Cache key for configuration
    pub fn config(key: &str) -> String {
        format!("config:{}", key)
    }
}
