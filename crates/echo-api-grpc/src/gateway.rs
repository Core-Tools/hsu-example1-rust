//! gRPC gateway (client adapter).
//!
//! # Rust Learning Note
//!
//! This is the **client-side adapter** - calls remote gRPC service!

use async_trait::async_trait;
use tonic::transport::Channel;
use tracing::{debug, error};

use hsu_common::Result;
use echo_contract::EchoService;
use crate::generated::{EchoRequest, echo_service_client::EchoServiceClient};

/// gRPC gateway for calling remote Echo service.
///
/// # Rust Learning Note
///
/// ## Client Adapter
///
/// ```
/// Domain Code
///     ↓
/// EchoGrpcGateway (THIS ADAPTER)
///     ↓
/// tonic Generated Client
///     ↓
/// gRPC Network
/// ```
pub struct EchoGrpcGateway {
    client: EchoServiceClient<Channel>,
}

impl EchoGrpcGateway {
    /// Creates a gateway from an existing client.
    ///
    /// # Rust Learning Note
    ///
    /// This is the **only** way to create an `EchoGrpcGateway`.
    /// Used by `ServiceGatewayFactory<C>` and the old `EchoGrpcGatewayFactory`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let channel = tonic::transport::Channel::from_static("http://localhost:50051")
    ///     .connect()
    ///     .await?;
    /// let client = EchoServiceClient::new(channel);
    /// let gateway = EchoGrpcGateway::from_client(client);
    /// ```
    pub fn from_client(client: EchoServiceClient<Channel>) -> Self {
        Self { client }
    }
}

/// Implement the EchoService trait for EchoGrpcGateway.
///
/// # Rust Learning Note
///
/// This implementation allows EchoGrpcGateway to be used anywhere
/// an EchoService is expected - providing a uniform interface regardless
/// of whether it's a local service or a remote gRPC call!
///
/// ## Challenge: &self vs &mut self
///
/// The EchoService trait uses `&self`, but tonic's client needs `&mut self`.
/// We solve this using interior mutability with `tokio::sync::Mutex`.
///
/// **Without Mutex:**
/// ```rust,ignore
/// pub async fn echo(&self, message: String) -> Result<String> {
///     self.client.echo(...)  // ❌ Error: needs &mut self
/// }
/// ```
///
/// **With Mutex:**
/// ```rust,ignore
/// client: tokio::sync::Mutex<EchoServiceClient<Channel>>
/// 
/// pub async fn echo(&self, message: String) -> Result<String> {
///     let mut client = self.client.lock().await;
///     client.echo(...)  // ✅ Works!
/// }
/// ```
///
/// This is a common pattern in async Rust!
#[async_trait]
impl EchoService for EchoGrpcGateway {
    async fn echo(&self, message: String) -> Result<String> {
        debug!("[EchoGrpcGateway] EchoService trait call: {}", message);
        
        let request = tonic::Request::new(EchoRequest { message });
        
        // Clone the client - tonic clients are cheap to clone
        // (they use Arc internally)
        let mut client = self.client.clone();
        
        let response = client
            .echo(request)
            .await
            .map_err(|e| {
                error!("gRPC call failed: {}", e);
                hsu_common::Error::Protocol(format!("gRPC error: {}", e))
            })?;
        
        Ok(response.into_inner().message)
    }
}

/// Factory for creating EchoGrpcGateway instances.
///
/// # Rust Learning Note
///
/// This factory implements the `ProtocolGatewayFactory` trait,
/// matching Go's `GatewayFactoryFunc` pattern!
///
/// **Go equivalent:**
/// ```go
/// func NewGRPCGateway1(
///     clientConnection moduleproto.ProtocolClientConnection,
///     logger logging.Logger,
/// ) moduletypes.ServiceGateway {
///     grpcClient := proto.NewEchoServiceClient(clientConnection)
///     return &grpcGateway1{grpcClient: grpcClient, logger: logger}
/// }
/// ```
///
/// **Rust version:**
/// ```rust
/// #[async_trait]
/// impl ProtocolGatewayFactory for EchoGrpcGatewayFactory {
///     async fn create_gateway(&self, address: String) -> Result<ServiceGateway> {
///         let gateway = EchoGrpcGateway::connect(address).await?;
///         Ok(ServiceGateway::Grpc(GrpcGateway::Echo(Arc::new(gateway))))
///     }
/// }
/// ```
///
/// ## Usage
///
/// ```rust,ignore
/// use hsu_module_management::{GatewayConfig, ProtocolGatewayFactory};
/// use echo_api_grpc::EchoGrpcGatewayFactory;
///
/// let config = GatewayConfig::new(ServiceID::from("echo-service"), Protocol::Grpc)
///     .with_factory(Arc::new(EchoGrpcGatewayFactory));
/// ```
pub struct EchoGrpcGatewayFactory;

impl EchoGrpcGatewayFactory {
    /// Creates a new factory instance.
    pub fn new() -> Self {
        Self
    }
}

impl Default for EchoGrpcGatewayFactory {
    fn default() -> Self {
        Self::new()
    }
}

// DEPRECATED: Old trait implementation removed
// Use ServiceGatewayFactory<C> pattern instead (see echo-api crate)

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests require a running gRPC server
    // These are unit tests that just verify compilation
    
    #[test]
    fn test_gateway_compiles() {
        // Just verify the types compile
        let _ = std::marker::PhantomData::<EchoGrpcGateway>;
    }
    
    #[test]
    fn test_factory_creation() {
        let factory = EchoGrpcGatewayFactory::new();
        let _ = factory; // Use it
        
        let factory2 = EchoGrpcGatewayFactory::default();
        let _ = factory2;
    }
}

