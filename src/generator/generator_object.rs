//! This module provides a factory function for creating key generator instances.
use std::error::Error;
use std::sync::Arc;
use crate::config::GeneratorConfig;
use crate::generator::Generator;
use crate::generator::random::RandomGenerator;
use crate::generator::redis::RedisGenerator;
use crate::generator::primitive_root_redis::PrimitiveRootRedisGenerator;


/// Creates a new key generation layer based on the provided configuration.
///
/// # Arguments
///
/// * `config` - The configuration specifying which generator to create.
///
/// # Returns
///
/// A `Result` containing a thread-safe `Arc` of a `Generator` trait object,
/// or an error if the generator cannot be created.
pub async fn new_key_generation_layer(config: &GeneratorConfig) -> Result<Arc<dyn Generator>, Box<dyn Error>> {
    match config { 
        GeneratorConfig::Random => {
            let generator = RandomGenerator::new();
            Ok(Arc::new(generator))
        },
        GeneratorConfig::Redis(redis_config) => {
            let generator = RedisGenerator::new(redis_config);
            Ok(Arc::new(generator))
        },
        GeneratorConfig::PrimitiveRootRedis(redis_config, primitive_config) => {
            let generator = PrimitiveRootRedisGenerator::new(redis_config, primitive_config)?;
            Ok(Arc::new(generator))
        },
        // Add other generator configurations here
    }
}
