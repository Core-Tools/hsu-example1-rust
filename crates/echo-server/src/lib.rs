//! Echo Server Module (Layer 5 - Application Glue + Layer 6 - Wiring)
//!
//! # Architecture
//!
//! This crate contains the **server module** for the Echo service.
//! It includes:
//! - Module initialization
//! - Service provider implementation (server-specific!)
//! - Handler registration
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
//! `cmd/srv/echogrpcsrv/module.go`

pub mod module;
pub mod service_provider;

pub use module::{init_echo_server_module, EchoServerModuleConfig};
pub use service_provider::EchoServerServiceProvider;

