//! gRPC protocol adapters for Echo service.
//!
//! This crate demonstrates the **Adapter Pattern** - bridging gRPC to domain logic!
//!
//! # Architecture Note
//!
//! This crate is **domain + protocol specific** (Echo + gRPC):
//! - Contains gRPC adapters for Echo service
//! - Contains extension traits for Go-like gateway usage
//! - Bridges between protocol (gRPC) and domain (Echo business logic)
//!
//! ## What Goes Here
//!
//! 1. ✅ gRPC server adapter (`EchoGrpcHandler`)
//! 2. ✅ gRPC client adapter (`EchoGrpcGateway`)
//! 3. ✅ Extension traits (`ServiceGatewayEchoExt`)
//! 4. ✅ Protocol-specific code (protobuf, tonic)
//!
//! ## What Stays in Domain Layer
//!
//! - ❌ Business logic (`EchoService` trait + impl) → in `echo-domain`
//! - ❌ Domain models → in `echo-domain`
//! - ❌ Protocol-agnostic code → in `echo-domain`

pub mod generated {
    //! Generated gRPC code from protobuf.
    tonic::include_proto!("proto");
}

pub mod handler;
pub mod gateway;
pub mod gateway_ext;
pub mod server;

pub use handler::EchoGrpcHandler;
pub use gateway::{EchoGrpcGateway, EchoGrpcGatewayFactory};
pub use gateway_ext::ServiceGatewayEchoExt;
pub use server::{run_echo_grpc_server, spawn_echo_grpc_server};

