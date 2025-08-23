//! This module defines the configuration structures for the key generation service.
//! It includes configurations for the service itself, different types of key generators,
//! logging, and tracing. The configurations are loaded from environment variables.

use std::env;
use anyhow::{anyhow, Result};

/// `GenerationKeyServiceConfig` holds the main configuration for the service.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenerationKeyServiceConfig {
    /// The port on which the gRPC server will listen.
    pub listen_port: u16,
    /// The configuration for the chosen key generator.
    pub generator_config: GeneratorConfig,
}


/// `GeneratorConfig` defines the different types of key generators available.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeneratorConfig {
    /// A generator that produces random keys.
    Random,
    /// A generator that uses Redis to produce incremental keys.
    Redis(RedisConfig),
    /// A generator that uses a primitive root calculation with Redis.
    PrimitiveRootRedis(RedisConfig, PrimitiveConfig),
}

/// `RedisConfig` holds the configuration for connecting to Redis.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RedisConfig {
    /// The URL of the Redis server.
    pub url: String,
}

/// `PrimitiveConfig` holds the configuration for the primitive root generator.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrimitiveConfig {
    /// The prime number to use in the calculation.
    pub prime: u128,
    /// The starting value for the increment.
    pub start: u128,
    /// The primitive root to use in the calculation.
    pub primitive_root: u128,
}

/// `LokiConfig` holds the configuration for connecting to Loki.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LokiConfig {
    /// The URL of the Loki server.
    pub url: String,
}

/// `OTLPTraceConfig` holds the configuration for OTLP tracing.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OTLPTraceConfig {
    /// The endpoint of the OTLP collector.
    pub endpoint: String,
}


impl PrimitiveConfig {
    /// Creates a new `PrimitiveConfig` from environment variables.
    ///
    /// # Returns
    ///
    /// Returns an error if the required environment variables are not set
    /// or if they contain invalid values, otherwise a `PrimitiveConfig`.
    pub fn from_env() -> Result<Self> {
        let prime = env::var("GENERATOR_PRIME")
            .unwrap_or_else(|_| "1000003".to_string())
            .parse::<u128>()
            .map_err(|_| anyhow!("Invalid prime value"))?;

        let start = env::var("GENERATOR_INCREMENT_START")
            .unwrap_or_else(|_| "0".to_string())
            .parse::<u128>()
            .map_err(|_| anyhow!("Invalid increment start value"))?;

        let primitive_root = env::var("GENERATOR_PRIME_PRIMITIVE")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u128>()
            .map_err(|_| anyhow!("Invalid primitive root value"))?;

        Ok(PrimitiveConfig {
            prime,
            start,
            primitive_root,
        })
    }
}


impl RedisConfig {
    /// Creates a new `RedisConfig` from environment variables.
    ///
    /// # Returns
    ///
    /// Returns an error if the `REDIS_URL` environment variable is not set
    /// otherwise a `RedisConfig`.

    pub fn from_env() -> Result<Self> {
        Ok(RedisConfig {
            url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        })
    }
}


impl GeneratorConfig {
    /// Creates a new `GeneratorConfig` from environment variables.
    ///
    /// # Returns
    ///
    /// Returns an error if the required environment variables are not set
    /// or if they contain invalid values, otherwise a `GENERATOR_TYPE`.
    pub fn from_env() -> Result<Self> {
        let generator_type = env::var("GENERATOR_TYPE").unwrap_or_else(|_| "random".to_string());
        match generator_type.as_str() {
            "random" => Ok(GeneratorConfig::Random),
            "redis" => Ok(GeneratorConfig::Redis(RedisConfig::from_env()?)),
            "primitive_root_redis" => Ok(GeneratorConfig::PrimitiveRootRedis(
                RedisConfig::from_env()?,
                PrimitiveConfig::from_env()?,
            )),
            _ => Err(anyhow!("Unsupported generator type: {}", generator_type)),
        }
    }
}


impl GenerationKeyServiceConfig {
    /// Creates a new `GenerationKeyServiceConfig` from environment variables.
    ///
    /// # Returns
    ///
    /// Returns an error if the required environment variables are not set
    /// or if they contain invalid values, otherwise a `GenerationKeyServiceConfig`.
    pub fn from_env() -> Result<Self> {
        let listen_port = env::var("GENERATION_KEY_SERVICE_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        let generator_config = GeneratorConfig::from_env()?;

        Ok(GenerationKeyServiceConfig {
            listen_port,
            generator_config,
        })
    }
}

