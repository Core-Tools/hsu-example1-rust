//! Echo Client Module (Layers 3 + 5)
//!
//! # Architecture
//!
//! This crate contains the **client module** for the Echo service.
//!
//! ## Layer Separation
//!
//! - **Layer 3 (Module/Domain)**: `module.rs` - Module behavior
//! - **Layer 5 (Module Wiring)**: `wiring.rs` - Module self-registration
//! - **Layer 5 (Service Provider)**: `service_provider.rs` - Service access
//!
//! ## Why Separate from echo-server?
//!
//! **Per-Module Wiring!**
//! - echo-client: Provides gateways, no handlers
//! - echo-server: Provides handlers, no gateways
//!
//! This matches the Golang design!
//!
//! ## Golang Equivalent
//!
//! - Domain: `pkg/echoclient/echoclientdomain/module.go`
//! - Wiring: `pkg/echoclient/echoclientwiring/wiring.go`

pub mod module;
pub mod service_provider;
pub mod wiring;

pub use module::EchoClientModule;
pub use service_provider::EchoClientServiceProvider;
pub use wiring::{init_echo_client_module, EchoClientModuleConfig};

