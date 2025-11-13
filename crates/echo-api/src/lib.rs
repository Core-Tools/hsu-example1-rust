//! Echo API - Reusable API Components (Layer 3/5 Boundary)
//!
//! # Architecture
//!
//! This crate contains **reusable implementations** that bridge protocol adapters
//! with application logic. It's used by both server and client modules.
//!
//! ## What's Here
//!
//! 1. ✅ `EchoServiceGatewaysImpl` - Reusable gateway provider
//! 2. ✅ `EchoHandlersRegistrar` - Reusable handler registrar
//! 3. ✅ `echo_direct_closure_enable` - Direct closure enabler
//!
//! ## Why Separate from echo-api-grpc?
//!
//! - echo-api-grpc: Thin protocol adapters (Layer 3)
//! - echo-api: Reusable implementations (Layer 3/5 boundary)
//!
//! This allows multiple modules to reuse the same implementations!
//!
//! ## Golang Equivalent
//!
//! `pkg/api/` (without `grpc/` and `contract/` subdirs)

pub mod gateways;
pub mod handlers;
pub mod direct_closure;

pub use gateways::{EchoServiceGatewaysImpl, new_echo_service_gateways};
pub use handlers::{EchoHandlersRegistrar, new_echo_handlers_registrar};
pub use direct_closure::echo_direct_closure_enabler;

