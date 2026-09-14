// Redis Client wrapper for HarmonyOS NAPI

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::*;
use redis::Client;

use crate::connection::RedisConnection;
use crate::json_connection::RedisJsonConnection;
use crate::types::RedisClientConfig;

/// Redis Client for HarmonyOS
///
/// This class represents a Redis client that can create connections to a Redis server.
#[napi]
pub struct RedisClient {
    inner: Client,
}

impl RedisClient {
    /// Create a RedisClient from an existing redis::Client
    /// This is used internally by other modules
    pub(crate) fn from_inner(client: Client) -> Self {
        RedisClient { inner: client }
    }
}

#[napi]
impl RedisClient {
    /// Create a new Redis client from URL
    ///
    /// # Arguments
    /// * `url` - Redis connection URL
    ///   - TCP: `redis://127.0.0.1:6379`
    ///   - TCP with auth: `redis://:password@127.0.0.1:6379`
    ///   - TCP with user and password: `redis://username:password@127.0.0.1:6379`
    ///   - TCP with database: `redis://127.0.0.1:6379/0`
    ///   - TLS: `rediss://127.0.0.1:6379`
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisClient } from 'libredis_ohos.so';
    ///
    /// const client = new RedisClient("redis://127.0.0.1:6379");
    /// ```
    #[napi(constructor)]
    pub fn new(url: String) -> Result<Self> {
        let client = Client::open(url.as_str()).map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to create Redis client: {}", e))
        })?;
        Ok(RedisClient { inner: client })
    }

    /// Create a new Redis client from configuration object
    ///
    /// This method allows you to create a Redis client by passing individual
    /// configuration parameters instead of a URL string. All parameters are optional
    /// and have sensible defaults.
    ///
    /// # Arguments
    /// * `config` - Configuration object with the following optional fields:
    ///   - `host`: Redis server host (default: "127.0.0.1")
    ///   - `port`: Redis server port (default: 6379)
    ///   - `db`: Database index (default: 0)
    ///   - `username`: Username for authentication (Redis 6.0+)
    ///   - `password`: Password for authentication
    ///   - `use_tls`: Use TLS/SSL connection (default: false)
    ///   - `timeout_ms`: Connection timeout in milliseconds
    ///
    /// # Returns
    /// A new RedisClient instance
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisClient } from 'libredis_ohos.so';
    ///
    /// // Minimal configuration (uses all defaults)
    /// const client1 = RedisClient.fromConfig({});
    ///
    /// // Custom host and port
    /// const client2 = RedisClient.fromConfig({
    ///   host: "192.168.1.100",
    ///   port: 6380
    /// });
    ///
    /// // With authentication
    /// const client3 = RedisClient.fromConfig({
    ///   host: "redis.example.com",
    ///   port: 6379,
    ///   password: "mypassword",
    ///   db: 1
    /// });
    ///
    /// // With TLS and full authentication
    /// const client4 = RedisClient.fromConfig({
    ///   host: "secure-redis.example.com",
    ///   port: 6380,
    ///   username: "myuser",
    ///   password: "mypassword",
    ///   use_tls: true,
    ///   db: 2
    /// });
    ///
    /// // Get connection
    /// const conn = client1.getConnection();
    /// ```
    #[napi(factory)]
    pub fn from_config(config: RedisClientConfig) -> Result<Self> {
        let url = config.to_url();
        let client = Client::open(url.as_str()).map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to create Redis client: {}", e))
        })?;
        Ok(RedisClient { inner: client })
    }

    /// Get a synchronous connection to Redis
    ///
    /// This method creates a new connection to the Redis server.
    /// The connection can be used to execute Redis commands.
    ///
    /// # Returns
    /// A RedisConnection object
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = new RedisClient("redis://127.0.0.1:6379");
    /// const conn = client.getConnection();
    /// ```
    #[napi]
    pub fn get_connection(&self) -> Result<RedisConnection> {
        let conn = self.inner.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to connect to Redis: {}", e))
        })?;
        Ok(RedisConnection::new(conn))
    }

    /// Get a synchronous connection with timeout
    ///
    /// # Arguments
    /// * `timeout_ms` - Connection timeout in milliseconds
    ///
    /// # Returns
    /// A RedisConnection object
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = new RedisClient("redis://127.0.0.1:6379");
    /// const conn = client.getConnectionWithTimeout(5000); // 5 seconds timeout
    /// ```
    #[napi]
    pub fn get_connection_with_timeout(&self, timeout_ms: u32) -> Result<RedisConnection> {
        let timeout = std::time::Duration::from_millis(timeout_ms as u64);
        let conn = self
            .inner
            .get_connection_with_timeout(timeout)
            .map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to connect to Redis: {}", e))
            })?;
        Ok(RedisConnection::new(conn))
    }

    /// Get a connection and switch to specified database
    ///
    /// This is a convenience method that creates a connection and immediately
    /// switches to the specified database using the SELECT command.
    ///
    /// # Arguments
    /// * `db` - Database index (0-15 typically, depends on Redis configuration)
    ///
    /// # Returns
    /// A RedisConnection object already switched to the specified database
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = new RedisClient("redis://127.0.0.1:6379");
    /// const conn = client.getConnectionWithDb(1); // Connect and switch to database 1
    /// conn.set("key", "value"); // This will be stored in database 1
    /// ```
    #[napi]
    pub fn get_connection_with_db(&self, db: i32) -> Result<RedisConnection> {
        let mut conn = self.get_connection()?;
        conn.select(db)?;
        Ok(conn)
    }

    /// Get a connection with timeout and switch to specified database
    ///
    /// # Arguments
    /// * `timeout_ms` - Connection timeout in milliseconds
    /// * `db` - Database index (0-15 typically)
    ///
    /// # Returns
    /// A RedisConnection object already switched to the specified database
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = new RedisClient("redis://127.0.0.1:6379");
    /// const conn = client.getConnectionWithTimeoutAndDb(5000, 2); // 5s timeout, database 2
    /// ```
    #[napi]
    pub fn get_connection_with_timeout_and_db(
        &self,
        timeout_ms: u32,
        db: i32,
    ) -> Result<RedisConnection> {
        let mut conn = self.get_connection_with_timeout(timeout_ms)?;
        conn.select(db)?;
        Ok(conn)
    }

    /// Get a JSON connection to Redis
    ///
    /// This method creates a connection specifically for RedisJSON commands.
    /// Requires RedisJSON module to be loaded on the Redis server.
    ///
    /// # Returns
    /// A RedisJsonConnection object
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = new RedisClient("redis://127.0.0.1:6379");
    /// const jsonConn = client.getJsonConnection();
    /// jsonConn.jsonSet("user:1", "$", JSON.stringify({name: "John", age: 30}));
    /// const user = jsonConn.jsonGet("user:1", "$");
    /// ```
    #[napi]
    pub fn get_json_connection(&self) -> Result<RedisJsonConnection> {
        let conn = self.inner.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to connect to Redis: {}", e))
        })?;
        Ok(RedisJsonConnection::new(conn))
    }
}
