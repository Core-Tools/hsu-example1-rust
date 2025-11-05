//! Echo gRPC Client - NEW ARCHITECTURE! ✨
//!
//! # What This Demonstrates
//!
//! 1. **Init-based Registration**: Modules self-register at startup
//! 2. **Declarative Configuration**: Simple config-based wiring
//! 3. **Minimal Main**: <60 lines (matches Golang!)
//! 4. **gRPC Communication**: Client → Server via gRPC protocol
//!
//! # Architecture (NEW!)
//!
//! ```
//! main.rs (this file)
//!     ↓ calls
//! echo_client::init()
//!     ↓ registers descriptor
//! Framework Registry
//!     ↓ framework calls
//! create_service_provider() → create_module()
//!     ↓ creates
//! Running client module!
//! ```
//!
//! # Comparison with Golang
//!
//! **Go version:** (`hsu-example1-go/cmd/cli/echogrpccli/main.go`)
//! ```go
//! func main() {
//!     config := &modulewiring.Config{
//!         Modules: []modulewiring.ModuleConfig{
//!             {ID: "echo-client", Enabled: true},
//!         },
//!     }
//!     modulewiring.RunWithConfig(config, logger)
//! }
//! ```
//!
//! **Rust version:** (this file - similar pattern!)

use hsu_common::{ModuleID, Result};
use hsu_module_api::{Config, ModuleConfig, RuntimeConfig, ServiceRegistryConfig, run_with_config};
use clap::Parser;

use echo_client::{init_echo_client_module, EchoClientModuleConfig};

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about = "Echo gRPC Client - NEW ARCHITECTURE")]
struct Args {
    /// Service registry URL
    #[arg(short, long, default_value = "http://localhost:8080")]
    registry_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();
    
    init_echo_client_module(EchoClientModuleConfig::default())?;
    
    let config = Config {
        runtime: RuntimeConfig {
            service_registry: ServiceRegistryConfig {
                url: args.registry_url,
            },
            servers: vec![],
        },
        modules: vec![
            ModuleConfig {
                id: ModuleID::from("echo-client"),
                enabled: true,
                servers: vec![],
            },
        ],
    };
    
    run_with_config(config).await
}
