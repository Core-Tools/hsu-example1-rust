//! Echo Client Module Initialization
//!
//! # Architecture
//!
//! This module handles initialization of the Echo client module.
//! It registers the module with the framework and sets up wiring.

use std::sync::Arc;
use std::collections::HashMap;
use hsu_common::{ModuleID, Result};
use hsu_module_api::{ServiceProviderHandle, ServiceConnector};
use tracing::{debug, info};

use crate::service_provider::EchoClientServiceProvider;

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
    info!("[EchoClientModule] Initializing with config: module_id={}", config.module_id);
    
    // TODO: Store config for later use
    // For now, we just log it
    
    info!("[EchoClientModule] ✅ Module initialized successfully");
    Ok(())
}

