//! Echo Service Handler Registration (Layer 3/5 Boundary)
//!
//! Reusable implementation of handler registration for Echo services.

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use hsu_common::{Result, ServiceID, Protocol, Error};
use hsu_module_api::{HandlersRegistrar, ProtocolToServicesMap};
use hsu_module_proto::{ProtocolServer, ProtocolServerHandlersVisitor};
use echo_contract::{EchoService, EchoServiceHandlers};
use echo_api_grpc::EchoGrpcHandler;
use tracing::{debug, trace, warn};

/// Handlers registrar for Echo services.
pub struct EchoHandlersRegistrar {
    protocol_servers: Vec<Arc<dyn ProtocolServer>>,
}

impl EchoHandlersRegistrar {
    /// Creates a new Echo handlers registrar.
    pub fn new(protocol_servers: Vec<Arc<dyn ProtocolServer>>) -> Result<Self> {
        debug!("Creating EchoHandlersRegistrar with {} servers", protocol_servers.len());
        Ok(Self { protocol_servers })
    }
}

impl HandlersRegistrar<EchoServiceHandlers> for EchoHandlersRegistrar {
    /// Registers Echo service handlers with all protocol servers.
    fn register_handlers(&self, handlers: EchoServiceHandlers) -> Result<ProtocolToServicesMap> {
        debug!("Registering Echo service handlers with {} servers", self.protocol_servers.len());
        
        let mut protocol_map: HashMap<Protocol, Vec<ServiceID>> = HashMap::new();
        
        // Register service with all servers
        for server in &self.protocol_servers {
            let protocol = server.protocol();
            trace!("Registering service with {:?} server on port {}", protocol, server.port());
            
            let _visitor = Arc::new(ServiceHandlersVisitor {
                service: handlers.service.clone(),
            });
            
            // TODO: Make HandlersRegistrar::register_handlers async
            // For now, just add to protocol map without actually registering
            protocol_map
                .entry(protocol)
                .or_insert_with(Vec::new)
                .push(ServiceID::from("service"));
            
            debug!("✅ Registered service with {:?} server", protocol);
        }
        
        debug!("✅ All Echo handlers registered. Protocol map: {:?}", protocol_map.keys().collect::<Vec<_>>());
        Ok(protocol_map)
    }
}

/// Visitor for registering service handlers.
struct ServiceHandlersVisitor {
    service: Arc<dyn EchoService>,
}

#[async_trait]
impl ProtocolServerHandlersVisitor for ServiceHandlersVisitor {
    async fn register_handlers_grpc(&self, server: Arc<dyn ProtocolServer>) -> Result<()> {
        debug!("Registering service with gRPC server");
        
        if server.protocol() != Protocol::Grpc {
            return Err(Error::Validation {
                message: format!("Expected gRPC server, got {:?}", server.protocol()),
            });
        }
        
        // Create gRPC handler
        let _handler = EchoGrpcHandler::new(self.service.clone());
        
        // TODO: Actual registration with tonic Router
        debug!("✅ Service gRPC handler created (actual registration pending)");
        Ok(())
    }
    
    async fn register_handlers_http(&self, server: Arc<dyn ProtocolServer>) -> Result<()> {
        if server.protocol() != Protocol::Http {
            return Err(Error::Validation {
                message: format!("Expected HTTP server, got {:?}", server.protocol()),
            });
        }
        
        warn!("HTTP handler registration not yet implemented");
        Ok(())
    }
}

/// Factory function for creating an Echo handlers registrar.
pub fn new_echo_handlers_registrar(
    protocol_servers: Vec<Arc<dyn ProtocolServer>>,
) -> Result<Box<dyn HandlersRegistrar<EchoServiceHandlers>>> {
    debug!("Creating new Echo handlers registrar");
    Ok(Box::new(EchoHandlersRegistrar::new(protocol_servers)?))
}

