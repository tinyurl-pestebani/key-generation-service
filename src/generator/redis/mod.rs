//! This module defines a Redis-based key generator that increments a counter
//! in Redis to produce unique keys.

use std::sync::Arc;
use redis::Client;
use tonic::async_trait;
use crate::config::RedisConfig;
use crate::generator::error::GeneratorError;
use crate::generator::GeneratorInteger;

/// `RedisGenerator` generates keys by incrementing a Redis counter.
#[derive(Clone, Debug)]
pub struct RedisGenerator {
    /// A thread-safe pool of Redis clients.
    pub(crate) pool: Arc<Client>,
}


impl RedisGenerator {
    /// Creates a new `RedisGenerator`.
    ///
    /// # Arguments
    ///
    /// * `config` - The Redis configuration.
    pub fn new(config: &RedisConfig) -> Self {
        let client = Client::open(config.url.clone()).unwrap();
        Self {
            pool: Arc::new(client),
        }
    }
}


#[async_trait]
impl GeneratorInteger for RedisGenerator {
    /// Asynchronously generates a key by incrementing the "incr:count" counter in Redis.
    ///
    /// # Returns
    ///
    /// A `Result` which is either the new integer key or a `GeneratorError`.
    async fn generate_key(&self) -> Result<usize, GeneratorError> {
        let con = self.pool.clone();
        let mut cn: Client = (*con).clone();
        let res = redis::cmd("INCR").arg("incr:count").query(&mut cn).map_err(|err| {
            // TODO: Implement retries policies
            if err.is_timeout() || err.is_connection_refusal() || err.is_connection_dropped() {
                GeneratorError::ConnectionError
            } else {
                GeneratorError::UnknownError(err.to_string())
            }
        })?;
        Ok(res)
    }
}
