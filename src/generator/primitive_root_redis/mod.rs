//! This module defines a key generator that uses a primitive root calculation
//! combined with a Redis-based counter.
use std::error::Error;
use tonic::async_trait;
use crate::config::{PrimitiveConfig, RedisConfig};
use crate::generator::{max_number, GeneratorInteger};
use crate::generator::error::GeneratorError;
use crate::generator::redis::RedisGenerator;


/// A key generator that uses a primitive root and Redis to generate keys.
#[derive(Clone, Debug)]
pub struct PrimitiveRootRedisGenerator {
    pub(crate) redis_generator: RedisGenerator,
    primitive_config: PrimitiveConfig,
}



impl PrimitiveRootRedisGenerator {
    /// Create a new instance of `PrimitiveRootRedisGenerator`.
    ///
    /// # Arguments
    ///
    /// * `config` - Redis configuration.
    /// * `primitive_config` - Configuration for the primitive root calculation.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `PrimitiveRootRedisGenerator` or an error.
    pub fn new(config: &RedisConfig, primitive_config: &PrimitiveConfig) -> Result<Self, Box<dyn Error>> {
        let redis_generator = RedisGenerator::new(config);

        if primitive_config.prime as usize > max_number() {
            return Err("Generator prime is larger than max number".into());
        }
        
        Ok(
            Self {
                redis_generator,
                primitive_config: primitive_config.clone(),
            }
        )
    }

    /// Calculate the key using the formula: `key = (primitive_root ^ (incr + incr_start)) % prime`.
    ///
    /// # Arguments
    ///
    /// * `incr` - The increment value from Redis.
    ///
    /// # Returns
    ///
    /// The calculated key as a `usize`.
    pub fn calculate_key(&self, incr: usize) -> usize {
        let mut result = 1;
        let mut base = self.primitive_config.primitive_root;
        let mut exponent = (incr as u128 + self.primitive_config.start) % self.primitive_config.prime;
        
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % self.primitive_config.prime;
            }
            base = (base * base) % self.primitive_config.prime;
            exponent /= 2;
        }
        result as usize
    }
}

/// Generate a key using the generator.
#[async_trait]
impl GeneratorInteger for PrimitiveRootRedisGenerator {
    async fn generate_key(&self) -> Result<usize, GeneratorError> {
        let key = self.redis_generator.generate_key().await?;
        Ok(self.calculate_key(key))
    }
}
