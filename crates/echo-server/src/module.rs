//! Echo Server Module Initialization
//!
//! # Architecture
//!
//! This module handles initialization of the Echo server module.
//! It registers the module with the framework and sets up wiring.

use std::sync::{Arc, Once};
use std::collections::HashMap;
use async_trait::async_trait;
use hsu_common::{ModuleID, Result};
use hsu_module_api::{
    ServiceProviderHandle, ServiceConnector, 
    ProtocolToServicesMap, HandlersRegistrarOptions,
    new_module_descriptor, register_module, Module, 
};
use echo_contract::{EchoServiceHandlers, EchoServiceGateways};
use crate::service::EchoServiceImpl;
use echo_api::{new_echo_handlers_registrar, echo_direct_closure_enabler};
use tracing::{debug, info};

use crate::service_provider::EchoServerServiceProvider;

/// Configuration for Echo server module.
pub struct EchoServerModuleConfig {
    pub module_id: ModuleID,
    pub grpc_port: u16,
}

impl Default for EchoServerModuleConfig {
    fn default() -> Self {
        Self {
            module_id: ModuleID::from("echo"),  // Match Golang: "echo" not "echo-server"!
            grpc_port: 0,
        }
    }
}

/// Factory function for creating the service provider.
///
/// This is a **function pointer** (not a closure) to match the framework API.
///
/// Note: Server modules receive protocol_servers from the framework at module creation time,
/// not at service provider creation time. So we create an empty service provider here.
fn create_service_provider(
    _service_connector: Arc<dyn ServiceConnector>,
) -> ServiceProviderHandle {
    debug!("[EchoServerModule] Creating service provider");
    
    // For a server module, we don't provide service gateways
    // (servers provide handlers, not gateways)
    ServiceProviderHandle {
        service_provider: Box::new(EchoServerServiceProvider {}),
        service_gateways_map: HashMap::new(),  // No gateways provided
    }
}

/// Echo server module implementation.
///
/// This is the server module that provides Echo services.
pub struct EchoServerModule {
    id: ModuleID,
    _service_provider: EchoServerServiceProvider,
}

impl EchoServerModule {
    pub fn new(service_provider: EchoServerServiceProvider) -> Self {
        Self {
            id: ModuleID::from("echo"),  // Note: This is "echo", not "echo-server"!
            _service_provider: service_provider,
        }
    }
}

#[async_trait]
impl Module for EchoServerModule {
    fn id(&self) -> &ModuleID {
        &self.id
    }

    async fn start(&mut self) -> Result<()> {
        info!("[EchoServer] Starting...");
        // Server just needs to be ready - handlers are already registered
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("[EchoServer] Stopping...");
        Ok(())
    }
}

/// Factory function for creating module.
///
/// Signature matches TypedModuleFactoryFunc<SP, SH>:
/// fn(SP) -> (Box<dyn Module>, SH)
fn create_module(service_provider: EchoServerServiceProvider) -> (Box<dyn Module>, EchoServiceHandlers) {
    debug!("[EchoServerModule] Creating module");
    
    // Create service handlers (implementations)
    
    // Create module
    let module = EchoServerModule::new(service_provider);

    // Create service handlers (implementations)
    //let handlers = service_provider.create_service_handlers();
    let handlers = EchoServiceHandlers {
        service: Arc::new(EchoServiceImpl::new()),
    };

    (Box::new(module), handlers)
}

/// Function for registering handlers with protocol servers.
///
/// This is called by the framework with the protocol servers.
fn echo_handlers_registrar(
    options: HandlersRegistrarOptions<EchoServiceHandlers>,
) -> Result<ProtocolToServicesMap> {
    debug!("[EchoServerModule] Creating handlers registrar with {} servers", options.protocol_servers.len());
    let registrar = new_echo_handlers_registrar(options.protocol_servers)?;
    registrar.register_handlers(options.service_handlers)
}

static INIT: Once = Once::new();

/// Initializes the Echo server module.
///
/// This function:
/// 1. Registers module descriptor with framework
///
/// Note: Protocol servers and actual setup happens at runtime!
///
/// ## Comparison with Golang
///
/// **Go version:**
/// ```go
/// func init() {
///     modulewiring.RegisterModule("echo", modulewiring.ModuleDescriptor[...]{
///         ServiceProviderFactoryFunc:   NewEchoServiceProvider,
///         ModuleFactoryFunc:            echodomain.NewEchoModule,
///         HandlersRegistrarFactoryFunc: echoapi.NewEchoHandlersRegistrar,
///         DirectClosureEnableFunc:      echoapi.EchoDirectClosureEnable,
///     })
/// }
/// ```
pub fn init_echo_server_module(config: EchoServerModuleConfig) -> Result<()> {
    INIT.call_once(|| {
        info!("[EchoServerModule] Initializing with config: module_id={}, grpc_port={}", 
            config.module_id, config.grpc_port);
        
        // Note: SG type is Arc<dyn EchoServiceGateways> because that's how CLIENTS access this server!
        // The SG parameter represents "gateway type used to access this module's services"
        let descriptor = new_module_descriptor::<
            EchoServerServiceProvider,
            Arc<dyn EchoServiceGateways>,  // Gateway type for clients accessing this server
            EchoServiceHandlers,            // Handler type this server provides
        >(
            create_service_provider,
            create_module,
            Some(echo_handlers_registrar),  // Server provides handlers!
            Some(echo_direct_closure_enabler), // Enable direct closure!
        );
        
        register_module(config.module_id.clone(), descriptor);
        
        info!("[EchoServerModule] ✅ Module registered successfully");
    });
    
    Ok(())
}

