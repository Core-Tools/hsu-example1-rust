//! Echo Service Handler Registration (Layer 3/5 Boundary)
//!
//! Reusable implementation of handler registration for Echo services.

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use hsu_common::{Result, ServiceID, Protocol, Error};
use hsu_module_api::{ProtocolToServicesMap};
use hsu_module_proto::{ProtocolServer, ProtocolServerHandlersVisitor, grpc_server::GrpcServiceAdder};
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

    /// Registers Echo service handlers with all protocol servers.
    pub fn register_handlers(&self, handlers: EchoServiceHandlers) -> Result<ProtocolToServicesMap> {
        debug!("Registering Echo service handlers with {} servers", self.protocol_servers.len());
        
        let mut protocol_map: HashMap<Protocol, Vec<ServiceID>> = HashMap::new();
        
        // Create visitor for handler registration
        let visitor = Arc::new(ServiceHandlersVisitor {
            service: handlers.service.clone(),
        });
        
        // Register service with all servers
        // Note: We use tokio::task::block_in_place to call async methods from sync context
        // within an async runtime. This moves the blocking operation to a separate thread.
        for server in &self.protocol_servers {
            let protocol = server.protocol();
            trace!("Registering service with {:?} server on port {}", protocol, server.port());
            
            // Call the protocol-specific registration method
            // block_in_place allows us to call block_on from within an async context
            let result = tokio::task::block_in_place(|| {
                let handle = tokio::runtime::Handle::current();
                match protocol {
                    Protocol::Grpc => {
                        handle.block_on(visitor.register_handlers_grpc(server.clone()))
                    }
                    Protocol::Http => {
                        handle.block_on(visitor.register_handlers_http(server.clone()))
                    }
                    _ => {
                        warn!("Unsupported protocol: {:?}", protocol);
                        return Ok(());
                    }
                }
            });
            
            result?;
            
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

/// Service adder for Echo gRPC service.
/// 
/// Implements GrpcServiceAdder to add Echo service to a tonic Router.
struct EchoGrpcServiceAdder {
    handler: Arc<EchoGrpcHandler>,
}

impl GrpcServiceAdder for EchoGrpcServiceAdder {
    fn add_to_server(&self, mut server: tonic::transport::Server) -> tonic::transport::server::Router {
        use echo_api_grpc::generated::echo_service_server::EchoServiceServer;
        server.add_service(EchoServiceServer::new((*self.handler).clone()))
    }
    
    fn add_to_router(&self, router: tonic::transport::server::Router) -> tonic::transport::server::Router {
        use echo_api_grpc::generated::echo_service_server::EchoServiceServer;
        router.add_service(EchoServiceServer::new((*self.handler).clone()))
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
        let handler = Arc::new(EchoGrpcHandler::new(self.service.clone()));
        
        // Create service adder that knows how to add Echo service to Router
        let service_adder = Arc::new(EchoGrpcServiceAdder { handler });
        
        // Register the service adder with the gRPC server
        server.add_grpc_service_adder(service_adder).await?;
        
        debug!("✅ Echo service gRPC handler registered");
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
) -> Result<EchoHandlersRegistrar> {
    debug!("Creating new Echo handlers registrar");
    Ok(EchoHandlersRegistrar::new(protocol_servers)?)
}
