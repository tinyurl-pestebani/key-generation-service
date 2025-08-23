//! This is the main entry point for the key generation service.
//! It sets up the server, configures tracing and logging, and starts the
//! gRPC service.

use tonic::transport::Server;
use tokio::{time::Duration, time};
use tracing::info;
use rust_otel_setup::otel::OpenTelemetryObject;
use rust_otel_setup::config as otel_config;
use rust_proto_pkg::generated::key_generator_service_server::KeyGeneratorServiceServer;
use tonic_tracing_opentelemetry::middleware::server::OtelGrpcLayer;
use crate::generator::generator_object::new_key_generation_layer;

mod generator;
mod service;
mod config;


// grpcurl  -plaintext -d '{}' -proto v1/key-generator.proto  localhost:8080 tinyurl.v1.KeyGeneratorService/Ping
// grpcurl  -plaintext -d '{}' -proto key-generator.proto  localhost:8080 tinyurl.v1.KeyGeneratorService/GenerateKey


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::GenerationKeyServiceConfig::from_env()?;

    let generator = new_key_generation_layer(&config.generator_config).await?;
    let generator_service = service::CustomKeyGeneratorService::new(generator).await?;

    let otl_object = OpenTelemetryObject::new(&otel_config::LogConfig::from_env()?, &otel_config::TraceConfig::from_env()?, "key-generation-service".into()).await?;

    let addr = format!("[::]:{}", config.listen_port).parse()?;
    info!("stating server on {addr}");
    let gs = KeyGeneratorServiceServer::new(generator_service);

    Server::builder()
        .layer(OtelGrpcLayer::default())
        .add_service(gs)
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");
            time::sleep(Duration::from_secs(1)).await;
            otl_object.stop().unwrap();
        })
        .await?;
    Ok(())
}
