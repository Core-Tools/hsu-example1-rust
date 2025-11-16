//! Echo Client Module Wiring (Layer 5)
//!
//! # Architecture
//!
//! This is the **Module Wiring** layer - module self-registration code.
//! 
//! This layer:
//! - Registers the module with the framework
//! - Creates module descriptor
//! - Defines factory functions
//!
//! This is MODULE-specific, not application-specific!
//! Each module has its own wiring that defines how it integrates with the framework.

use std::sync::{Arc, Once};
use std::collections::HashMap;
use hsu_common::{ModuleID, Result};
use hsu_module_api::{
    ServiceProviderHandle, ServiceConnector, 
    new_module_descriptor, register_module, Module,
};
use tracing::{debug, info};

use crate::service_provider::EchoClientServiceProvider;
use crate::module::EchoClientModule;

/// Configuration for Echo client module.
pub struct EchoClientModuleConfig {
    pub module_id: ModuleID,
}

impl Default for EchoClientModuleConfig {
    fn default() -> Self {
        Self {
            module_id: ModuleID::from("echo-client"),
        }
    }
}

/// Factory function for creating the service provider.
///
/// This is a **function pointer** (not a closure) to match the framework API.
///
/// # Architecture Note
///
/// Notice we don't need to know the target module ID here! The echo API layer
/// (`new_echo_service_gateways()`) knows it's for the "echo" module - that's
/// intrinsic to the echo-specific Layer 5 code, not a configuration parameter.
fn create_service_provider(
    service_connector: Arc<dyn ServiceConnector>,
) -> ServiceProviderHandle {
    debug!("[EchoClientModule] Creating service provider");
    
    let service_provider = EchoClientServiceProvider::new(service_connector);
    
    // Store the gateways in the map (keyed by target module ID)
    let gateways = service_provider.get_gateways();
    let target_module_id = gateways.module_id();  // Ask the gateways for their module ID!
    
    let mut service_gateways_map = HashMap::new();
    service_gateways_map.insert(
        target_module_id,
        Box::new(gateways) as Box<dyn std::any::Any + Send + Sync>,
    );
    
    ServiceProviderHandle {
        service_provider: Box::new(service_provider),
        service_gateways_map,
    }
}

/// Factory function for creating module.
///
/// Signature matches TypedModuleFactoryFunc<SP, SH>:
/// fn(SP) -> (Box<dyn Module>, SH)
fn create_module(service_provider: EchoClientServiceProvider) -> (Box<dyn Module>, ()) {
    debug!("[EchoClientModule] Creating module");
    
    let module = EchoClientModule::new(
        service_provider,
        "Hello from Rust client!".to_string(),
    );
    
    let handlers = (); // Client doesn't provide handlers
    
    (Box::new(module), handlers)
}

static INIT: Once = Once::new();

/// Initializes the Echo client module.
///
/// This function:
/// 1. Registers module descriptor with framework
///
/// Note: Actual module creation happens at runtime!
///
/// ## Comparison with Golang
///
/// **Go version:**
/// ```go
/// func init() {
///     moduleapi.RegisterModule(
///         "echo-client",
///         moduleapi.ModuleDescriptor[...]{
///             ServiceProviderFactory: func(options) ServiceProvider {...},
///         },
///     )
/// }
/// ```
pub fn init_echo_client_module(config: EchoClientModuleConfig) -> Result<()> {
    INIT.call_once(|| {
        info!("[EchoClientModule] Initializing with config: module_id={}", config.module_id);
        
        let descriptor = new_module_descriptor::<EchoClientServiceProvider, (), ()>(
            create_service_provider,
            create_module,
            None, // No handlers registrar (client module)
            None, // No direct closure enable (client module)
        );
        
        register_module(config.module_id.clone(), descriptor);
        
        info!("[EchoClientModule] ✅ Module registered successfully");
    });
    
    Ok(())
}

