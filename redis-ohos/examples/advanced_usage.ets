/**
 * Advanced Redis Usage Examples for HarmonyOS
 * 
 * This file demonstrates advanced features including:
 * - TLS/SSL connections
 * - Redis Sentinel for high availability
 * - Redis Cluster for distributed setups
 */

import {
  RedisClient,
  RedisClusterClient,
  RedisSentinel,
  RedisSentinelClient,
  TlsClientBuilder,
  TlsConfig,
  SentinelNodeConfig,
  ClusterConfig
} from 'libredis_ohos.so';

// ==================== TLS/SSL Examples ====================

/**
 * Example 1: Basic TLS connection (using system trust store)
 */
function basicTlsExample() {
  // Simple TLS connection using rediss:// scheme
  const client = new RedisClient("rediss://127.0.0.1:6380");
  const conn = client.getConnection();
  
  conn.set("secure_key", "secure_value");
  const value = conn.get("secure_key");
  console.log("Secure value:", value);
}

/**
 * Example 2: TLS with custom certificates (mTLS)
 */
function mtlsExample() {
  // Configure TLS with custom certificates
  const tlsConfig: TlsConfig = {
    clientCert: readCertFile("client-cert.pem"),
    clientKey: readCertFile("client-key.pem"),
    rootCert: readCertFile("ca-cert.pem")
  };
  
  const builder = new TlsClientBuilder("rediss://127.0.0.1:6380");
  builder.withTlsConfig(tlsConfig);
  
  const client = builder.build();
  const conn = client.getConnection();
  
  conn.set("mtls_key", "mtls_value");
  console.log("mTLS connection successful!");
}

/**
 * Example 3: TLS with authentication
 */
function tlsWithAuthExample() {
  const client = new RedisClient("rediss://username:password@redis.example.com:6380/0");
  const conn = client.getConnection();
  
  conn.set("auth_key", "auth_value");
  console.log("TLS with auth successful!");
}

// ==================== Redis Sentinel Examples ====================

/**
 * Example 4: Basic Sentinel usage
 */
function basicSentinelExample() {
  // Connect to Sentinel nodes
  const sentinel = RedisSentinel.build([
    "redis://127.0.0.1:26379",
    "redis://127.0.0.1:26380",
    "redis://127.0.0.1:26381"
  ]);
  
  // Get connection to master
  const masterConn = sentinel.masterFor("mymaster");
  masterConn.set("key", "value");
  
  // Get connection to replica (for read operations)
  const replicaConn = sentinel.replicaFor("mymaster");
  const value = replicaConn.get("key");
  console.log("Value from replica:", value);
}

/**
 * Example 5: Sentinel with TLS and authentication
 */
function sentinelWithTlsExample() {
  const sentinel = RedisSentinel.build([
    "redis://sentinel_user:sentinel_pass@127.0.0.1:26379"
  ]);
  
  // Configure connection to Redis nodes
  const nodeConfig: SentinelNodeConfig = {
    tlsMode: "secure",
    username: "redis_user",
    password: "redis_pass",
    db: 1
  };
  
  const masterConn = sentinel.masterForWithConfig("mymaster", nodeConfig);
  masterConn.set("secure_key", "secure_value");
  
  const replicaConn = sentinel.replicaForWithConfig("mymaster", nodeConfig);
  const value = replicaConn.get("secure_key");
  console.log("Value:", value);
}

/**
 * Example 6: Sentinel Client (simplified interface)
 */
function sentinelClientExample() {
  // Create a client that always connects to master
  const masterClient = RedisSentinelClient.buildMaster(
    ["redis://127.0.0.1:26379"],
    "mymaster"
  );
  
  // Create a client that always connects to replicas
  const replicaClient = RedisSentinelClient.buildReplica(
    ["redis://127.0.0.1:26379"],
    "mymaster"
  );
  
  // Use like regular clients
  const masterConn = masterClient.getConnection();
  masterConn.set("key", "value");
  
  const replicaConn = replicaClient.getConnection();
  const value = replicaConn.get("key");
  console.log("Value:", value);
}

/**
 * Example 7: Sentinel Client with configuration
 */
function sentinelClientWithConfigExample() {
  const config: SentinelNodeConfig = {
    tlsMode: "insecure",
    password: "mypassword",
    db: 0
  };
  
  const client = RedisSentinelClient.buildWithConfig(
    ["redis://127.0.0.1:26379"],
    "mymaster",
    "master",  // or "replica"
    config
  );
  
  const conn = client.getConnection();
  conn.set("key", "value");
}

// ==================== Redis Cluster Examples ====================

/**
 * Example 8: Basic Cluster usage
 */
function basicClusterExample() {
  // Connect to cluster (only need one node, but can specify multiple for redundancy)
  const client = RedisClusterClient.build([
    "redis://127.0.0.1:7000",
    "redis://127.0.0.1:7001",
    "redis://127.0.0.1:7002"
  ]);
  
  const conn = client.getConnection();
  
  // Commands are automatically routed to the correct node
  conn.set("user:1000", "John");
  conn.set("user:2000", "Jane");
  
  const user1 = conn.get("user:1000");
  const user2 = conn.get("user:2000");
  
  console.log("User 1:", user1);
  console.log("User 2:", user2);
}

/**
 * Example 9: Cluster with authentication and read from replicas
 */
function clusterWithConfigExample() {
  const config: ClusterConfig = {
    username: "cluster_user",
    password: "cluster_pass",
    readFromReplicas: true,  // Enable reading from replica nodes
    connectionTimeoutMs: 5000,
    responseTimeoutMs: 3000
  };
  
  const client = RedisClusterClient.buildWithConfig(
    ["redis://127.0.0.1:7000"],
    config
  );
  
  const conn = client.getConnection();
  
  // Write to master
  conn.set("key", "value");
  
  // Read operations may be served by replicas
  const value = conn.get("key");
  console.log("Value:", value);
}

/**
 * Example 10: Cluster with TLS
 */
function clusterWithTlsExample() {
  const config: ClusterConfig = {
    password: "mypassword",
    useTls: true,
    tlsMode: "secure",  // or "insecure" for self-signed certs
    readFromReplicas: false
  };
  
  const client = RedisClusterClient.buildWithConfig(
    ["rediss://127.0.0.1:7000"],
    config
  );
  
  const conn = client.getConnection();
  conn.set("secure_key", "secure_value");
  console.log("Cluster TLS connection successful!");
}

/**
 * Example 11: Cluster operations
 */
function clusterOperationsExample() {
  const client = RedisClusterClient.build(["redis://127.0.0.1:7000"]);
  const conn = client.getConnection();
  
  // String operations
  conn.set("counter", "0");
  const newValue = conn.incr("counter");
  console.log("Counter:", newValue);
  
  // Key operations
  conn.expire("counter", 3600);  // Expire in 1 hour
  const ttl = conn.ttl("counter");
  console.log("TTL:", ttl, "seconds");
  
  // Check existence
  const exists = conn.exists(["counter", "nonexistent"]);
  console.log("Keys exist:", exists);
  
  // Delete keys
  const deleted = conn.del(["counter"]);
  console.log("Deleted:", deleted, "keys");
}

// ==================== Combined Examples ====================

/**
 * Example 12: High availability setup with Sentinel and TLS
 */
function highAvailabilityExample() {
  // Sentinel nodes with TLS
  const sentinel = RedisSentinel.build([
    "rediss://sentinel1.example.com:26379",
    "rediss://sentinel2.example.com:26379",
    "rediss://sentinel3.example.com:26379"
  ]);
  
  // Redis nodes also use TLS
  const nodeConfig: SentinelNodeConfig = {
    tlsMode: "secure",
    username: "app_user",
    password: "app_password",
    db: 0
  };
  
  // Get master for writes
  const masterConn = sentinel.masterForWithConfig("production", nodeConfig);
  masterConn.set("session:12345", JSON.stringify({
    userId: 1000,
    loginTime: Date.now()
  }));
  
  // Get replica for reads
  const replicaConn = sentinel.replicaForWithConfig("production", nodeConfig);
  const session = replicaConn.get("session:12345");
  console.log("Session:", session);
}

/**
 * Example 13: Distributed cache with Cluster
 */
function distributedCacheExample() {
  const config: ClusterConfig = {
    password: "cache_password",
    readFromReplicas: true,
    connectionTimeoutMs: 2000,
    responseTimeoutMs: 1000
  };
  
  const client = RedisClusterClient.buildWithConfig(
    ["redis://cache1:7000", "redis://cache2:7000", "redis://cache3:7000"],
    config
  );
  
  const conn = client.getConnection();
  
  // Cache user data
  const userId = "user:1000";
  const userData = JSON.stringify({
    name: "John Doe",
    email: "john@example.com",
    role: "admin"
  });
  
  conn.set(userId, userData);
  conn.expire(userId, 3600);  // Cache for 1 hour
  
  // Retrieve from cache
  const cached = conn.get(userId);
  if (cached) {
    const user = JSON.parse(cached);
    console.log("User from cache:", user);
  }
}

// Helper function to read certificate files
function readCertFile(filename: string): string {
  // In real application, implement file reading logic
  // This is just a placeholder
  return "-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----";
}

