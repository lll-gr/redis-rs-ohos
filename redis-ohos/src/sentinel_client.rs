// Redis Sentinel Client wrapper for HarmonyOS NAPI

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::*;
use redis::sentinel::{Sentinel, SentinelClient, SentinelNodeConnectionInfo, SentinelServerType};
use redis::{ConnectionInfo, IntoConnectionInfo, RedisConnectionInfo, TlsMode};

use crate::connection::RedisConnection;
use crate::types::SentinelNodeConfig;

/// Redis Sentinel for HarmonyOS
///
/// This class provides access to Redis Sentinel for high availability setups.
/// Sentinel monitors Redis master and replica nodes and provides automatic failover.
#[napi]
pub struct RedisSentinel {
    inner: Sentinel,
}

#[napi]
impl RedisSentinel {
    /// Create a new Sentinel client from node URLs
    ///
    /// # Arguments
    /// * `nodes` - Array of sentinel node URLs (e.g., ["redis://127.0.0.1:26379"])
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { RedisSentinel } from 'libredis_ohos.so';
    ///
    /// const sentinel = RedisSentinel.build([
    ///   "redis://127.0.0.1:26379",
    ///   "redis://127.0.0.1:26380",
    ///   "redis://127.0.0.1:26381"
    /// ]);
    /// ```
    #[napi(factory)]
    pub fn build(nodes: Vec<String>) -> Result<Self> {
        let connection_infos: Vec<ConnectionInfo> = nodes
            .into_iter()
            .map(|url| {
                url.into_connection_info().map_err(|e| {
                    napi_ohos::Error::from_reason(format!("Failed to parse node URL: {}", e))
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let sentinel = Sentinel::build(connection_infos).map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to build Sentinel: {}", e))
        })?;

        Ok(RedisSentinel { inner: sentinel })
    }

    /// Get a connection to the master node
    ///
    /// # Arguments
    /// * `service_name` - Name of the master service (configured in Sentinel)
    ///
    /// # Returns
    /// A connection to the master node
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const masterConn = sentinel.masterFor("mymaster");
    /// masterConn.set("key", "value");
    /// ```
    #[napi]
    pub fn master_for(&mut self, service_name: String) -> Result<RedisConnection> {
        let client = self
            .inner
            .master_for(&service_name, None)
            .map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to get master client: {}", e))
            })?;

        let conn = client.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to connect to master: {}", e))
        })?;

        Ok(RedisConnection::new(conn))
    }

    /// Get a connection to a replica node
    ///
    /// # Arguments
    /// * `service_name` - Name of the master service (configured in Sentinel)
    ///
    /// # Returns
    /// A connection to a replica node
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const replicaConn = sentinel.replicaFor("mymaster");
    /// const value = replicaConn.get("key");
    /// ```
    #[napi]
    pub fn replica_for(&mut self, service_name: String) -> Result<RedisConnection> {
        let client = self
            .inner
            .replica_for(&service_name, None)
            .map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to get replica client: {}", e))
            })?;

        let conn = client.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to connect to replica: {}", e))
        })?;

        Ok(RedisConnection::new(conn))
    }

    /// Get a connection to the master node with custom configuration
    ///
    /// # Arguments
    /// * `service_name` - Name of the master service
    /// * `config` - Node connection configuration (TLS, auth, etc.)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const config = {
    ///   tlsMode: "secure",
    ///   username: "user",
    ///   password: "pass"
    /// };
    /// const masterConn = sentinel.masterForWithConfig("mymaster", config);
    /// ```
    #[napi]
    pub fn master_for_with_config(
        &mut self,
        service_name: String,
        config: SentinelNodeConfig,
    ) -> Result<RedisConnection> {
        let node_info = build_sentinel_node_info(config)?;

        let client = self
            .inner
            .master_for(&service_name, Some(&node_info))
            .map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to get master client: {}", e))
            })?;

        let conn = client.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to connect to master: {}", e))
        })?;

        Ok(RedisConnection::new(conn))
    }

    /// Get a connection to a replica node with custom configuration
    ///
    /// # Arguments
    /// * `service_name` - Name of the master service
    /// * `config` - Node connection configuration (TLS, auth, etc.)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const config = {
    ///   tlsMode: "insecure",
    ///   password: "pass"
    /// };
    /// const replicaConn = sentinel.replicaForWithConfig("mymaster", config);
    /// ```
    #[napi]
    pub fn replica_for_with_config(
        &mut self,
        service_name: String,
        config: SentinelNodeConfig,
    ) -> Result<RedisConnection> {
        let node_info = build_sentinel_node_info(config)?;

        let client = self
            .inner
            .replica_for(&service_name, Some(&node_info))
            .map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to get replica client: {}", e))
            })?;

        let conn = client.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to connect to replica: {}", e))
        })?;

        Ok(RedisConnection::new(conn))
    }
}

/// Redis Sentinel Client for HarmonyOS
///
/// A utility wrapper around Sentinel that provides a Client-like interface.
/// This always connects to the same type of node (master or replica).
#[napi]
pub struct RedisSentinelClient {
    inner: SentinelClient,
}

#[napi]
impl RedisSentinelClient {
    /// Create a Sentinel client that connects to master nodes
    ///
    /// # Arguments
    /// * `nodes` - Array of sentinel node URLs
    /// * `service_name` - Name of the master service
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = RedisSentinelClient.buildMaster(
    ///   ["redis://127.0.0.1:26379"],
    ///   "mymaster"
    /// );
    /// const conn = client.getConnection();
    /// ```
    #[napi(factory)]
    pub fn build_master(nodes: Vec<String>, service_name: String) -> Result<Self> {
        build_sentinel_client(nodes, service_name, SentinelServerType::Master, None)
    }

    /// Create a Sentinel client that connects to replica nodes
    ///
    /// # Arguments
    /// * `nodes` - Array of sentinel node URLs
    /// * `service_name` - Name of the master service
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = RedisSentinelClient.buildReplica(
    ///   ["redis://127.0.0.1:26379"],
    ///   "mymaster"
    /// );
    /// const conn = client.getConnection();
    /// ```
    #[napi(factory)]
    pub fn build_replica(nodes: Vec<String>, service_name: String) -> Result<Self> {
        build_sentinel_client(nodes, service_name, SentinelServerType::Replica, None)
    }

    /// Create a Sentinel client with custom configuration
    ///
    /// # Arguments
    /// * `nodes` - Array of sentinel node URLs
    /// * `service_name` - Name of the master service
    /// * `server_type` - "master" or "replica"
    /// * `config` - Node connection configuration
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const config = { tlsMode: "secure", password: "pass" };
    /// const client = RedisSentinelClient.buildWithConfig(
    ///   ["redis://127.0.0.1:26379"],
    ///   "mymaster",
    ///   "master",
    ///   config
    /// );
    /// ```
    #[napi(factory)]
    pub fn build_with_config(
        nodes: Vec<String>,
        service_name: String,
        server_type: String,
        config: SentinelNodeConfig,
    ) -> Result<Self> {
        let server_type = match server_type.to_lowercase().as_str() {
            "master" => SentinelServerType::Master,
            "replica" => SentinelServerType::Replica,
            _ => {
                return Err(napi_ohos::Error::from_reason(
                    "server_type must be 'master' or 'replica'".to_string(),
                ))
            }
        };

        build_sentinel_client(nodes, service_name, server_type, Some(config))
    }

    /// Get a connection from the Sentinel client
    ///
    /// # Returns
    /// A connection to the configured node type (master or replica)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const conn = client.getConnection();
    /// conn.set("key", "value");
    /// ```
    #[napi]
    pub fn get_connection(&self) -> Result<RedisConnection> {
        let conn = self.inner.get_connection().map_err(|e| {
            napi_ohos::Error::from_reason(format!("Failed to get connection: {}", e))
        })?;

        Ok(RedisConnection::new(conn))
    }
}

// Helper functions

fn build_sentinel_node_info(config: SentinelNodeConfig) -> Result<SentinelNodeConnectionInfo> {
    let mut node_info = SentinelNodeConnectionInfo::default();

    // Set TLS mode
    if let Some(tls_mode_str) = config.tls_mode {
        let tls_mode = match tls_mode_str.to_lowercase().as_str() {
            "secure" => TlsMode::Secure,
            "insecure" => TlsMode::Insecure,
            "none" => {
                // Don't set TLS mode
                return Ok(node_info);
            }
            _ => {
                return Err(napi_ohos::Error::from_reason(
                    "tls_mode must be 'secure', 'insecure', or 'none'".to_string(),
                ))
            }
        };
        node_info = node_info.set_tls_mode(tls_mode);
    }

    // Set Redis connection info if needed
    if config.username.is_some() || config.password.is_some() || config.db.is_some() {
        let mut redis_info = RedisConnectionInfo::default();

        if let Some(username) = config.username {
            redis_info = redis_info.set_username(username);
        }
        if let Some(password) = config.password {
            redis_info = redis_info.set_password(password);
        }
        if let Some(db) = config.db {
            redis_info = redis_info.set_db(db);
        }

        node_info = node_info.set_redis_connection_info(redis_info);
    }

    Ok(node_info)
}

fn build_sentinel_client(
    nodes: Vec<String>,
    service_name: String,
    server_type: SentinelServerType,
    config: Option<SentinelNodeConfig>,
) -> Result<RedisSentinelClient> {
    let node_info = if let Some(cfg) = config {
        Some(build_sentinel_node_info(cfg)?)
    } else {
        None
    };

    let client = SentinelClient::build(nodes, service_name, node_info, server_type).map_err(
        |e| napi_ohos::Error::from_reason(format!("Failed to build Sentinel client: {}", e)),
    )?;

    Ok(RedisSentinelClient { inner: client })
}

