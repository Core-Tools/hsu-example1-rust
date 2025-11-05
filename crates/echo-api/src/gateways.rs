//! Echo Service Gateways Implementation (Layer 3/5 Boundary)
//!
//! Reusable implementation of `EchoServiceGateways` trait.

use std::sync::Arc;
use async_trait::async_trait;
use hsu_common::{ModuleID, ServiceID, Protocol, Result};
use hsu_module_api::{ServiceConnector, ServiceGatewayFactory, GatewayFactoryFuncs};
use echo_contract::{EchoService, EchoServiceGateways, EchoServiceHandlers};
use echo_api_grpc::EchoGrpcGateway;
use tracing::debug;

/// Implementation of EchoServiceGateways.
pub struct EchoServiceGatewaysImpl {
    module_id: ModuleID,
    service_connector: Arc<dyn ServiceConnector>,
    service_handlers: std::sync::RwLock<Option<EchoServiceHandlers>>,
}

impl EchoServiceGatewaysImpl {
    /// Creates a new Echo service gateways provider.
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
        vec![ServiceID::from("service")]
    }
    
    fn enable_direct_closure(&self, handlers: EchoServiceHandlers) {
        debug!("[EchoServiceGateways] Enabling direct closure for module {}", self.module_id);
        *self.service_handlers.write().unwrap() = Some(handlers);
    }
    
    async fn get_service(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>> {
        debug!("[EchoServiceGateways] Getting service with protocol {:?}", protocol);
        
        // Get direct handler if available
        let direct_handler = self.service_handlers
            .read()
            .unwrap()
            .as_ref()
            .map(|h| h.service.clone());
        
        // Create the generic factory
        let factory = ServiceGatewayFactory::<dyn EchoService>::new(
            self.module_id.clone(),
            ServiceID::from("service"),
            self.service_connector.clone(),
            GatewayFactoryFuncs {
                // Direct factory
                direct: direct_handler.map(|handler| {
                    Box::new(move || {
                        debug!("[EchoServiceGateways] Using direct handler");
                        Ok(handler.clone())
                    }) as Box<dyn Fn() -> Result<Arc<dyn EchoService>> + Send + Sync>
                }),
                
                // gRPC factory
                grpc: Some(Box::new(|channel| {
                    debug!("[EchoServiceGateways] Creating gRPC gateway");
                    let client = echo_api_grpc::generated::echo_service_client::EchoServiceClient::new(channel);
                    let gateway = EchoGrpcGateway::from_client(client);
                    Ok(Arc::new(gateway) as Arc<dyn EchoService>)
                }) as Box<dyn Fn(tonic::transport::Channel) -> Result<Arc<dyn EchoService>> + Send + Sync>),
                
                // HTTP factory
                http: None,
            },
        );
        
        let service = factory.new_service_gateway(protocol).await?;
        debug!("[EchoServiceGateways] ✅ Service gateway created successfully");
        Ok(service)
    }
}

/// Factory function to create EchoServiceGateways.
/// Factory function for creating EchoServiceGateways.
///
/// # Architecture Note
///
/// The target module ID ("echo") is **hard-coded** because this is
/// echo-specific Layer 5 code. It intrinsically "knows" it's for the
/// echo service - that's not configuration, that's **identity**!
///
/// ## Comparison with Golang
///
/// ```go
/// func NewEchoServiceGateways(...) echocontract.EchoServiceGateways {
///     return &echoServiceGateways{
///         targetModuleID: moduletypes.ModuleID("echo"),  // <- Hard-coded!
///         ...
///     }
/// }
/// ```
pub fn new_echo_service_gateways(
    service_connector: Arc<dyn ServiceConnector>,
) -> Arc<dyn EchoServiceGateways> {
    let module_id = ModuleID::from("echo");  // Hard-coded - this is echo-specific code!
    Arc::new(EchoServiceGatewaysImpl::new(module_id, service_connector))
}

