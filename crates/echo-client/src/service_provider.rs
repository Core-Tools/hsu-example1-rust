//! Service Provider for Echo Client Module
//!
//! # Architecture
//!
//! This is the **client-specific** service provider!
//! - Provides: EchoServiceGateways (for calling echo server)
//! - Does NOT provide: EchoServiceHandlers (client doesn't have them!)
//!
//! # Important Design Note
//!
//! Notice that we don't pass a target module ID to `new_echo_service_gateways()`!
//! That's because `new_echo_service_gateways()` is **echo-specific** Layer 5 code
//! that intrinsically knows it's for the "echo" module. The target module ID is
//! not configuration - it's the **identity** of the echo API layer itself.

use std::sync::Arc;
use echo_contract::EchoServiceGateways;
use hsu_module_api::ServiceConnector;
use echo_api::new_echo_service_gateways;
use tracing::debug;

/// Service provider for Echo client module.
///
/// ## Comparison with Golang
///
/// **Go version:**
/// ```go
/// type EchoClientServiceProvider struct {
///     gateways EchoServiceGateways
/// }
///
/// func (p *EchoClientServiceProvider) GetGateways() EchoServiceGateways {
///     return p.gateways
/// }
/// ```
pub struct EchoClientServiceProvider {
    gateways: Arc<dyn EchoServiceGateways>,
}

impl EchoClientServiceProvider {
    /// Creates a new client service provider.
    ///
    /// Note: We don't need to pass a target module ID because
    /// `new_echo_service_gateways()` knows it's for the "echo" module.
    pub fn new(
        service_connector: Arc<dyn ServiceConnector>,
    ) -> Self {
        debug!("[EchoClientServiceProvider] Creating echo service gateways");
        
        let gateways = new_echo_service_gateways(service_connector);
        
        Self { gateways }
    }
    
    /// Gets the service gateways.
    pub fn get_gateways(&self) -> Arc<dyn EchoServiceGateways> {
        self.gateways.clone()
    }
}

