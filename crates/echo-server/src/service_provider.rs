//! Service Provider for Echo Server Module
//!
//! # Architecture
//!
//! This is the **server-specific** service provider!
//! - Provides: EchoServiceHandlers (for registration)
//! - Does NOT provide: EchoServiceGateways (server doesn't need them!)

use std::sync::Arc;
use hsu_common::Result;
use hsu_module_api::HandlersRegistrar;
use echo_contract::EchoServiceHandlers;
use echo_domain::EchoServiceImpl;
use echo_api::new_echo_handlers_registrar;
use hsu_module_proto::ProtocolServer;
use tracing::debug;

/// Service provider for Echo server module.
///
/// ## Comparison with Golang
///
/// **Go version:**
/// ```go
/// type echoServiceProvider struct{}  // Empty!
/// ```
///
/// In Golang, the server service provider is **empty** because:
/// - The handler registrar factory is in the `ModuleDescriptor`
/// - The module factory creates the handlers
/// - The framework calls descriptor functions directly
///
/// ## Why Rust Has Helper Methods
///
/// This Rust implementation adds convenience methods that Golang doesn't have.
/// These could be removed to exactly match Golang's empty struct pattern, but
/// they provide a convenient place to encapsulate server-side logic.
///
/// **Trade-off:**
/// - ✅ Pro: Encapsulation, easier to test
/// - ❌ Con: Not exactly matching Golang's minimal pattern
///
/// For strict Golang parity, this could be simplified to:
/// ```rust
/// pub struct EchoServerServiceProvider {}  // Empty like Go
/// ```
/// And move all logic to standalone functions in the descriptor.
#[derive(Clone)]
pub struct EchoServerServiceProvider {
    protocol_servers: Vec<Arc<dyn ProtocolServer>>,
}

impl EchoServerServiceProvider {
    /// Creates a new server service provider.
    pub fn new(protocol_servers: Vec<Arc<dyn ProtocolServer>>) -> Self {
        debug!("[EchoServerServiceProvider] Creating with {} protocol servers", protocol_servers.len());
        Self { protocol_servers }
    }
    
    /// Gets the handlers registrar for this server.
    ///
    /// **Note:** In Golang, this is called directly from the descriptor,
    /// not through the service provider. This is a Rust convenience method.
    pub fn get_handlers_registrar(&self) -> Result<Box<dyn HandlersRegistrar<EchoServiceHandlers>>> {
        debug!("[EchoServerServiceProvider] Creating handlers registrar");
        new_echo_handlers_registrar(self.protocol_servers.clone())
    }
    
    /// Creates the service handlers (implementations).
    ///
    /// **Note:** In Golang, this is done in the module factory,
    /// not through the service provider. This is a Rust convenience method.
    pub fn create_service_handlers(&self) -> EchoServiceHandlers {
        debug!("[EchoServerServiceProvider] Creating service handlers");
        EchoServiceHandlers {
            service: Arc::new(EchoServiceImpl::new()),
        }
    }
}

