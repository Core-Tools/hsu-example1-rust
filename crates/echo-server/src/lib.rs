//! Echo Server Module (Layers 3 + 5)
//!
//! # Architecture
//!
//! This crate contains the **server module** for the Echo service.
//!
//! ## Layer Separation
//!
//! - **Layer 3 (Module/Domain)**: `module.rs` + `service.rs` - Module behavior & business logic
//! - **Layer 5 (Module Wiring)**: `wiring.rs` - Module self-registration
//! - **Layer 5 (Service Provider)**: `service_provider.rs` - Service registration
//!
//! ## Why Separate from echo-client?
//!
//! **Per-Module Wiring!**
//! - echo-server: Provides handlers, no gateways
//! - echo-client: Provides gateways, no handlers
//!
//! This matches the Golang design!
//!
//! ## Golang Equivalent
//!
//! - Domain: `pkg/echoserver/echoserverdomain/module.go`
//! - Wiring: `pkg/echoserver/echoserverwiring/wiring.go`

pub mod module;
pub mod service_provider;
pub mod service;
pub mod wiring;

pub use module::EchoServerModule;
pub use service_provider::EchoServerServiceProvider;
pub use service::EchoServiceImpl;
pub use wiring::{init_echo_server_module, EchoServerModuleConfig};

