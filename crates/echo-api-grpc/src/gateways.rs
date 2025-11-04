//! Echo Service Gateways - Protocol-Agnostic Service Access
//!
//! # Architecture
//!
//! This module provides `EchoServiceGateways` - a high-level interface for
//! accessing Echo services without knowing about protocols!
//!
//! ## The New Pattern (Phase 2)
//!
//! We now use `ServiceGatewayFactory<C>` to create typed gateways:
//!
//! ```text
//! Client Code
//!     ↓
//! EchoServiceGateways.get_service1(Protocol::Auto)
//!     ↓
//! ServiceGatewayFactory<dyn EchoService>::new_service_gateway(protocol)
//!     ↓
//! ServiceConnector.connect(protocol, visitor)
//!     ↓
//! Visitor creates typed gateway
//!     ↓
//! Arc<dyn EchoService> returned (no protocol matching!)
//! ```
//!
//! ## Comparison with Go
//!
//! **Go version:**
//! ```go
//! type EchoServiceGateways interface {
//!     GetService1(ctx context.Context, protocol Protocol) (echocontract.Service1, error)
//!     GetService2(ctx context.Context, protocol Protocol) (echocontract.Service2, error)
//! }
//!
//! func (g *echoServiceGateways) GetService1(ctx context.Context, protocol Protocol) (echocontract.Service1, error) {
//!     factory := modulewiring.ServiceGatewayFactory[echocontract.Service1]{
//!         ModuleID:         g.targetModuleID,
//!         ServiceID:        "service1",
//!         ServiceConnector: g.serviceConnector,
//!         GatewayFactoryFuncs: modulewiring.GatewayFactoryFuncs[echocontract.Service1]{
//!             Direct: directFunc,
//!             GRPC:   grpcFunc,
//!         },
//!     }
//!     return factory.NewServiceGateway(ctx, protocol)
//! }
//! ```
//!
//! **Rust version (this file):**
//! ```rust
//! pub trait EchoServiceGateways: Send + Sync {
//!     async fn get_service1(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>>;
//!     async fn get_service2(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>>;
//! }
//!
//! impl EchoServiceGateways for EchoServiceGatewaysImpl {
//!     async fn get_service1(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>> {
//!         let factory = ServiceGatewayFactory::<dyn EchoService>::new(
//!             self.module_id.clone(),
//!             ServiceID::from("service1"),
//!             self.service_connector.clone(),
//!             GatewayFactoryFuncs {
//!                 direct: direct_func,
//!                 grpc: grpc_func,
//!                 http: None,
//!             },
//!         );
//!         factory.new_service_gateway(protocol).await
//!     }
//! }
//! ```
//!
//! Nearly identical! Main differences:
//! - Rust: `async fn` (Go uses context)
//! - Rust: `Arc<dyn EchoService>` (Go uses interface directly)

use std::sync::Arc;
use async_trait::async_trait;
use hsu_common::{ModuleID, ServiceID, Protocol, Result};
use hsu_module_api::{ServiceConnector, ServiceGatewayFactory, GatewayFactoryFuncs};
use hsu_module_management::module_types::EchoService;
use tracing::debug;

use crate::gateway::EchoGrpcGateway;

/// Trait for accessing Echo services in a protocol-agnostic way.
///
/// This trait provides methods to get service gateways without knowing
/// about the underlying protocol. The framework handles protocol selection,
/// connection establishment, and gateway creation automatically!
///
/// # Example Usage
///
/// ```rust,ignore
/// // Create gateways provider
/// let gateways = EchoServiceGatewaysImpl::new(
///     ModuleID::from("echo"),
///     service_connector,
/// );
///
/// // Get typed service (protocol-agnostic!)
/// let service1 = gateways.get_service1(Protocol::Auto).await?;
///
/// // Use the service (no protocol knowledge!)
/// let response = service1.echo("Hello".to_string()).await?;
/// ```
#[async_trait]
pub trait EchoServiceGateways: Send + Sync {
    /// Returns the target module ID.
    fn module_id(&self) -> ModuleID;
    
    /// Returns the list of service IDs this gateway provider knows about.
    fn service_ids(&self) -> Vec<ServiceID>;
    
    /// Enables direct closure for the services.
    ///
    /// When direct closure is enabled, the services will store handlers
    /// that can be called directly (in-process) without network communication.
    ///
    /// This is typically called during module initialization.
    ///
    /// # Rust Learning Note
    ///
    /// Takes `&self` instead of `&mut self` because implementations use
    /// interior mutability (`RwLock`) to allow mutation through shared references.
    /// This is necessary for trait objects (`Arc<dyn EchoServiceGateways>`).
    fn enable_direct_closure(&self, handlers: EchoServiceHandlers);
    
    /// Gets a gateway for service1.
    ///
    /// # Arguments
    ///
    /// * `protocol` - Desired protocol (Auto, Direct, gRPC, HTTP)
    ///
    /// # Returns
    ///
    /// Returns a typed service gateway that implements `EchoService`.
    /// The caller doesn't need to know which protocol was selected!
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Get service with auto protocol selection
    /// let service = gateways.get_service1(Protocol::Auto).await?;
    ///
    /// // Use it (protocol-agnostic!)
    /// let response = service.echo("Hello".to_string()).await?;
    /// ```
    async fn get_service1(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>>;
    
    /// Gets a gateway for service2.
    async fn get_service2(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>>;
}

/// Service handlers for direct (in-process) calls.
///
/// When direct closure is enabled, these handlers are stored and used
/// for Protocol::Direct or Protocol::Auto (when available locally).
#[derive(Clone)]
pub struct EchoServiceHandlers {
    pub service1: Arc<dyn EchoService>,
    pub service2: Arc<dyn EchoService>,
}

/// Implementation of EchoServiceGateways.
///
/// # Rust Learning Note
///
/// ## Interior Mutability Pattern
///
/// We use `RwLock<Option<EchoServiceHandlers>>` for the handlers because:
/// - The trait methods take `&self` (immutable reference)
/// - But `enable_direct_closure()` needs to mutate the handlers
///
/// ```rust
/// pub struct EchoServiceGatewaysImpl {
///     service_handlers: std::sync::RwLock<Option<EchoServiceHandlers>>,
///     // ...
/// }
/// ```
///
/// **RwLock allows:**
/// - Multiple concurrent readers (get_service1, get_service2)
/// - Single writer (enable_direct_closure)
///
/// This is safe because:
/// - Reads happen during service calls (frequent)
/// - Writes happen only at initialization (rare)
pub struct EchoServiceGatewaysImpl {
    /// Target module ID.
    module_id: ModuleID,
    
    /// Service connector for protocol selection and connection.
    service_connector: Arc<dyn ServiceConnector>,
    
    /// Service handlers for direct (local) calls.
    ///
    /// RwLock provides interior mutability - we can mutate through &self.
    service_handlers: std::sync::RwLock<Option<EchoServiceHandlers>>,
}

impl EchoServiceGatewaysImpl {
    /// Creates a new Echo service gateways provider.
    ///
    /// # Arguments
    ///
    /// * `module_id` - Target module ID
    /// * `service_connector` - Connector for protocol selection
    pub fn new(
        module_id: ModuleID,
        service_connector: Arc<dyn ServiceConnector>,
    ) -> Self {
        Self {
            module_id,
            service_connector,
            service_handlers: std::sync::RwLock::new(None),
        }
    }
}

#[async_trait]
impl EchoServiceGateways for EchoServiceGatewaysImpl {
    fn module_id(&self) -> ModuleID {
        self.module_id.clone()
    }
    
    fn service_ids(&self) -> Vec<ServiceID> {
        vec![
            ServiceID::from("service1"),
            ServiceID::from("service2"),
        ]
    }
    
    fn enable_direct_closure(&self, handlers: EchoServiceHandlers) {
        debug!(
            "[EchoServiceGateways] Enabling direct closure for module {}",
            self.module_id
        );
        *self.service_handlers.write().unwrap() = Some(handlers);
    }
    
    async fn get_service1(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>> {
        debug!(
            "[EchoServiceGateways] Getting service1 with protocol {:?}",
            protocol
        );
        
        // Get direct handler if available
        let direct_handler = self.service_handlers
            .read()
            .unwrap()
            .as_ref()
            .map(|h| h.service1.clone());
        
        // Create the generic factory with protocol-specific factory functions
        let factory = ServiceGatewayFactory::<dyn EchoService>::new(
            self.module_id.clone(),
            ServiceID::from("service1"),
            self.service_connector.clone(),
            GatewayFactoryFuncs {
                // Direct factory: Returns the stored handler
                direct: direct_handler.map(|handler| {
                    Box::new(move || {
                        debug!("[EchoServiceGateways] Using direct handler for service1");
                        Ok(handler.clone())
                    }) as Box<dyn Fn() -> Result<Arc<dyn EchoService>> + Send + Sync>
                }),
                
                // gRPC factory: Creates a new gRPC gateway
                grpc: Some(Box::new(|channel| {
                    debug!("[EchoServiceGateways] Creating gRPC gateway for service1");
                    
                    // Create the gRPC client using the tonic channel
                    let client = crate::generated::echo_service_client::EchoServiceClient::new(channel);
                    let gateway = EchoGrpcGateway::from_client(client);
                    
                    Ok(Arc::new(gateway) as Arc<dyn EchoService>)
                }) as Box<dyn Fn(tonic::transport::Channel) -> Result<Arc<dyn EchoService>> + Send + Sync>),
                
                // HTTP factory: Not yet implemented
                http: None,
            },
        );
        
        // Use the factory to create the gateway
        // The factory will:
        // 1. Call service_connector.connect()
        // 2. Connector selects protocol (Auto -> Direct if available, else gRPC)
        // 3. Connector calls visitor.protocol_is_X()
        // 4. Visitor calls the appropriate factory function
        // 5. Factory returns typed gateway
        let service = factory.new_service_gateway(protocol).await?;
        
        debug!("[EchoServiceGateways] ✅ Service1 gateway created successfully");
        Ok(service)
    }
    
    async fn get_service2(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>> {
        debug!(
            "[EchoServiceGateways] Getting service2 with protocol {:?}",
            protocol
        );
        
        // Get direct handler if available
        let direct_handler = self.service_handlers
            .read()
            .unwrap()
            .as_ref()
            .map(|h| h.service2.clone());
        
        // Similar to get_service1, but for service2
        let factory = ServiceGatewayFactory::<dyn EchoService>::new(
            self.module_id.clone(),
            ServiceID::from("service2"),
            self.service_connector.clone(),
            GatewayFactoryFuncs {
                direct: direct_handler.map(|handler| {
                    Box::new(move || {
                        debug!("[EchoServiceGateways] Using direct handler for service2");
                        Ok(handler.clone())
                    }) as Box<dyn Fn() -> Result<Arc<dyn EchoService>> + Send + Sync>
                }),
                
                // gRPC for service2 not yet implemented (same gateway as service1)
                grpc: Some(Box::new(|channel| {
                    debug!("[EchoServiceGateways] Creating gRPC gateway for service2");
                    let client = crate::generated::echo_service_client::EchoServiceClient::new(channel);
                    let gateway = EchoGrpcGateway::from_client(client);
                    Ok(Arc::new(gateway) as Arc<dyn EchoService>)
                }) as Box<dyn Fn(tonic::transport::Channel) -> Result<Arc<dyn EchoService>> + Send + Sync>),
                
                http: None,
            },
        );
        
        let service = factory.new_service_gateway(protocol).await?;
        
        debug!("[EchoServiceGateways] ✅ Service2 gateway created successfully");
        Ok(service)
    }
}

/// Convenience function to create a new EchoServiceGateways instance.
///
/// # Example
///
/// ```rust,ignore
/// use echo_api_grpc::new_echo_service_gateways;
///
/// let gateways = new_echo_service_gateways(
///     ModuleID::from("echo"),
///     service_connector,
/// );
/// ```
pub fn new_echo_service_gateways(
    module_id: ModuleID,
    service_connector: Arc<dyn ServiceConnector>,
) -> Arc<dyn EchoServiceGateways> {
    Arc::new(EchoServiceGatewaysImpl::new(module_id, service_connector))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateways_creation() {
        use hsu_module_api::ServiceConnectorImpl;
        use hsu_module_api::ServiceRegistryClient;
        
        let registry_client = Arc::new(ServiceRegistryClient::new("http://localhost:8080"));
        let connector = Arc::new(ServiceConnectorImpl::new(registry_client)) as Arc<dyn ServiceConnector>;
        
        let gateways = EchoServiceGatewaysImpl::new(
            ModuleID::from("echo"),
            connector,
        );
        
        assert_eq!(gateways.module_id(), ModuleID::from("echo"));
        assert_eq!(gateways.service_ids().len(), 2);
    }
}

