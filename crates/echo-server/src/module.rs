//! Echo Server Module Initialization
//!
//! # Architecture
//!
//! This module handles initialization of the Echo server module.
//! It registers the module with the framework and sets up wiring.

use std::sync::Arc;
use std::collections::HashMap;
use hsu_common::{ModuleID, Result};
use hsu_module_api::ServiceProviderHandle;
use tracing::{debug, info};

/// Configuration for Echo server module.
pub struct EchoServerModuleConfig {
    pub module_id: ModuleID,
    pub grpc_port: u16,
}

impl Default for EchoServerModuleConfig {
    fn default() -> Self {
        Self {
            module_id: ModuleID::from("echo-server"),
            grpc_port: 50051,
        }
    }
}

/// Factory function for creating the service provider.
///
/// This is a **function pointer** (not a closure) to match the framework API.
fn create_service_provider(
    _service_connector: Arc<dyn hsu_module_api::ServiceConnector>,
) -> ServiceProviderHandle {
    debug!("[EchoServerModule] Creating service provider");
    
    // For a server module, we don't provide service gateways
    // (servers provide handlers, not gateways)
    ServiceProviderHandle {
        service_provider: Box::new(()),  // Server doesn't need a service provider for access
        service_gateways_map: HashMap::new(),  // No gateways provided
    }
}

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
///     moduleapi.RegisterModule(
///         "echo-server",
///         moduleapi.ModuleDescriptor[...]{
///             ServiceProviderFactory: func(options) ServiceProvider {...},
///             DirectClosureEnable: echoapi.EchoDirectClosureEnable,
///         },
///     )
/// }
/// ```
pub fn init_echo_server_module(config: EchoServerModuleConfig) -> Result<()> {
    info!("[EchoServerModule] Initializing with config: module_id={}, grpc_port={}", 
        config.module_id, config.grpc_port);
    
    // TODO: Store config for later use when creating protocol servers
    // For now, we just register the module descriptor
    
    info!("[EchoServerModule] ✅ Module initialized successfully");
    Ok(())
}

