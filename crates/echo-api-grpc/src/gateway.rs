//! gRPC gateway (client adapter).
//!
//! # Rust Learning Note
//!
//! This is the **client-side adapter** - calls remote gRPC service!

use async_trait::async_trait;
use tonic::transport::Channel;
use tracing::{debug, error};

use hsu_common::Result;
use hsu_module_management::module_types::EchoService;
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
    /// Connects to a remote Echo service.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use echo_api_grpc::EchoGrpcGateway;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let gateway = EchoGrpcGateway::connect("http://localhost:50051")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn connect(addr: impl Into<String>) -> Result<Self> {
        let addr = addr.into();
        debug!("Connecting to gRPC service at: {}", addr);
        
        let client = EchoServiceClient::connect(addr)
            .await
            .map_err(|e| hsu_common::Error::Protocol(format!("Failed to connect: {}", e)))?;
        
        Ok(Self { client })
    }

    /// Calls the echo method on the remote service (public API).
    ///
    /// Note: This is now also available via the EchoService trait!
    pub async fn echo_message(&mut self, message: String) -> Result<String> {
        debug!("Calling remote echo: {}", message);
        
        let request = tonic::Request::new(EchoRequest { message });
        
        let response = self.client
            .echo(request)
            .await
            .map_err(|e| {
                error!("gRPC call failed: {}", e);
                hsu_common::Error::Protocol(format!("gRPC error: {}", e))
            })?;
        
        Ok(response.into_inner().message)
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

#[async_trait]
impl hsu_module_management::ProtocolGatewayFactory for EchoGrpcGatewayFactory {
    /// Creates an EchoGrpcGateway connected to the given address.
    ///
    /// # Rust Learning Note
    ///
    /// This is the key method that the framework calls!
    ///
    /// **Flow:**
    /// 1. Framework discovers service address from registry
    /// 2. Framework calls `factory.create_gateway(address)`
    /// 3. Factory creates EchoGrpcGateway and connects
    /// 4. Factory wraps in ServiceGateway::Grpc enum
    /// 5. Framework returns to user code
    ///
    /// **Result:** User gets a fully-functional gateway without
    /// knowing about service discovery, connection, or wrapping!
    async fn create_gateway(&self, address: String) -> Result<hsu_module_management::ServiceGateway> {
        debug!("[EchoGrpcGatewayFactory] Creating gateway for address: {}", address);
        
        // Connect to the gRPC service
        let gateway = EchoGrpcGateway::connect(address).await?;
        
        debug!("[EchoGrpcGatewayFactory] ✅ Gateway created and connected");
        
        // Wrap in the ServiceGateway enum structure
        use hsu_module_management::module_types::GrpcGateway;
        use std::sync::Arc;
        
        Ok(hsu_module_management::ServiceGateway::Grpc(
            GrpcGateway::Echo(Arc::new(gateway))
        ))
    }
}

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

