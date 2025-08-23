//! This module defines the gRPC service implementation for the key generator.
use std::sync::Arc;
use tracing::instrument;
use tonic::{async_trait, Request, Response, Status};
use rust_proto_pkg::generated::{GenerateKeyRequest, GenerateKeyResponse, PingRequest, PingResponse};
use rust_proto_pkg::generated::key_generator_service_server::KeyGeneratorService;
use crate::generator::Generator;

/// `CustomKeyGeneratorService` is the implementation of the `KeyGeneratorService` trait.
#[derive(Debug)]
pub struct CustomKeyGeneratorService {
    pub(crate) generator: Arc<dyn Generator + Send + Sync>,
}


impl CustomKeyGeneratorService {
    /// Creates a new `CustomKeyGeneratorService`.
    ///
    /// # Arguments
    ///
    /// * `generator` - The generator used for the service.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new `CustomKeyGeneratorService` or an error.
    pub async fn new(generator: Arc<dyn Generator>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { generator })
    }
}

#[async_trait]
impl KeyGeneratorService for CustomKeyGeneratorService {
    /// Handles the Ping RPC.
    #[instrument(level = "info", target = "service::ping", skip(self, _request))]
    async fn ping(
        &self,
        _request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse {
            response: "pong".into(),
        };
    
        Ok::<Response<PingResponse>, Status>(Response::new(reply))
    }

    /// Handles the GenerateKey RPC.
    #[instrument(level = "info", target = "service::generate_key", skip(self, _request))]
    async fn generate_key(&self, _request: Request<GenerateKeyRequest>) -> Result<Response<GenerateKeyResponse>, Status> {
        let key = self.generator.generate_key().await?;
        Ok(Response::new(GenerateKeyResponse{key}))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::MockGenerator;
    use crate::generator::error::GeneratorError;

    pub fn get_generator() -> CustomKeyGeneratorService {
        let generator = Arc::new(MockGenerator::new());
        CustomKeyGeneratorService { generator }
    }

    #[tokio::test]
    async fn test_ping() {
        let service = get_generator();
        let request = Request::new(PingRequest {});
        let response = service.ping(request).await.unwrap();
        assert_eq!(response.into_inner().response, "pong");
    }

    #[tokio::test]
    async fn test_generate_key_ok() {
        let mut mock_gen = MockGenerator::new();
        mock_gen.expect_generate_key().return_const(Ok("abcdef12".to_string()));
        let service = CustomKeyGeneratorService { generator: Arc::new(mock_gen) };
        let request = Request::new(GenerateKeyRequest {});
        let response = service.generate_key(request).await.unwrap();
        assert_eq!(response.into_inner().key, "abcdef12");
    }

    #[tokio::test]
    async fn test_generate_key_err() {
        let mut mock_gen = MockGenerator::new();
        mock_gen.expect_generate_key().return_const(Err(GeneratorError::ConnectionError));
        let service = CustomKeyGeneratorService { generator: Arc::new(mock_gen) };
        let request = Request::new(GenerateKeyRequest {});
        let response = service.generate_key(request).await.unwrap_err();
        assert_eq!(response.code(), tonic::Code::Unavailable);
    }
}
