//! Echo Client Module - Framework-integrated client for Echo service.
//!
//! # Architecture Note
//!
//! This crate contains the **client module** that uses the HSU framework
//! to call Echo services. It's separated from the application layer
//! (main.rs) to clearly show the reusable business logic.
//!
//! ## Matches Go Structure
//!
//! This mirrors the Go implementation:
//! ```
//! Go:   hsu-example1-go/cmd/cli/echoclient/module.go
//! Rust: hsu-example1-rust/crates/echo-client/src/lib.rs
//! ```
//!
//! ## Layer Responsibilities
//!
//! ```
//! Application Layer (bins/echo-grpc-cli/main.rs):
//! - Configuration (registry URL, etc.)
//! - Module wiring
//! - Runtime lifecycle
//! - Protocol-specific setup (GatewayFactory registration)
//!
//! Client Module Layer (crates/echo-client/):
//! - Business logic (making echo calls)
//! - Uses framework's ServiceGatewayFactory
//! - Protocol-agnostic code (thanks to extension trait!)
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use echo_client::EchoClientModule;
//!
//! let mut runtime = ModuleRuntime::new(config);
//! let client_module = EchoClientModule::new("Hello".to_string());
//! runtime.add_module(Box::new(client_module));
//! runtime.start().await?;
//! ```

use async_trait::async_trait;
use hsu_common::{ModuleID, ServiceID, Protocol, Result};
use hsu_module_management::{Module, ServiceGatewayFactory, ServiceHandlersMap};
use echo_api_grpc::ServiceGatewayEchoExt;  // ← Extension trait for gateway usage
use std::sync::Arc;
use tracing::info;

/// Echo Client Module - Uses framework's ServiceGatewayFactory.
///
/// # Architecture Note
///
/// This module demonstrates the CLIENT side of the HSU framework:
/// - Uses `ServiceGatewayFactory` injected by the runtime
/// - Discovers services automatically through the factory
/// - Factory handles protocol selection (Direct vs gRPC vs HTTP)
/// - No manual service discovery code needed!
///
/// # Comparison with Go
///
/// This matches the Golang `echoclient` module pattern exactly:
/// - Go: `hsu-example1-go/cmd/cli/echoclient/module.go`
/// - Rust: This file
///
/// Both have the same structure:
/// 1. Module struct with factory field
/// 2. `set_service_gateway_factory` method
/// 3. `start` method that uses factory to get gateway
/// 4. Business logic that calls the service
///
/// # Code Flow
///
/// 1. Runtime creates module
/// 2. Runtime injects gateway factory via `set_service_gateway_factory`
/// 3. Module's `start` method:
///    - Uses factory to get gateway (with `Protocol::Auto`)
///    - Framework discovers service (local or remote)
///    - Framework selects protocol (Direct or gRPC)
///    - Framework calls user's `ProtocolGatewayFactory`
/// 4. Module extracts domain service using extension trait
/// 5. Module makes calls (protocol-agnostic!)
pub struct EchoClientModule {
    id: ModuleID,
    factory: Option<Arc<dyn ServiceGatewayFactory>>,
    message: String,
}

impl EchoClientModule {
    /// Creates a new Echo client module.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send to the Echo service
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let client = EchoClientModule::new("Hello, Echo!".to_string());
    /// ```
    pub fn new(message: String) -> Self {
        Self {
            id: ModuleID::from("echo-client"),
            factory: None,
            message,
        }
    }
}

#[async_trait]
impl Module for EchoClientModule {
    fn id(&self) -> &ModuleID {
        &self.id
    }

    fn set_service_gateway_factory(&mut self, factory: Arc<dyn ServiceGatewayFactory>) {
        info!("[EchoClientModule] Received gateway factory");
        self.factory = Some(factory);
    }

    fn service_handlers_map(&self) -> Option<ServiceHandlersMap> {
        // Client module has no handlers
        None
    }

    async fn start(&mut self) -> Result<()> {
        info!("[EchoClientModule] Starting...");
        
        let factory = self.factory.as_ref()
            .ok_or_else(|| hsu_common::Error::Validation {
                message: "Gateway factory not injected".to_string()
            })?;

        // Get gateway to echo service
        // The factory will:
        // 1. Look up "echo" module in the registry (or check local)
        // 2. Find the "echo-service" service
        // 3. Auto-select protocol: Direct if local, gRPC if remote
        // 4. Call user's ProtocolGatewayFactory to create gateway
        info!("[EchoClientModule] Requesting gateway for 'echo' module, 'echo-service' service");
        info!("[EchoClientModule] Using Protocol::Auto - framework will choose optimal protocol");
        
        let target_module = ModuleID::from("echo");
        let service_id = ServiceID::from("echo-service");
        
        let gateway = factory
            .new_service_gateway(
                &target_module,
                &service_id,
                Protocol::Auto,  // ← Let framework choose!
            )
            .await?;
        
        info!("[EchoClientModule] ✅ Got gateway from factory!");
        
        // Extract EchoService trait - just like Go's cast!
        // This works for ANY protocol: Direct, gRPC, or HTTP!
        //
        // Compare to Go pattern:
        //   typifiedService, ok := gateway.(contract.Contract1)
        //
        // Rust equivalent:
        let echo_service = gateway.as_echo_service()?;
        //                         ↑
        //                         One method call - protocol-agnostic!
        //                         Extension trait hides protocol matching!
        
        info!("[EchoClientModule] ✅ Extracted EchoService (protocol-agnostic)");
        
        // Now use the service - same code works for ALL protocols!
        // This is the power of abstraction + extension traits! 🎯
        info!("[EchoClientModule] 📤 Sending: \"{}\"", self.message);
        let response = echo_service.echo(self.message.clone()).await?;
        info!("[EchoClientModule] 📥 Response: \"{}\"", response);
        
        // Make another call to demonstrate
        let message2 = "Protocol::Auto + Extension Trait = Perfect! 🚀";
        info!("[EchoClientModule] 📤 Sending: \"{}\"", message2);
        let response2 = echo_service.echo(message2.to_string()).await?;
        info!("[EchoClientModule] 📥 Response: \"{}\"", response2);
        
        info!("[EchoClientModule] ✅ All calls successful!");
        info!("[EchoClientModule] Started successfully");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("[EchoClientModule] Stopping...");
        Ok(())
    }
}

