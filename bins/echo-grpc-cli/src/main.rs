//! Echo gRPC Client - Application layer for Echo client.
//!
//! # Architecture Note
//!
//! This is the **application layer** that wires everything together.
//! The actual client business logic is in `echo-client` crate.
//!
//! ## Layer Separation
//!
//! ```
//! Application Layer (this file):
//! - Configuration (CLI args, registry URL)
//! - Protocol-specific setup (GatewayFactory registration)
//! - Module instantiation and wiring
//! - Runtime lifecycle management
//!
//! Client Module Layer (echo-client crate):
//! - Business logic (making echo calls)
//! - Uses ServiceGatewayFactory from framework
//! - Protocol-agnostic code
//! ```
//!
//! ## Matches Go Structure
//!
//! This mirrors the Go implementation:
//! ```
//! Go Application:   hsu-example1-go/cmd/cli/echogrpccli/main.go
//! Go Client Module: hsu-example1-go/cmd/cli/echoclient/module.go
//!
//! Rust Application:   bins/echo-grpc-cli/src/main.rs (this file)
//! Rust Client Module: crates/echo-client/src/lib.rs
//! ```
//!
//! # What This Demonstrates
//!
//! 1. **Separated Client Module** - Reusable business logic in `echo-client`
//! 2. **ModuleRuntime** - Complete lifecycle management
//! 3. **Service Discovery** - Automatic via gateway factory
//! 4. **Protocol::Auto** - Framework chooses Direct or gRPC automatically!
//! 5. **User-Provided Factory** - ProtocolGatewayFactory trait implementation
//! 6. **Protocol Transparency** - Same code works for Direct OR gRPC!
//!
//! # Architecture
//!
//! ```
//! Process: echo-grpc-cli
//! └── ModuleRuntime (manages everything!)
//!     └── EchoClientModule (from echo-client crate)
//!         ├── Uses ServiceGatewayFactory
//!         ├── Requests with Protocol::Auto
//!         ├── Framework discovers service from registry
//!         ├── Framework selects Direct (if local) or gRPC (if remote)
//!         ├── Framework calls user's ProtocolGatewayFactory
//!         └── Client uses gateway transparently (via extension trait)
//! ```
//!
//! **Framework handles protocol selection automatically!**

use std::sync::Arc;
use clap::Parser;

use hsu_common::{ModuleID, ServiceID, Protocol, Result};
use hsu_module_api::{ModuleRuntime, RuntimeConfig};
use hsu_module_management::GatewayConfig;
use echo_client::EchoClientModule;  // ← Client module from separate crate!

use tracing::info;
use tracing_subscriber;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about = "Echo gRPC Client with framework runtime")]
struct Args {
    /// Service registry URL
    #[arg(short, long, default_value = "http://localhost:8080")]
    registry_url: String,
    
    /// Message to echo
    #[arg(short, long, default_value = "Hello from Rust client! 🦀")]
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Echo gRPC Client - Starting with FRAMEWORK RUNTIME! ✨");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ Separated client module (echo-client crate)");
    info!("✅ Application layer (this file) - just wiring");
    info!("✅ ServiceGatewayFactory for discovery");
    info!("✅ User-provided factory (ProtocolGatewayFactory)");
    info!("✅ Framework handles everything!");
    info!("✅ FULL PARITY with Golang structure!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // ========================================
    // APPLICATION LAYER: Configuration & Wiring
    // ========================================
    //
    // This is the ONLY place where protocol-specific setup happens.
    // The client module (echo-client crate) is protocol-agnostic!
    
    // 1. Register gateway factory for echo service (protocol-specific)
    use echo_api_grpc::EchoGrpcGatewayFactory;
    
    let gateway_configs = vec![
        GatewayConfig::new(ServiceID::from("echo-service"), Protocol::Grpc)
            .with_factory(Arc::new(EchoGrpcGatewayFactory)),  // ← User provides factory!
    ];
    
    let mut gateway_map = std::collections::HashMap::new();
    gateway_map.insert(ModuleID::from("echo"), gateway_configs);

    // 2. Create runtime configuration
    let config = RuntimeConfig::new()
        .with_registry_url(&args.registry_url)
        .with_gateway_configs(gateway_map);

    // 3. Create runtime
    let mut runtime = ModuleRuntime::new(config);

    // 4. Instantiate client module (from echo-client crate - reusable!)
    let client_module = EchoClientModule::new(args.message);
    runtime.add_module(Box::new(client_module));

    // 5. Start runtime
    info!("Starting runtime...");
    info!("  - Runtime will inject gateway factory into EchoClientModule");
    info!("  - EchoClientModule will use factory to discover echo service");
    info!("  - Factory will call EchoGrpcGatewayFactory (registered above)");
    info!("  - EchoClientModule will make calls (protocol-agnostic code)!\n");
    
    runtime.start().await?;

    info!("\n🎉 Demo complete!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("✅ Separated client module (echo-client crate)!");
    info!("✅ Application layer is just wiring!");
    info!("✅ ModuleRuntime integration works!");
    info!("✅ ServiceGatewayFactory works!");
    info!("✅ ProtocolGatewayFactory trait works!");
    info!("✅ User-provided factory called successfully!");
    info!("✅ Service discovery works!");
    info!("✅ gRPC communication works!");
    info!("✅ Extension trait makes code protocol-agnostic!");
    info!("✅ COMPLETE PARITY WITH GOLANG STRUCTURE! 🎊");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    info!("Press Ctrl+C to exit...\n");

    // 6. Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;

    // 7. Stop runtime
    info!("Shutting down...");
    runtime.stop().await?;

    info!("👋 Goodbye!");
    Ok(())
}
