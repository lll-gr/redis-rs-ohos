// Redis types for HarmonyOS NAPI

use napi_derive_ohos::napi;

/// Redis Client Configuration
///
/// Configuration object for creating a Redis client with individual parameters.
/// All fields are optional and have default values.
///
/// # Example (ArkTS)
/// ```typescript
/// import { RedisClient, RedisClientConfig } from 'libredis_ohos.so';
///
/// // Minimal configuration (uses all defaults)
/// const config1: RedisClientConfig = {};
/// const client1 = RedisClient.fromConfig(config1);
///
/// // Custom host and port
/// const config2: RedisClientConfig = {
///   host: "192.168.1.100",
///   port: 6380
/// };
/// const client2 = RedisClient.fromConfig(config2);
///
/// // With authentication
/// const config3: RedisClientConfig = {
///   host: "redis.example.com",
///   password: "mypassword",
///   db: 1
/// };
/// const client3 = RedisClient.fromConfig(config3);
/// ```
#[napi(object)]
#[derive(Debug, Clone)]
pub struct RedisClientConfig {
    /// Redis server host (default: "127.0.0.1")
    pub host: Option<String>,

    /// Redis server port (default: 6379)
    pub port: Option<u16>,

    /// Database index (default: 0)
    pub db: Option<i32>,

    /// Username for authentication (Redis 6.0+)
    pub username: Option<String>,

    /// Password for authentication
    pub password: Option<String>,

    /// Use TLS/SSL connection (default: false)
    /// If true, uses rediss:// protocol
    pub use_tls: Option<bool>,

    /// Connection timeout in milliseconds (default: no timeout)
    pub timeout_ms: Option<u32>,
}

impl Default for RedisClientConfig {
    fn default() -> Self {
        RedisClientConfig {
            host: Some("127.0.0.1".to_string()),
            port: Some(6379),
            db: Some(0),
            username: None,
            password: None,
            use_tls: Some(false),
            timeout_ms: None,
        }
    }
}

impl RedisClientConfig {
    /// Build a Redis connection URL from the configuration
    pub fn to_url(&self) -> String {
        let host = self.host.as_deref().unwrap_or("127.0.0.1");
        let port = self.port.unwrap_or(6379);
        let db = self.db.unwrap_or(0);
        let use_tls = self.use_tls.unwrap_or(false);

        let protocol = if use_tls { "rediss" } else { "redis" };

        // Build auth part
        let auth = match (&self.username, &self.password) {
            (Some(username), Some(password)) => format!("{}:{}@", username, password),
            (None, Some(password)) => format!(":{}@", password),
            _ => String::new(),
        };

        format!("{}://{}{}:{}/{}", protocol, auth, host, port, db)
    }
}

/// Redis value type enumeration
///
/// Represents the different types of values that can be stored in Redis.
#[napi]
pub enum RedisValueType {
    /// String type
    String,
    /// List type
    List,
    /// Set type
    Set,
    /// Sorted Set (ZSet) type
    ZSet,
    /// Hash type
    Hash,
    /// Stream type
    Stream,
    /// None/Unknown type
    None,
}

/// Redis hash field expiration option
///
/// Used with hash field expiration commands (HEXPIRE, HPEXPIRE, etc.)
/// to control when expiration should be set.
#[napi]
pub enum RedisExpireOption {
    /// Set expiration regardless of the field's current expiration
    None,
    /// Only set expiration when the field has no expiration
    NX,
    /// Only set expiration when the field has an existing expiration
    XX,
    /// Only set expiration when the new expiration is greater than current one
    GT,
    /// Only set expiration when the new expiration is less than current one
    LT,
}

/// Result type for hash field expiration commands
///
/// Represents the result of setting expiration on a hash field.
#[napi]
pub enum RedisExpireResult {
    /// The expiration was successfully set (value: 1)
    Success,
    /// The condition was not met (value: 0)
    ConditionNotMet,
    /// Called with 0 seconds/milliseconds (value: 2)
    CalledWithZero,
    /// The field does not exist (value: -2)
    FieldNotExists,
    /// The field exists but has no expiration (value: -1)
    FieldHasNoExpiration,
}

impl RedisValueType {
    /// Convert from redis::ValueType to RedisValueType
    pub fn from_redis_value_type(value_type: redis::ValueType) -> Self {
        match value_type {
            redis::ValueType::String => RedisValueType::String,
            redis::ValueType::List => RedisValueType::List,
            redis::ValueType::Set => RedisValueType::Set,
            redis::ValueType::ZSet => RedisValueType::ZSet,
            redis::ValueType::Hash => RedisValueType::Hash,
            redis::ValueType::Stream => RedisValueType::Stream,
            _ => RedisValueType::None,
        }
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            RedisValueType::String => "string".to_string(),
            RedisValueType::List => "list".to_string(),
            RedisValueType::Set => "set".to_string(),
            RedisValueType::ZSet => "zset".to_string(),
            RedisValueType::Hash => "hash".to_string(),
            RedisValueType::Stream => "stream".to_string(),
            RedisValueType::None => "none".to_string(),
        }
    }
}

impl RedisExpireOption {
    /// Convert to redis::ExpireOption
    pub fn to_redis_expire_option(&self) -> redis::ExpireOption {
        match self {
            RedisExpireOption::None => redis::ExpireOption::NONE,
            RedisExpireOption::NX => redis::ExpireOption::NX,
            RedisExpireOption::XX => redis::ExpireOption::XX,
            RedisExpireOption::GT => redis::ExpireOption::GT,
            RedisExpireOption::LT => redis::ExpireOption::LT,
        }
    }
}

impl RedisExpireResult {
    /// Convert from redis::IntegerReplyOrNoOp to RedisExpireResult
    pub fn from_integer_reply(reply: redis::IntegerReplyOrNoOp) -> Self {
        match reply {
            redis::IntegerReplyOrNoOp::IntegerReply(1) => RedisExpireResult::Success,
            redis::IntegerReplyOrNoOp::IntegerReply(0) => RedisExpireResult::ConditionNotMet,
            redis::IntegerReplyOrNoOp::IntegerReply(2) => RedisExpireResult::CalledWithZero,
            redis::IntegerReplyOrNoOp::NotExists => RedisExpireResult::FieldNotExists,
            redis::IntegerReplyOrNoOp::ExistsButNotRelevant => RedisExpireResult::FieldHasNoExpiration,
            _ => RedisExpireResult::ConditionNotMet,
        }
    }

    /// Convert to integer value
    pub fn to_i32(&self) -> i32 {
        match self {
            RedisExpireResult::Success => 1,
            RedisExpireResult::ConditionNotMet => 0,
            RedisExpireResult::CalledWithZero => 2,
            RedisExpireResult::FieldNotExists => -2,
            RedisExpireResult::FieldHasNoExpiration => -1,
        }
    }
}

/// Database statistics from keyspace info
///
/// Contains statistics for a single Redis database.
///
/// # Example (ArkTS)
/// ```typescript
/// const keyspaceInfo = conn.getKeyspaceInfo();
/// for (const [dbName, stats] of Object.entries(keyspaceInfo)) {
///   console.log(`Database: ${dbName}`);
///   console.log(`  Keys: ${stats.keys}`);
///   console.log(`  Expires: ${stats.expires}`);
///   console.log(`  Avg TTL: ${stats.avgTtl} ms`);
/// }
/// ```
#[napi(object)]
#[derive(Debug, Clone)]
pub struct DatabaseStats {
    /// Number of keys in the database
    pub keys: i64,

    /// Number of keys with an expiration
    pub expires: i64,

    /// Average TTL in milliseconds (0 if no keys have TTL)
    pub avg_ttl: i64,
}

impl Default for DatabaseStats {
    fn default() -> Self {
        DatabaseStats {
            keys: 0,
            expires: 0,
            avg_ttl: 0,
        }
    }
}

/// Redis INFO data structure
///
/// Contains all sections from the Redis INFO command as structured fields.
/// Each field is a Map containing key-value pairs for that section.
///
/// # Example (ArkTS)
/// ```typescript
/// const info = conn.getInfoParsed();
///
/// // Access server section
/// if (info.server) {
///   console.log("Redis version:", info.server.get("redis_version"));
///   console.log("OS:", info.server.get("os"));
/// }
///
/// // Access memory section
/// if (info.memory) {
///   console.log("Used memory:", info.memory.get("used_memory_human"));
/// }
///
/// // Access stats section
/// if (info.stats) {
///   console.log("Total commands:", info.stats.get("total_commands_processed"));
/// }
/// ```
#[napi(object)]
#[derive(Debug, Clone, Default)]
pub struct RedisInfo {
    /// Server information (version, mode, OS, etc.)
    pub server: Option<std::collections::HashMap<String, String>>,

    /// Client connections information
    pub clients: Option<std::collections::HashMap<String, String>>,

    /// Memory usage information
    pub memory: Option<std::collections::HashMap<String, String>>,

    /// Persistence (RDB/AOF) information
    pub persistence: Option<std::collections::HashMap<String, String>>,

    /// General statistics
    pub stats: Option<std::collections::HashMap<String, String>>,

    /// Replication information
    pub replication: Option<std::collections::HashMap<String, String>>,

    /// CPU usage information
    pub cpu: Option<std::collections::HashMap<String, String>>,

    /// Command statistics
    pub commandstats: Option<std::collections::HashMap<String, String>>,

    /// Cluster information
    pub cluster: Option<std::collections::HashMap<String, String>>,

    /// Keyspace statistics (per-database)
    pub keyspace: Option<std::collections::HashMap<String, String>>,

    /// Modules information
    pub modules: Option<std::collections::HashMap<String, String>>,

    /// Error statistics
    pub errorstats: Option<std::collections::HashMap<String, String>>,
}

impl RedisInfo {
    /// Set a section's data
    pub fn set_section(&mut self, section_name: &str, data: std::collections::HashMap<String, String>) {
        match section_name.to_lowercase().as_str() {
            "server" => self.server = Some(data),
            "clients" => self.clients = Some(data),
            "memory" => self.memory = Some(data),
            "persistence" => self.persistence = Some(data),
            "stats" => self.stats = Some(data),
            "replication" => self.replication = Some(data),
            "cpu" => self.cpu = Some(data),
            "commandstats" => self.commandstats = Some(data),
            "cluster" => self.cluster = Some(data),
            "keyspace" => self.keyspace = Some(data),
            "modules" => self.modules = Some(data),
            "errorstats" => self.errorstats = Some(data),
            _ => {} // Ignore unknown sections
        }
    }
}

/// TLS Certificate Configuration
///
/// Configuration for TLS/SSL connections with custom certificates.
///
/// # Example (ArkTS)
/// ```typescript
/// const tlsConfig: TlsConfig = {
///   clientCert: readFileAsString("client-cert.pem"),
///   clientKey: readFileAsString("client-key.pem"),
///   rootCert: readFileAsString("ca-cert.pem")
/// };
/// ```
#[napi(object)]
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Client certificate in PEM format (for mTLS)
    pub client_cert: Option<String>,

    /// Client private key in PEM format (for mTLS)
    pub client_key: Option<String>,

    /// Root CA certificate in PEM format
    pub root_cert: Option<String>,
}

/// Sentinel Node Configuration
///
/// Configuration for connecting to Redis Sentinel nodes.
///
/// # Example (ArkTS)
/// ```typescript
/// const sentinelConfig: SentinelNodeConfig = {
///   tlsMode: "secure",
///   username: "sentinel_user",
///   password: "sentinel_pass",
///   db: 0
/// };
/// ```
#[napi(object)]
#[derive(Debug, Clone)]
pub struct SentinelNodeConfig {
    /// TLS mode: "none", "secure", or "insecure"
    pub tls_mode: Option<String>,

    /// Username for authentication
    pub username: Option<String>,

    /// Password for authentication
    pub password: Option<String>,

    /// Database index
    pub db: Option<i32>,
}

/// Cluster Client Configuration
///
/// Configuration for Redis Cluster connections.
///
/// # Example (ArkTS)
/// ```typescript
/// const clusterConfig: ClusterConfig = {
///   username: "cluster_user",
///   password: "cluster_pass",
///   readFromReplicas: true,
///   connectionTimeoutMs: 5000,
///   responseTimeoutMs: 3000
/// };
/// ```
#[napi(object)]
#[derive(Debug, Clone)]
pub struct ClusterConfig {
    /// Username for authentication
    pub username: Option<String>,

    /// Password for authentication
    pub password: Option<String>,

    /// Whether to read from replica nodes (default: false)
    pub read_from_replicas: Option<bool>,

    /// Connection timeout in milliseconds
    pub connection_timeout_ms: Option<u32>,

    /// Response timeout in milliseconds
    pub response_timeout_ms: Option<u32>,

    /// Use TLS/SSL connection
    pub use_tls: Option<bool>,

    /// TLS mode: "secure" or "insecure"
    pub tls_mode: Option<String>,
}

