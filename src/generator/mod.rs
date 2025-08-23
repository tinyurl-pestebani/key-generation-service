/// This file is part of the `generator` module, which provides functionality
/// for generating unique keys. It defines the core traits and functions for key generation.

use tonic::async_trait;
use std::fmt::Debug;

pub(crate) mod generator_object;
mod random;
mod redis;
mod primitive_root_redis;
pub(crate) mod error;

use error::GeneratorError;

#[cfg(test)]
use mockall::automock;


/// A trait for key generators that produce string-based keys.
#[cfg_attr(test, automock)]
#[async_trait]
pub trait Generator: Debug + Send + Sync {
    /// Asynchronously generates a new key.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `String` representing the generated key,
    /// or a `GeneratorError` if key generation fails.
    async fn generate_key(&self) -> Result<String, GeneratorError>;
}


/// A trait for key generators that produce integer-based keys.
#[async_trait]
pub trait GeneratorInteger {
    /// Asynchronously generates a new integer key.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `usize` representing the generated key,
    /// or a `GeneratorError` if key generation fails.
    async fn generate_key(&self) -> Result<usize, GeneratorError>;
}

/// Determines the number of digits for the generated keys based on the
/// `NUMBER_DIGITS` environment variable.
///
/// # Returns
///
/// The number of digits, defaulting to 8 if the environment variable is not set or invalid.
fn number_digits() -> usize {
    let max_number = std::env::var("NUMBER_DIGITS").unwrap_or("8".to_string());
    max_number.parse::<usize>().unwrap_or(8)
}


/// Implements the `Generator` trait for any type that implements `GeneratorInteger`.
/// This allows any integer-based generator to be used as a string-based generator
/// by converting the integer to a string.
#[async_trait]
impl <T: GeneratorInteger + Send + Sync + Debug> Generator for T {
    async fn generate_key(&self) -> Result<String, GeneratorError> {
        let number = self.generate_key().await?;
        Ok(convert_to_string(number))
    }
}

/// Calculates the maximum number that can be represented with the given number of digits
/// in base 62.
///
/// # Returns
///
/// The maximum number as a `usize`.
pub fn max_number() -> usize {
    let digits = number_digits();
    62_usize.pow(digits as u32) - 1
}

/// Converts a number to a base 62 string.
///
/// # Arguments
///
/// * `number` - The number to convert.
///
/// # Returns
///
/// A `String` representing the number in base 62.
pub fn convert_to_string(number: usize) -> String {
    let mut result = String::new();
    let mut num = number;
    let base = 62;
    let chars: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect();

    for _ in 0..number_digits() {
        let remainder = num % base;
        result.push(chars[remainder]);
        num /= base;
    }

    result.chars().rev().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_convert_to_string() {
        assert_eq!(convert_to_string(0), "00000000");
        assert_eq!(convert_to_string(1), "00000001");
        assert_eq!(convert_to_string(61), "0000000z");
        assert_eq!(convert_to_string(62), "00000010");
        assert_eq!(convert_to_string(63), "00000011");
        assert_eq!(convert_to_string(12345678), "0000pnfq");
    }

    #[tokio::test]
    async fn test_max_number() {
        assert_eq!(max_number(), 62_usize.pow(8_u32) - 1);
    }
}
