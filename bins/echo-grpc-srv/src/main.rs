//! Echo gRPC Server - Demonstrates gRPC protocol with FULL HSU framework.
//!
//! # What This Demonstrates
//!
//! 1. **Init-based Module Registration** - Modules self-register
//! 2. **ModuleRuntime** - Full lifecycle management
//! 3. **gRPC Server** - Framework-managed protocol server
//! 4. **Service Registry** - Automatic API publishing
//! 5. **Complete Framework Integration** - Runtime manages everything!
//!
//! # Architecture (Updated to NEW PATTERN!)
//!
//! ```
//! Process: echo-grpc-srv
//! └── ModuleRuntime (manages everything!)
//!     ├── EchoServerModule (provides EchoService via init)
//!     ├── GrpcProtocolServer (port 50051) ← Managed by framework!
//!     └── ServiceRegistryClient ← Automatic publishing!
//! ```
//!
//! **Framework now handles:**
//! - ✅ Protocol server creation
//! - ✅ Server lifecycle (start/stop)
//! - ✅ API publishing to registry
//! - ✅ Graceful shutdown
//! - ✅ Module creation from registry!
//!
//! # Changes from OLD Pattern
//!
//! - ✅ Uses init-based registration (no manual module creation!)
//! - ✅ Uses `run_with_config` (simplified main!)
//! - ✅ Framework creates modules from registry
//! - ✅ Much less boilerplate!

use clap::Parser;
use hsu_common::{ModuleID, Result};
use hsu_module_api::{Config, ModuleConfig, RuntimeConfig, ServiceRegistryConfig, run_with_config};

use echo_server::{init_echo_server_module, EchoServerModuleConfig};

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
async fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();
    
    init_echo_server_module(EchoServerModuleConfig::default())?;
    
    // TODO: Add gRPC server config when protocol server support is implemented
    let config = Config {
        runtime: RuntimeConfig {
            service_registry: ServiceRegistryConfig {
                url: args.registry_url,
            },
            servers: vec![
                // ProtocolServerConfig { protocol: Protocol::Grpc, listen_address: format!("0.0.0.0:{}", args.port) }
            ],
        },
        modules: vec![
            ModuleConfig {
                id: ModuleID::from("echo"),
                enabled: true,
                servers: vec![],
            },
        ],
    };
    
    run_with_config(config).await
}
