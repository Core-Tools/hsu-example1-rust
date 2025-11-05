//! Echo service domain logic.
//!
//! This is a simple echo service that demonstrates:
//! - Domain business logic
//! - Module trait implementation
//! - Service handlers
//!
//! # Architecture Note
//!
//! This crate is **protocol-agnostic**:
//! - Contains pure business logic
//! - No knowledge of gRPC, HTTP, or other protocols
//! - Defines domain service traits and implementations
//!
//! ## What Goes Here
//!
//! 1. ✅ Business logic (`EchoService` trait + impl)
//! 2. ✅ Domain models
//! 3. ✅ Server module (protocol-agnostic)
//! 4. ✅ Domain-specific validation
//!
//! ## What Goes in API/Protocol Layer
//!
//! - ❌ gRPC adapters → in `echo-api-grpc`
//! - ❌ HTTP adapters → in `echo-api-http`
//! - ❌ Extension traits → in `echo-api-grpc`, `echo-api-http`, etc.
//! - ❌ Protocol-specific code → in protocol adapter crates

// TODO(Phase 8): Refactor module.rs to use new architecture
// pub mod module;
pub mod service;

// pub use module::EchoModule;
pub use service::EchoServiceImpl;

// Re-export the trait from contract for convenience
pub use echo_contract::EchoService;

