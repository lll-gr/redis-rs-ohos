//! HarmonyOS NAPI bindings for redis-rs
//!
//! This crate provides HarmonyOS-compatible NAPI bindings for the redis-rs library.
//! It wraps the core redis functionality with #[napi] attributes to expose it to ArkTS/JavaScript.

#![allow(dead_code)]

// Re-export logging functions
pub use crate::native_log::*;

// Modules
mod client;
mod cluster_client;
mod connection;
mod json_connection;
mod native_log;
mod sentinel_client;
mod tls_config;
mod types;

// Re-export main types
pub use client::RedisClient;
pub use cluster_client::{RedisClusterClient, RedisClusterConnection};
pub use connection::RedisConnection;
pub use json_connection::RedisJsonConnection;
pub use sentinel_client::{RedisSentinel, RedisSentinelClient};
pub use tls_config::TlsClientBuilder;
pub use types::{
    ClusterConfig, DatabaseStats, RedisClientConfig, RedisExpireOption, RedisExpireResult,
    RedisInfo, RedisValueType, SentinelNodeConfig, TlsConfig,
};