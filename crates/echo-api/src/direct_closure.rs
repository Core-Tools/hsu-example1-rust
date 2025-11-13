//! Direct Closure Support for Echo Services (Layer 3/5 Boundary)
//!
//! Enables direct (in-process) service calls.

use std::sync::Arc;
use hsu_module_api::DirectClosureEnablerOptions;
use echo_contract::{EchoServiceGateways, EchoServiceHandlers};
use tracing::debug;

/// Enables direct closure for Echo services.
///
/// This function is called by the module registry during initialization.
/// It performs two key operations:
///
/// 1. Registers with ServiceConnector
/// 2. Stores handlers in gateways
pub fn echo_direct_closure_enabler(
    options: DirectClosureEnablerOptions<Arc<dyn EchoServiceGateways>, EchoServiceHandlers>,
) {
    debug!("[EchoDirectClosure] Enabling direct closure for module {}", 
        options.service_gateways.module_id());
    
    // 1. Register with ServiceConnector
    options.service_connector.enable_direct_closure(
        options.service_gateways.module_id(),
        options.service_gateways.service_ids(),
    );
    
    // 2. Store handlers in gateways
    options.service_gateways.enable_direct_closure(options.service_handlers);
    
    debug!("[EchoDirectClosure] ✅ Direct closure enabled successfully");
}

