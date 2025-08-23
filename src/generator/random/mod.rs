//! This module defines a key generator that produces random numbers.
use rand::Rng;
use tonic::async_trait;
use crate::generator::{max_number, GeneratorInteger};
use crate::generator::error::GeneratorError;

/// A key generator that produces random numbers.
#[derive(Clone, Debug)]
pub struct RandomGenerator;



impl RandomGenerator {
    /// Creates a new `RandomGenerator`.
    pub fn new() -> Self {
        Self {}
    }
}


#[async_trait]
impl GeneratorInteger for RandomGenerator {
    /// Generates a random number within the allowed range.
    ///
    /// # Returns
    ///
    /// A `Result` containing a random `usize` or a `GeneratorError`.
    async fn generate_key(&self) -> Result<usize, GeneratorError> {
        let mut rng = rand::rng();
        Ok(rng.random_range(0..=max_number()))
    }
}
