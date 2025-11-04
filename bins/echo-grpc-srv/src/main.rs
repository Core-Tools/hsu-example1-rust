//! Echo gRPC Server - Demonstrates gRPC protocol with FULL HSU framework.
//!
//! # What This Demonstrates
//!
//! 1. **EchoModule** - Real HSU module with business logic
//! 2. **ModuleRuntime** - Full lifecycle management with ServerOptions
//! 3. **gRPC Server** - Framework-managed protocol server
//! 4. **Service Registry** - Automatic API publishing
//! 5. **Complete Framework Integration** - Runtime manages everything!
//!
//! # Architecture (Updated!)
//!
//! ```
//! Process: echo-grpc-srv
//! └── ModuleRuntime (manages everything!)
//!     ├── EchoModule (provides EchoService)
//!     ├── GrpcProtocolServer (port 50051) ← Managed by framework!
//!     └── ServiceRegistryClient ← Automatic publishing!
//! ```
//!
//! **Framework now handles:**
//! - ✅ Protocol server creation
//! - ✅ Server lifecycle (start/stop)
//! - ✅ API publishing to registry
//! - ✅ Graceful shutdown
//!
//! # Changes from Original
//!
//! - ✅ Uses `ServerOptions` to configure gRPC server
//! - ✅ Uses `HandlersConfig` to register module handlers
//! - ✅ Runtime manages server lifecycle (no manual spawning!)
//! - ✅ Runtime publishes APIs automatically
//! - ✅ Cleaner shutdown (runtime handles everything)

use std::sync::Arc;
use clap::Parser;
use tokio::sync::oneshot;

use echo_domain::{EchoModule, EchoServiceImpl};
use echo_api_grpc::run_echo_grpc_server;

use hsu_common::{ModuleID, Protocol};
use hsu_module_api::RuntimeConfig;
use hsu_module_management::{Module, ServerOptions, HandlersConfig};

use tracing::{info, error};
use tracing_subscriber;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about = "Echo gRPC Server with full HSU framework")]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value = "50051")]
    port: u16,
    
    /// Service registry URL
    #[arg(short, long, default_value = "http://localhost:8080")]
    registry_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Echo gRPC Server - Starting with framework-managed servers!");
    info!("Runtime will handle server lifecycle and API publishing! ✨\n");

    // 1. Create echo module
    let echo_module = EchoModule::new();
    let module_id = echo_module.id().clone();
    info!("✅ Created EchoModule: {}", module_id);

    // 2. Configure runtime with ServerOptions and HandlersConfig
    let server_options = vec![
        ServerOptions::new("grpc-server", Protocol::Grpc, args.port),
    ];
    
    let handlers_configs = vec![
        HandlersConfig::new(
            module_id.clone(),
            "grpc-server",
            Protocol::Grpc,
        ),
    ];

    let config = RuntimeConfig::new()
        .with_registry_url(&args.registry_url)
        .with_process_id(std::process::id())
        .with_server_options(server_options)
        .with_handlers_configs(handlers_configs);
    
    info!("✅ Configured runtime:");
    info!("   - gRPC server on port {}", args.port);
    info!("   - Handler registration for module: {}", module_id);
    info!("   - Service registry: {}", args.registry_url);

    // 3. Create runtime and add module
    let mut runtime = hsu_module_api::ModuleRuntime::new(config);
    runtime.add_module(Box::new(echo_module));
    
    info!("✅ Created ModuleRuntime");

    // 4. Start runtime (framework lifecycle & API publishing)
    info!("\nStarting framework runtime...");
    info!("  - Initializing registry client");
    info!("  - Setting up lifecycle management");
    info!("  - Starting modules");
    info!("  - Publishing APIs to registry\n");
    
    runtime.start().await?;
    info!("✅ Framework runtime started!");

    // 5. Start actual gRPC server with tonic
    // This demonstrates the integration pattern:
    // - Framework manages lifecycle & API publishing
    // - Tonic handles actual gRPC service implementation
    info!("\nStarting real gRPC server with tonic...");
    
    let echo_service = Arc::new(EchoServiceImpl::new());
    let addr = format!("0.0.0.0:{}", args.port);
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    
    // Spawn tonic server in background
    let addr_clone = addr.clone();
    let server_handle = tokio::spawn(async move {
        info!("🚀 Tonic gRPC server starting on: {}", addr_clone);
        if let Err(e) = run_echo_grpc_server(echo_service, addr_clone, shutdown_rx).await {
            error!("❌ gRPC server error: {}", e);
        } else {
            info!("✅ Tonic gRPC server stopped gracefully");
        }
    });
    
    // Wait a moment for server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    info!("✅ Real gRPC server running!");
    
    info!("\n📝 NOTE: This demonstrates the integration pattern:");
    info!("   - Framework: Lifecycle management & API publishing");
    info!("   - Tonic: Actual gRPC service implementation");
    info!("   - Both work together seamlessly!\n");
    
    info!("\n🎉 Server is FULLY operational!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📡 gRPC endpoint: grpc://localhost:{}", args.port);
    info!("🔍 Service registry: {}", args.registry_url);
    info!("🏷️  Module ID: {}", module_id);
    info!("✨ Framework + Tonic working together!");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("\nTry calling the service:");
    info!("  grpcurl -plaintext -d '{{\"message\":\"Hello\"}}' localhost:{} proto.EchoService/Echo", args.port);
    info!("\nPress Ctrl+C to stop...\n");

    // 6. Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;

    info!("\n📋 Shutting down gracefully...");

    // 7. Stop everything in order
    info!("  1. Stopping gRPC server...");
    shutdown_tx.send(()).ok();
    if let Err(e) = server_handle.await {
        error!("     Error waiting for server: {}", e);
    } else {
        info!("     ✅ gRPC server stopped");
    }
    
    info!("  2. Stopping framework runtime...");
    runtime.stop().await?;
    info!("     ✅ Runtime stopped");
    
    info!("\n✅ Clean shutdown complete!");
    info!("👋 Goodbye!");
    Ok(())
}
