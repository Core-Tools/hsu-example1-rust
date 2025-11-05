//! gRPC Protocol Adapters for Echo Service (Layer 3)
//!
//! **⚠️ PHASE 5 REFACTORING:**  
//! This crate now contains **ONLY** thin protocol adapters (factory functions).
//!
//! # What's Here (Layer 3 - Protocol Adapters)
//!
//! 1. ✅ gRPC server adapter (`EchoGrpcHandler`)
//! 2. ✅ gRPC client adapter (`EchoGrpcGateway`)
//! 3. ✅ Protocol-specific code (protobuf, tonic)
//! 4. ✅ Factory functions (thin wrappers)
//!
//! # What Moved Out
//!
//! ## To `echo-contract/` (Layer 3 - Contracts):
//! - ❌ `EchoService` trait (protocol-agnostic)
//! - ❌ `EchoServiceHandlers` struct
//! - ❌ `EchoServiceGateways` trait
//!
//! ## To `echo-api/` (Layer 3/5 Boundary - Shared Components):
//! - ❌ `gateways.rs` → `echo-api/src/gateways.rs` (EchoServiceGatewaysImpl)
//! - ❌ `handlers.rs` → `echo-api/src/handlers.rs` (EchoHandlersRegistrar)
//! - ❌ `direct_closure.rs` → `echo-api/src/direct_closure.rs`
//!
//! ## Deleted (Already in Framework):
//! - ❌ `server.rs` (Layer 1 code - use `hsu-module-proto::GrpcProtocolServer`)
//!
//! # Architecture
//!
//! ```text
//! Before (WRONG):
//!     echo-api-grpc/
//!     ├── gateway.rs      (Layer 3) ✅
//!     ├── handler.rs      (Layer 3) ✅
//!     ├── server.rs       (Layer 1) ❌ WRONG LAYER!
//!     ├── gateways.rs     (Layer 5) ❌ WRONG LAYER!
//!     ├── handlers.rs     (Layer 5) ❌ WRONG LAYER!
//!     └── direct_closure.rs (Layer 5) ❌ WRONG LAYER!
//!
//! After (CORRECT):
//!     echo-api-grpc/
//!     ├── gateway.rs      (Layer 3) ✅ Thin adapter
//!     └── handler.rs      (Layer 3) ✅ Thin adapter
//! ```

pub mod generated {
    //! Generated gRPC code from protobuf.
    tonic::include_proto!("proto");
}

pub mod handler;
pub mod gateway;

pub use handler::EchoGrpcHandler;
pub use gateway::{EchoGrpcGateway, EchoGrpcGatewayFactory};

