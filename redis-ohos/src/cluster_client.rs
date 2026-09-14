// Redis Cluster Client wrapper for HarmonyOS NAPI

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::*;
use redis::cluster::{ClusterClient, ClusterClientBuilder};
use redis::{Commands, IntoConnectionInfo, TlsMode};
use std::time::Duration;

use crate::types::ClusterConfig;

/// Redis Cluster Client for HarmonyOS
///
/// This class provides access to Redis Cluster for distributed Redis setups.
/// Redis Cluster automatically shards data across multiple nodes.
#[napi]
pub struct RedisClusterClient {
    inner: ClusterClient,
}

#[napi]
impl RedisClusterClient {
    /// Create a new Cluster client from node URLs
    ///
    /// # Arguments
    /// * `nodes` - Array of cluster node URLs (e.g., ["redis://127.0.0.1:7000"])
    ///   Only one node needs to be specified, but multiple can be provided for redundancy.
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisClusterClient } from 'libredis_ohos.so';
    ///
    /// const client = RedisClusterClient.build([
    ///   "redis://127.0.0.1:7000",
    ///   "redis://127.0.0.1:7001",
    ///   "redis://127.0.0.1:7002"
    /// ]);
    /// ```
    #[napi(factory)]
    pub fn build(nodes: Vec<String>) -> Result<Self> {
        let connection_infos = nodes
            .into_iter()
            .map(|url| {
                url.into_connection_info().map_err(|e| {
                    napi_ohos::Error::from_reason(format!("Failed to parse node URL: {}", e))
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let client = ClusterClient::new(connection_infos).map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to build Cluster client: {}", e))
        })?;

        Ok(RedisClusterClient { inner: client })
    }

    /// Create a Cluster client with custom configuration
    ///
    /// # Arguments
    /// * `nodes` - Array of cluster node URLs
    /// * `config` - Cluster configuration (auth, timeouts, read from replicas, etc.)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const config = {
    ///   username: "user",
    ///   password: "pass",
    ///   readFromReplicas: true,
    ///   connectionTimeoutMs: 5000,
    ///   responseTimeoutMs: 3000
    /// };
    /// const client = RedisClusterClient.buildWithConfig(
    ///   ["redis://127.0.0.1:7000"],
    ///   config
    /// );
    /// ```
    #[napi(factory)]
    pub fn build_with_config(nodes: Vec<String>, config: ClusterConfig) -> Result<Self> {
        let connection_infos = nodes
            .into_iter()
            .map(|url| {
                url.into_connection_info().map_err(|e| {
                    napi_ohos::Error::from_reason(format!("Failed to parse node URL: {}", e))
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut builder = ClusterClientBuilder::new(connection_infos);

        // Apply configuration
        if let Some(username) = config.username {
            builder = builder.username(username);
        }
        if let Some(password) = config.password {
            builder = builder.password(password);
        }
        if let Some(read_from_replicas) = config.read_from_replicas {
            builder = builder.read_from_replicas(read_from_replicas);
        }
        if let Some(timeout_ms) = config.connection_timeout_ms {
            builder = builder.connection_timeout(Duration::from_millis(timeout_ms as u64));
        }
        if let Some(timeout_ms) = config.response_timeout_ms {
            builder = builder.response_timeout(Duration::from_millis(timeout_ms as u64));
        }

        // Handle TLS configuration
        if let Some(use_tls) = config.use_tls {
            if use_tls {
                let tls_mode = if let Some(ref mode_str) = config.tls_mode {
                    match mode_str.to_lowercase().as_str() {
                        "secure" => TlsMode::Secure,
                        "insecure" => TlsMode::Insecure,
                        _ => {
                            return Err(napi_ohos::Error::from_reason(
                                "tls_mode must be 'secure' or 'insecure'".to_string(),
                            ))
                        }
                    }
                } else {
                    TlsMode::Secure // Default to secure
                };
                builder = builder.tls(tls_mode);
            }
        }

        let client = builder.build().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to build Cluster client: {}", e))
        })?;

        Ok(RedisClusterClient { inner: client })
    }

    /// Get a connection to the Redis Cluster
    ///
    /// # Returns
    /// A connection to the cluster that automatically routes commands to the correct nodes
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const conn = client.getConnection();
    /// conn.set("key", "value");
    /// const value = conn.get("key");
    /// ```
    #[napi]
    pub fn get_connection(&self) -> Result<RedisClusterConnection> {
        let conn = self.inner.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to get cluster connection: {}", e))
        })?;

        Ok(RedisClusterConnection { inner: conn })
    }
}

/// Redis Cluster Connection for HarmonyOS
///
/// This class represents an active connection to a Redis Cluster.
/// It automatically routes commands to the correct cluster nodes based on key hashing.
#[napi]
pub struct RedisClusterConnection {
    inner: redis::cluster::ClusterConnection,
}

#[napi]
impl RedisClusterConnection {
    // ==================== String Commands ====================

    /// SET command - Set a string value
    ///
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to set
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// conn.set("mykey", "myvalue");
    /// ```
    #[napi]
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        Commands::set(&mut self.inner, key, value)
            .map_err(|e| napi_ohos::Error::from_reason(format!("SET failed: {}", e)))
    }

    /// GET command - Get a string value
    ///
    /// # Arguments
    /// * `key` - The key to get
    ///
    /// # Returns
    /// The value as a string, or null if key doesn't exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const value = conn.get("mykey");
    /// if (value !== null) {
    ///   console.log("Value:", value);
    /// }
    /// ```
    #[napi]
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Commands::get(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("GET failed: {}", e)))
    }

    /// DEL command - Delete one or more keys
    ///
    /// # Arguments
    /// * `keys` - Array of keys to delete
    ///
    /// # Returns
    /// Number of keys that were deleted
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const deleted = conn.del(["key1", "key2", "key3"]);
    /// console.log(`Deleted ${deleted} keys`);
    /// ```
    #[napi]
    pub fn del(&mut self, keys: Vec<String>) -> Result<i32> {
        Commands::del(&mut self.inner, keys)
            .map_err(|e| napi_ohos::Error::from_reason(format!("DEL failed: {}", e)))
    }

    /// EXISTS command - Check if keys exist
    ///
    /// # Arguments
    /// * `keys` - Array of keys to check
    ///
    /// # Returns
    /// Number of keys that exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const count = conn.exists(["key1", "key2"]);
    /// console.log(`${count} keys exist`);
    /// ```
    #[napi]
    pub fn exists(&mut self, keys: Vec<String>) -> Result<i32> {
        Commands::exists(&mut self.inner, keys)
            .map_err(|e| napi_ohos::Error::from_reason(format!("EXISTS failed: {}", e)))
    }

    /// EXPIRE command - Set a timeout on a key
    ///
    /// # Arguments
    /// * `key` - The key to set timeout on
    /// * `seconds` - Timeout in seconds
    ///
    /// # Returns
    /// true if timeout was set, false if key doesn't exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// conn.set("tempkey", "value");
    /// conn.expire("tempkey", 60); // Expires in 60 seconds
    /// ```
    #[napi]
    pub fn expire(&mut self, key: String, seconds: i64) -> Result<bool> {
        Commands::expire(&mut self.inner, key, seconds)
            .map_err(|e| napi_ohos::Error::from_reason(format!("EXPIRE failed: {}", e)))
    }

    /// TTL command - Get the time to live for a key
    ///
    /// # Arguments
    /// * `key` - The key to check
    ///
    /// # Returns
    /// TTL in seconds, -1 if key has no expiry, -2 if key doesn't exist
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const ttl = conn.ttl("mykey");
    /// if (ttl > 0) {
    ///   console.log(`Key expires in ${ttl} seconds`);
    /// }
    /// ```
    #[napi]
    pub fn ttl(&mut self, key: String) -> Result<i64> {
        Commands::ttl(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("TTL failed: {}", e)))
    }

    /// INCR command - Increment the integer value of a key by one
    ///
    /// # Arguments
    /// * `key` - The key to increment
    ///
    /// # Returns
    /// The value after increment
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const newValue = conn.incr("counter");
    /// console.log(`Counter is now: ${newValue}`);
    /// ```
    #[napi]
    pub fn incr(&mut self, key: String) -> Result<i64> {
        Commands::incr(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("INCR failed: {}", e)))
    }

    /// DECR command - Decrement the integer value of a key by one
    ///
    /// # Arguments
    /// * `key` - The key to decrement
    ///
    /// # Returns
    /// The value after decrement
    #[napi]
    pub fn decr(&mut self, key: String) -> Result<i64> {
        Commands::decr(&mut self.inner, key)
            .map_err(|e| napi_ohos::Error::from_reason(format!("DECR failed: {}", e)))
    }
}

