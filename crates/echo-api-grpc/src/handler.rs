//! gRPC handler adapter.
//!
//! # Rust Learning Note
//!
//! This demonstrates the **Adapter Pattern**!
//!
//! ## Architecture
//!
//! ```
//! gRPC Client
//!     ↓
//! tonic Generated Server Trait
//!     ↓
//! EchoGrpcHandler (THIS ADAPTER)
//!     ↓
//! EchoServiceImpl (DOMAIN)
//! ```
//!
//! **Key insight:** Domain code doesn't know about gRPC!

use tonic::{Request, Response, Status};
use std::sync::Arc;
use tracing::{debug, error};

use echo_contract::EchoService;
#[cfg(test)]
use echo_domain::EchoServiceImpl;
use crate::generated::{EchoRequest, EchoResponse, echo_service_server::EchoService as EchoServiceTrait};

/// gRPC handler adapter for Echo service.
///
/// # Rust Learning Note
///
/// ## Adapter Pattern
///
/// ```rust
/// pub struct EchoGrpcHandler {
///     service: Arc<EchoServiceImpl>,  // Domain service
/// }
///
/// #[tonic::async_trait]
/// impl EchoServiceTrait for EchoGrpcHandler {
///     // gRPC interface
///     async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
///         // Extract data from gRPC request
///         let message = request.into_inner().message;
///         
///         // Call domain service
///         let result = self.service.echo(message).await?;
///         
///         // Wrap in gRPC response
///         Ok(Response::new(EchoResponse { message: result }))
///     }
/// }
/// ```
///
/// **Separation of concerns:**
/// - gRPC layer: Protocol details
/// - Domain layer: Business logic
pub struct EchoGrpcHandler {
    service: Arc<dyn EchoService>,
}

impl EchoGrpcHandler {
    /// Creates a new gRPC handler.
    ///
    /// Accepts any implementation of `EchoService` trait, enabling
    /// flexibility in the visitor pattern and handler registration.
    pub fn new(service: Arc<dyn EchoService>) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl EchoServiceTrait for EchoGrpcHandler {
    /// Handles Echo gRPC requests.
    ///
    /// # Rust Learning Note
    ///
    /// ## Error Conversion
    ///
    /// ```rust
    /// // Domain error
    /// let result = self.service.echo(message).await?;
    ///                                               ^
    ///                                               |
    /// // Needs to convert to tonic::Status
    /// ```
    ///
    /// **Solution:** Implement `From<Error> for Status`
    async fn echo(
        &self,
        request: Request<EchoRequest>,
    ) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        debug!("gRPC Echo request: {}", message);

        // Call domain service
        let result = self.service
            .echo(message)
            .await
            .map_err(|e| {
                error!("Echo service error: {}", e);
                Status::internal(format!("Service error: {}", e))
            })?;

        Ok(Response::new(EchoResponse { message: result }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_grpc_handler() {
        let service = Arc::new(EchoServiceImpl::new());
        let handler = EchoGrpcHandler::new(service);
        
        let request = Request::new(EchoRequest {
            message: "Hello via gRPC!".to_string(),
        });
        
        let response = handler.echo(request).await.unwrap();
        assert_eq!(response.into_inner().message, "Hello via gRPC!");
    }
}

