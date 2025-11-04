//! gRPC server helpers for Echo service.
//!
//! This module provides utilities to create a gRPC server
//! from an EchoService implementation.

use std::sync::Arc;
use tonic::transport::Server;
use tokio::sync::oneshot;
use tracing::{info, error};

use echo_domain::EchoServiceImpl;
use crate::{EchoGrpcHandler, generated::echo_service_server::EchoServiceServer};

/// Creates and runs an Echo gRPC server.
///
/// # Arguments
/// * `service` - The echo service implementation
/// * `addr` - Address to listen on (e.g., "0.0.0.0:50051")
/// * `shutdown_rx` - Receive channel for graceful shutdown
///
/// # Returns
/// The actual address the server is listening on (useful when port is 0)
pub async fn run_echo_grpc_server(
    service: Arc<EchoServiceImpl>,
    addr: impl Into<String>,
    mut shutdown_rx: oneshot::Receiver<()>,
) -> Result<String, Box<dyn std::error::Error>> {
    let addr_str = addr.into();
    let addr = addr_str.parse()?;

    info!("Starting Echo gRPC server on: {}", addr_str);

    // Create gRPC handler and service
    let handler = EchoGrpcHandler::new(service);
    let grpc_service = EchoServiceServer::new(handler);

    // Build and run server with graceful shutdown
    Server::builder()
        .add_service(grpc_service)
        .serve_with_shutdown(addr, async move {
            shutdown_rx.await.ok();
            info!("Echo gRPC server shutting down");
        })
        .await?;

    Ok(addr_str)
}

/// Helper to start a background gRPC server.
///
/// Returns a shutdown sender that can be used to stop the server.
pub fn spawn_echo_grpc_server(
    service: Arc<EchoServiceImpl>,
    addr: impl Into<String> + Send + 'static,
) -> oneshot::Sender<()> {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    tokio::spawn(async move {
        if let Err(e) = run_echo_grpc_server(service, addr, shutdown_rx).await {
            error!("gRPC server error: {}", e);
        }
    });

    shutdown_tx
}

