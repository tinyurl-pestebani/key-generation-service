//! This file defines the errors that can occur within the `generator` module.
use tonic::Status;
use thiserror::Error;


/// `GeneratorError` defines the error used in the generator module.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum GeneratorError {
    /// An error occurred while connecting to a dependency, like Redis.
    #[error("Connection error")]
    ConnectionError,
    /// The requested generator was not found.
    #[error("Generator not found")]
    GeneratorNotFound,
    /// An unknown or unexpected error occurred.
    #[error("Generator unknown error: {0}")]
    UnknownError(String),
}


/// Implements the conversion from `GeneratorError` to `tonic::Status`.
/// This allows `GeneratorError` to be used as a return type in gRPC services.
impl From<GeneratorError> for Status {
    fn from(err: GeneratorError) -> Self {
        match err {
            GeneratorError::ConnectionError => Status::unavailable("Connection error"),
            GeneratorError::GeneratorNotFound => Status::not_found("Generator not found"),
            GeneratorError::UnknownError(error) => Status::internal(format!("Generator error: {error}")),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_error_conversion() {
        let connection_error = GeneratorError::ConnectionError;
        let status: Status = connection_error.into();
        assert_eq!(status.code(), tonic::Code::Unavailable);
        assert_eq!(status.message(), "Connection error");

        let not_found_error = GeneratorError::GeneratorNotFound;
        let status: Status = not_found_error.into();
        assert_eq!(status.code(), tonic::Code::NotFound);
        assert_eq!(status.message(), "Generator not found");

        let unknown_error = GeneratorError::UnknownError("Some error".to_string());
        let status: Status = unknown_error.into();
        assert_eq!(status.code(), tonic::Code::Internal);
        assert_eq!(status.message(), "Generator error: Some error");
    }
}
