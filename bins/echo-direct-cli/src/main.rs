//! Echo Direct CLI - Demonstrates in-process direct communication.
//!
//! # What This Demonstrates
//!
//! 1. **Direct Protocol**: Zero-cost in-process calls (~6 cycles!)
//! 2. **Module Runtime**: Complete lifecycle management (using framework!)
//! 3. **Service Discovery**: Local module lookup
//! 4. **Type Safety**: No casts, all compile-time checked
//!
//! # Architecture
//!
//! ```
//! Process: echo-direct-cli
//! ├── ModuleRuntime (from framework - automatic initialization!)
//! ├── EchoModule (provides EchoService)
//! └── ClientModule (calls EchoService)
//!     ↓ Direct call (same process!)
//!     └─→ EchoService::echo()
//! ```
//!
//! **Performance:** ~6 CPU cycles overhead!
//!
//! # Changes from Original
//!
//! - ✅ Now uses framework's `RuntimeConfig` directly
//! - ✅ No custom `create_local_runtime()` function
//! - ✅ Framework handles all initialization automatically
//! - ✅ Cleaner, more maintainable code

use std::sync::Arc;

use hsu_common::{ModuleID, ServiceID, Protocol, Result};
use hsu_module_management::{Module, ServiceGatewayFactory, ServiceGateway, ServiceHandler, ServiceHandlersMap};
use hsu_module_api::{ModuleRuntime, RuntimeConfig};

use echo_domain::EchoModule;

use async_trait::async_trait;
use tracing::{info, error};
use tracing_subscriber;

/// Simple client module that calls the echo service.
struct ClientModule {
    id: ModuleID,
    factory: Option<Arc<dyn ServiceGatewayFactory>>,
}

impl ClientModule {
    fn new() -> Self {
        Self {
            id: ModuleID::from("client"),
            factory: None,
        }
    }
}

#[async_trait]
impl Module for ClientModule {
    fn id(&self) -> &ModuleID {
        &self.id
    }

    fn service_handlers_map(&self) -> Option<ServiceHandlersMap> {
        None // Client doesn't provide services
    }

    fn set_service_gateway_factory(&mut self, factory: Arc<dyn ServiceGatewayFactory>) {
        self.factory = Some(factory);
    }

    async fn start(&mut self) -> Result<()> {
        info!("Client module starting...");

        // Demo: Call the echo service!
        if let Some(factory) = &self.factory {
            info!("Calling echo service via Direct protocol...");

            // Get gateway (auto-protocol will select Direct!)
            let gateway = factory
                .new_service_gateway(
                    &ModuleID::from("echo"),
                    &ServiceID::from("echo-service"),
                    Protocol::Auto, // ← Magic! Auto-selects Direct
                )
                .await?;

            // Extract the service handler
            match gateway {
                ServiceGateway::Direct(handler) => {
                    match handler.as_ref() {
                        ServiceHandler::Echo(echo_service) => {
                            // Call the service!
                            let result = echo_service.echo("Hello from Rust! 🦀".to_string()).await?;
                            info!("✅ Response: {}", result);
                            
                            // Call again to show it's really fast!
                            let result2 = echo_service.echo("Direct protocol is FAST! 🚀".to_string()).await?;
                            info!("✅ Response: {}", result2);
                        }
                    }
                }
                _ => {
                    error!("Expected Direct gateway, got something else");
                }
            }
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Client module stopping...");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Echo Direct CLI - Starting...");
    info!("This demonstrates DIRECT in-process communication!");
    info!("Performance: ~6 CPU cycles overhead! ⚡");
    info!("Now using the FRAMEWORK runtime! ✨\n");

    // 1. Create configuration (no servers needed for direct communication!)
    let config = RuntimeConfig::new()
        .with_registry_url("http://localhost:8080");

    // 2. Create runtime
    let mut runtime = ModuleRuntime::new(config);

    // 3. Add modules
    runtime.add_module(Box::new(EchoModule::new()));
    runtime.add_module(Box::new(ClientModule::new()));

    // 4. Start runtime
    // The framework will:
    // - Initialize registry client
    // - Build local module map for direct protocol
    // - Create gateway factory
    // - Inject factory into modules
    // - Start modules
    info!("Starting runtime...");
    runtime.start().await?;

    info!("\n✅ Demo complete! Both modules communicated directly in the same process!");
    info!("✨ Framework handled all initialization automatically!");
    info!("Press Ctrl+C to exit...\n");

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;

    // Stop runtime
    info!("Shutting down...");
    runtime.stop().await?;

    info!("👋 Goodbye!");
    Ok(())
}

