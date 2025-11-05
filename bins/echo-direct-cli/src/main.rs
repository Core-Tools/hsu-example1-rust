//! Echo Direct CLI - Demonstrates in-process direct communication.
//!
//! # What This Demonstrates
//!
//! 1. **Direct Protocol**: Zero-cost in-process calls (~6 cycles!)
//! 2. **Init-based Registration**: Modules self-register at startup
//! 3. **Declarative Configuration**: Simple config-based wiring
//! 4. **Minimal Main**: <60 lines (matches Golang!)
//!
//! # Architecture (NEW!)
//!
//! ```
//! main.rs (this file)
//!     ↓ calls
//! echo_server::init() + echo_client::init()
//!     ↓ registers descriptors
//! Framework Registry
//!     ↓ framework calls
//! create_service_provider() → create_module()
//!     ↓ creates
//! Running modules!
//! ```
//!
//! # Comparison with Golang
//!
//! **Go version:**
//! ```go
//! func main() {
//!     config := &modulewiring.Config{
//!         Modules: []modulewiring.ModuleConfig{
//!             {ID: "echo", Enabled: true},
//!             {ID: "echo-client", Enabled: true},
//!         },
//!     }
//!     modulewiring.RunWithConfig(config, logger)
//! }
//! ```
//!
//! **Rust version:** (this file - similar pattern!)

use hsu_module_api::{Config, ModuleConfig, run_with_config};
use hsu_common::{ModuleID, Result};

use echo_server::{init_echo_server_module, EchoServerModuleConfig};
use echo_client::{init_echo_client_module, EchoClientModuleConfig};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    // Register modules
    init_echo_server_module(EchoServerModuleConfig::default())?;
    init_echo_client_module(EchoClientModuleConfig::default())?;
    
    // Configure and run
    let config = Config {
        runtime: Default::default(),
        modules: vec![
            ModuleConfig {
                id: ModuleID::from("echo"),
                enabled: true,
                servers: vec![],
            },
            ModuleConfig {
                id: ModuleID::from("echo-client"),
                enabled: true,
                servers: vec![],
            },
        ],
    };
    
    run_with_config(config).await
}
