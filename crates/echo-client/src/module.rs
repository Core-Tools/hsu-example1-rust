//! Echo Client Module (Layer 3)
//!
//! # Architecture
//!
//! This is the Module/Domain layer - module behavior and logic.
//!
//! Wiring (Layer 5) is in `wiring.rs` - kept separate!

use async_trait::async_trait;
use hsu_common::{ModuleID, Result};
use hsu_module_api::Module;
use tracing::info;

use crate::service_provider::EchoClientServiceProvider;

/// Echo client module implementation.
///
/// This is the Module/Domain layer (Layer 3) - module behavior.
/// 
/// Key characteristics:
/// - Protocol-agnostic (doesn't know gRPC vs direct)
/// - Uses service provider (from Layer 5)
/// - Implements Module trait (from Layer 1)
pub struct EchoClientModule {
    id: ModuleID,
    service_provider: EchoClientServiceProvider,
    message: String,
}

impl EchoClientModule {
    /// Creates a new echo client module.
    ///
    /// Note: This is called by the wiring layer (Layer 5).
    pub fn new(service_provider: EchoClientServiceProvider, message: String) -> Self {
        Self {
            id: ModuleID::from("echo-client"),
            service_provider,
            message,
        }
    }
}

#[async_trait]
impl Module for EchoClientModule {
    fn id(&self) -> &ModuleID {
        &self.id
    }

    async fn start(&mut self) -> Result<()> {
        info!("[EchoClient] Starting...");
        
        // Get gateways from service provider
        let gateways = self.service_provider.get_gateways();
        
        // Get service
        let service = gateways.get_service(hsu_common::Protocol::Auto).await?;
        
        info!("[EchoClient] Calling echo service...");
        let response = service.echo(self.message.clone()).await?;
        info!("[EchoClient] Response: {}", response);
        
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("[EchoClient] Stopping...");
        Ok(())
    }
}


