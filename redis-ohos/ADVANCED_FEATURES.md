# Advanced Features Guide

This guide covers the advanced features of redis-ohos including TLS/SSL, Sentinel, and Cluster support.

## Table of Contents

- [TLS/SSL Support](#tlsssl-support)
- [Redis Sentinel](#redis-sentinel)
- [Redis Cluster](#redis-cluster)
- [Configuration Reference](#configuration-reference)

## TLS/SSL Support

Redis-ohos supports secure connections using TLS/SSL encryption.

### Basic TLS Connection

Use the `rediss://` scheme for TLS connections:

```typescript
import { RedisClient } from 'libredis_ohos.so';

const client = new RedisClient("rediss://127.0.0.1:6380");
const conn = client.getConnection();
```

### TLS with Custom Certificates (mTLS)

For mutual TLS authentication with custom certificates:

```typescript
import { TlsClientBuilder, TlsConfig } from 'libredis_ohos.so';

const tlsConfig: TlsConfig = {
  clientCert: readFileAsString("client-cert.pem"),
  clientKey: readFileAsString("client-key.pem"),
  rootCert: readFileAsString("ca-cert.pem")
};

const builder = new TlsClientBuilder("rediss://127.0.0.1:6380");
builder.withTlsConfig(tlsConfig);

const client = builder.build();
const conn = client.getConnection();
```

### TLS with Authentication

Combine TLS with username/password authentication:

```typescript
const client = new RedisClient("rediss://username:password@redis.example.com:6380/0");
```

Or using configuration:

```typescript
const config = {
  host: "redis.example.com",
  port: 6380,
  username: "myuser",
  password: "mypass",
  useTls: true,
  db: 0
};

const client = RedisClient.fromConfig(config);
```

## Redis Sentinel

Redis Sentinel provides high availability through automatic failover. When a master fails, Sentinel promotes a replica to master.

### Basic Sentinel Usage

```typescript
import { RedisSentinel } from 'libredis_ohos.so';

// Connect to Sentinel nodes
const sentinel = RedisSentinel.build([
  "redis://127.0.0.1:26379",
  "redis://127.0.0.1:26380",
  "redis://127.0.0.1:26381"
]);

// Get connection to master (for writes)
const masterConn = sentinel.masterFor("mymaster");
masterConn.set("key", "value");

// Get connection to replica (for reads)
const replicaConn = sentinel.replicaFor("mymaster");
const value = replicaConn.get("key");
```

### Sentinel with TLS and Authentication

```typescript
import { RedisSentinel, SentinelNodeConfig } from 'libredis_ohos.so';

// Sentinel nodes with authentication
const sentinel = RedisSentinel.build([
  "redis://sentinel_user:sentinel_pass@127.0.0.1:26379"
]);

// Configure connection to Redis nodes
const nodeConfig: SentinelNodeConfig = {
  tlsMode: "secure",  // "secure", "insecure", or "none"
  username: "redis_user",
  password: "redis_pass",
  db: 1
};

const masterConn = sentinel.masterForWithConfig("mymaster", nodeConfig);
const replicaConn = sentinel.replicaForWithConfig("mymaster", nodeConfig);
```

### Sentinel Client (Simplified Interface)

For applications that always connect to the same type of node:

```typescript
import { RedisSentinelClient } from 'libredis_ohos.so';

// Client that always connects to master
const masterClient = RedisSentinelClient.buildMaster(
  ["redis://127.0.0.1:26379"],
  "mymaster"
);

// Client that always connects to replicas
const replicaClient = RedisSentinelClient.buildReplica(
  ["redis://127.0.0.1:26379"],
  "mymaster"
);

// Use like regular clients
const masterConn = masterClient.getConnection();
const replicaConn = replicaClient.getConnection();
```

### Sentinel Client with Configuration

```typescript
const config: SentinelNodeConfig = {
  tlsMode: "secure",
  password: "mypassword",
  db: 0
};

const client = RedisSentinelClient.buildWithConfig(
  ["redis://127.0.0.1:26379"],
  "mymaster",
  "master",  // or "replica"
  config
);
```

## Redis Cluster

Redis Cluster provides automatic sharding across multiple nodes for horizontal scaling.

### Basic Cluster Usage

```typescript
import { RedisClusterClient } from 'libredis_ohos.so';

// Connect to cluster (only need one node, but can specify multiple)
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
```

### Cluster with Configuration

```typescript
import { RedisClusterClient, ClusterConfig } from 'libredis_ohos.so';

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
```

### Cluster with TLS

```typescript
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
```

### Cluster Operations

The cluster connection supports common Redis commands:

```typescript
const conn = client.getConnection();

// String operations
conn.set("key", "value");
const value = conn.get("key");

// Numeric operations
conn.incr("counter");
conn.decr("counter");

// Key operations
conn.expire("key", 3600);  // Expire in 1 hour
const ttl = conn.ttl("key");
const exists = conn.exists(["key1", "key2"]);
conn.del(["key1", "key2"]);
```

## Configuration Reference

### TlsConfig

```typescript
interface TlsConfig {
  clientCert?: string;   // Client certificate in PEM format (for mTLS)
  clientKey?: string;    // Client private key in PEM format (for mTLS)
  rootCert?: string;     // Root CA certificate in PEM format
}
```

### SentinelNodeConfig

```typescript
interface SentinelNodeConfig {
  tlsMode?: string;      // "none", "secure", or "insecure"
  username?: string;     // Username for authentication
  password?: string;     // Password for authentication
  db?: number;          // Database index
}
```

### ClusterConfig

```typescript
interface ClusterConfig {
  username?: string;              // Username for authentication
  password?: string;              // Password for authentication
  readFromReplicas?: boolean;     // Read from replica nodes (default: false)
  connectionTimeoutMs?: number;   // Connection timeout in milliseconds
  responseTimeoutMs?: number;     // Response timeout in milliseconds
  useTls?: boolean;              // Use TLS/SSL connection
  tlsMode?: string;              // "secure" or "insecure"
}
```

## Best Practices

### High Availability Setup

For production environments, use Sentinel with TLS:

```typescript
const sentinel = RedisSentinel.build([
  "rediss://sentinel1.example.com:26379",
  "rediss://sentinel2.example.com:26379",
  "rediss://sentinel3.example.com:26379"
]);

const nodeConfig: SentinelNodeConfig = {
  tlsMode: "secure",
  username: "app_user",
  password: "app_password",
  db: 0
};

// Write to master
const masterConn = sentinel.masterForWithConfig("production", nodeConfig);
masterConn.set("key", "value");

// Read from replica
const replicaConn = sentinel.replicaForWithConfig("production", nodeConfig);
const value = replicaConn.get("key");
```

### Distributed Caching

For distributed caching, use Cluster with read from replicas:

```typescript
const config: ClusterConfig = {
  password: "cache_password",
  readFromReplicas: true,
  connectionTimeoutMs: 2000,
  responseTimeoutMs: 1000
};

const client = RedisClusterClient.buildWithConfig(
  ["redis://cache1:7000", "redis://cache2:7000"],
  config
);

const conn = client.getConnection();
conn.set("cache:key", "value");
conn.expire("cache:key", 3600);  // Cache for 1 hour
```

### Security Considerations

1. **Always use TLS in production** - Use `rediss://` scheme
2. **Enable authentication** - Set username and password
3. **Use mTLS for sensitive data** - Provide client certificates
4. **Rotate credentials regularly** - Update passwords periodically
5. **Use separate credentials** - Different credentials for Sentinel and Redis nodes
6. **Limit network access** - Use firewalls to restrict Redis access

## Troubleshooting

### TLS Connection Issues

- Verify certificate paths and formats (PEM)
- Check certificate validity dates
- Ensure hostname matches certificate CN/SAN
- Use `tlsMode: "insecure"` for self-signed certs (development only)

### Sentinel Connection Issues

- Verify Sentinel nodes are running and accessible
- Check Sentinel configuration for master name
- Ensure Redis nodes are properly configured in Sentinel
- Verify authentication credentials for both Sentinel and Redis

### Cluster Connection Issues

- Verify at least one cluster node is accessible
- Check cluster configuration with `CLUSTER INFO` command
- Ensure all cluster nodes are in healthy state
- Verify network connectivity between cluster nodes

## Examples

See `examples/advanced_usage.ets` for complete working examples of all features.

