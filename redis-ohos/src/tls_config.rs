// TLS Configuration for HarmonyOS NAPI

use napi_derive_ohos::napi;
use napi_ohos::bindgen_prelude::*;
use redis::{Client, TlsCertificates, ClientTlsConfig};

use crate::types::TlsConfig;

/// TLS Client Builder for HarmonyOS
///
/// This class provides methods to create Redis clients with TLS/SSL support
/// and custom certificate configurations.
#[napi]
pub struct TlsClientBuilder {
    url: String,
    tls_config: Option<TlsConfig>,
}

#[napi]
impl TlsClientBuilder {
    /// Create a new TLS client builder
    ///
    /// # Arguments
    /// * `url` - Redis connection URL (must use rediss:// scheme for TLS)
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// import { TlsClientBuilder } from 'libredis_ohos.so';
    ///
    /// const builder = new TlsClientBuilder("rediss://127.0.0.1:6380");
    /// ```
    #[napi(constructor)]
    pub fn new(url: String) -> Result<Self> {
        if !url.starts_with("rediss://") {
            return Err(napi_ohos::Error::from_reason(
                "TLS client requires URL with rediss:// scheme".to_string(),
            ));
        }
        Ok(TlsClientBuilder {
            url,
            tls_config: None,
        })
    }

    /// Set TLS certificates configuration
    ///
    /// # Arguments
    /// * `config` - TLS configuration with certificates
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const tlsConfig = {
    ///   clientCert: readFileAsString("client-cert.pem"),
    ///   clientKey: readFileAsString("client-key.pem"),
    ///   rootCert: readFileAsString("ca-cert.pem")
    /// };
    /// builder.withTlsConfig(tlsConfig);
    /// ```
    #[napi]
    pub fn with_tls_config(&mut self, config: TlsConfig) -> Result<()> {
        self.tls_config = Some(config);
        Ok(())
    }

    /// Build the Redis client with TLS configuration
    ///
    /// # Returns
    /// A Redis client configured with TLS
    ///
    /// # Example (ArkTS)
    /// ```typescript
    /// const client = builder.build();
    /// const conn = client.getConnection();
    /// ```
    #[napi]
    pub fn build(&self) -> Result<crate::client::RedisClient> {
        let client = if let Some(ref tls_config) = self.tls_config {
            // Build TlsCertificates from config
            let client_tls = if let (Some(ref cert), Some(ref key)) = 
                (&tls_config.client_cert, &tls_config.client_key) {
                Some(ClientTlsConfig {
                    client_cert: cert.as_bytes().to_vec(),
                    client_key: key.as_bytes().to_vec(),
                })
            } else {
                None
            };

            let root_cert = tls_config.root_cert.as_ref().map(|cert| cert.as_bytes().to_vec());

            let certificates = TlsCertificates {
                client_tls,
                root_cert,
            };

            // Parse the URL to get connection info
            let connection_info = self.url.parse().map_err(|e: redis::RedisError| {
                napi_ohos::Error::from_reason(format!("Failed to parse URL: {}", e))
            })?;

            // Build client with TLS
            redis::tls::inner_build_with_tls(connection_info, &certificates).map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to build TLS client: {}", e))
            })?
        } else {
            // Build without custom certificates (uses system trust store)
            Client::open(self.url.as_str()).map_err(|e| {
                napi_ohos::Error::from_reason(format!("Failed to create Redis client: {}", e))
            })?
        };

        Ok(crate::client::RedisClient::from_inner(client))
    }
}

