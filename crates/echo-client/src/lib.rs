//! Echo Client Module (Layer 5 - Application Glue + Layer 6 - Wiring)
//!
//! # Architecture
//!
//! This crate contains the **client module** for the Echo service.
//! It includes:
//! - Module initialization
//! - Service provider implementation (client-specific!)
//! - Business logic for calling Echo service
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
//! `cmd/cli/echoclient/module.go`

pub mod module;
pub mod service_provider;

pub use module::{init_echo_client_module, EchoClientModuleConfig, EchoClientModule};
pub use service_provider::EchoClientServiceProvider;

