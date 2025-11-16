//! Echo Server Module (Layer 3)
//!
//! # Architecture
//!
//! This is the Module/Domain layer - module behavior.
//!
//! Wiring (Layer 5) is in `wiring.rs` - kept separate!

use async_trait::async_trait;
use hsu_common::{ModuleID, Result};
use hsu_module_api::Module;
use tracing::info;

use crate::service_provider::EchoServerServiceProvider;

/// Echo server module implementation.
///
/// This is the Module/Domain layer (Layer 3) - module behavior.
///
/// Key characteristics:
/// - Protocol-agnostic (doesn't know gRPC vs direct)
/// - Provides handlers (registered by Layer 5)
/// - Implements Module trait (from Layer 1)
pub struct EchoServerModule {
    id: ModuleID,
    _service_provider: EchoServerServiceProvider,
}

impl EchoServerModule {
    /// Creates a new echo server module.
    ///
    /// Note: This is called by the wiring layer (Layer 5).
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


