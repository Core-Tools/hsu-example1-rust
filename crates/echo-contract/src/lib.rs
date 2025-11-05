//! Echo Service Contracts (Layer 3)
//!
//! Protocol-agnostic service interfaces.
//!
//! # Architecture
//!
//! This crate defines the **contract** between service providers and consumers.
//! It is:
//! - **Protocol-agnostic**: No gRPC, HTTP, or protocol knowledge
//! - **Reusable**: Used by all protocols and modules
//! - **Type-safe**: Compile-time guarantees
//!
//! ## Comparison with Golang
//!
//! **Golang:**
//! ```go
//! package echocontract
//!
//! type Service1 interface {
//!     Echo1(ctx context.Context, message string) (string, error)
//! }
//!
//! type EchoServiceHandlers struct {
//!     Service1 Service1
//! }
//!
//! type EchoServiceGateways interface {
//!     GetService1(ctx context.Context, protocol Protocol) (Service1, error)
//! }
//! ```
//!
//! **Rust (this crate):**
//! ```rust,ignore
//! #[async_trait]
//! pub trait EchoService: Send + Sync {
//!     async fn echo(&self, message: String) -> Result<String>;
//! }
//!
//! pub struct EchoServiceHandlers {
//!     pub service: Arc<dyn EchoService>,
//! }
//!
//! #[async_trait]
//! pub trait EchoServiceGateways: Send + Sync {
//!     async fn get_service(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>>;
//! }
//! ```

use std::sync::Arc;
use async_trait::async_trait;
use hsu_common::{Result, ModuleID, ServiceID, Protocol};

/// Echo service contract (protocol-agnostic).
///
/// This trait defines the business interface without any protocol knowledge.
/// It can be implemented by:
/// - Domain layer (EchoServiceImpl)
/// - Protocol adapters (EchoGrpcGateway)
/// - Test mocks
///
/// # Rust Learning Note
///
/// ## Why Arc<dyn Trait>?
///
/// We use `Arc<dyn EchoService>` because:
/// 1. **Arc**: Thread-safe shared ownership (Send + Sync)
/// 2. **dyn**: Dynamic dispatch (trait object)
/// 3. **EchoService**: The contract interface
///
/// This allows us to pass different implementations at runtime!
#[async_trait]
pub trait EchoService: Send + Sync {
    /// Echoes the input message.
    ///
    /// This is pure business logic - no protocol knowledge!
    async fn echo(&self, message: String) -> Result<String>;
}

/// Service handlers provided by server module.
///
/// This struct holds the actual service implementations that will be
/// registered with protocol servers (gRPC, HTTP, etc.).
///
/// # Rust Learning Note
///
/// In Golang:
/// ```go
/// type EchoServiceHandlers struct {
///     Service1 Service1
/// }
/// ```
///
/// In Rust:
/// ```rust,ignore
/// pub struct EchoServiceHandlers {
///     pub service: Arc<dyn EchoService>,
/// }
/// ```
///
/// Same concept - holder for service implementations!
#[derive(Clone)]
pub struct EchoServiceHandlers {
    /// The echo service implementation
    pub service: Arc<dyn EchoService>,
}

impl EchoServiceHandlers {
    /// Creates new service handlers.
    pub fn new(service: Arc<dyn EchoService>) -> Self {
        Self { service }
    }
}

/// Service gateways provided by wiring layer.
///
/// This trait defines how to get service instances with different protocols.
/// It's implemented by the wiring layer and used by client modules.
///
/// # Rust Learning Note
///
/// ## The Gateway Pattern
///
/// ```text
/// Client Module
///     ↓ asks for
/// EchoServiceGateways
///     ↓ returns
/// Arc<dyn EchoService>
///     ↓ client uses
/// .echo("Hello!")
/// ```
///
/// The client doesn't know or care if it's:
/// - Direct (local call)
/// - gRPC (remote call)
/// - HTTP (future)
///
/// Protocol selection is transparent!
#[async_trait]
pub trait EchoServiceGateways: Send + Sync {
    /// Returns the target module ID (e.g., "echo").
    fn module_id(&self) -> ModuleID;
    
    /// Returns the list of service IDs provided.
    fn service_ids(&self) -> Vec<ServiceID>;
    
    /// Enables direct closure (local calls) by registering handlers.
    ///
    /// This is called during module initialization to enable
    /// in-process calls without going through gRPC/HTTP.
    fn enable_direct_closure(&self, handlers: EchoServiceHandlers);
    
    /// Gets the echo service using the specified protocol.
    ///
    /// # Arguments
    ///
    /// * `protocol` - Protocol to use (Direct, Grpc, Http, Auto)
    ///
    /// # Returns
    ///
    /// A trait object that implements `EchoService`.
    ///
    /// # Rust Learning Note
    ///
    /// This is equivalent to Golang's:
    /// ```go
    /// func (g *EchoServiceGateways) GetService1(ctx context.Context, protocol Protocol) (Service1, error)
    /// ```
    ///
    /// Both return an interface/trait that the caller can use!
    async fn get_service(&self, protocol: Protocol) -> Result<Arc<dyn EchoService>>;
}

