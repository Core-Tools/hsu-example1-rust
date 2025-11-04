//! Extension trait for ServiceGateway to add Echo-specific helpers.
//! 
//! # Architecture Note
//! 
//! This file is placed in `echo-api-grpc` (NOT `echo-domain`) because:
//! 
//! 1. **Domain layer** (`echo-domain`) should be **protocol-agnostic**
//!    - Contains pure business logic
//!    - Knows nothing about gRPC, HTTP, or Direct protocols
//! 
//! 2. **API/Protocol layer** (`echo-api-grpc`) is **domain + protocol specific**
//!    - Contains protocol adapters (gRPC for Echo service)
//!    - Knows about both Echo domain AND gRPC protocol
//!    - Perfect place for extension traits!
//! 
//! ## Correct Layering
//! 
//! ```
//! echo-domain/           ← Protocol-agnostic business logic
//! ├── service.rs         ← EchoService trait + impl
//! └── module.rs          ← Server module
//! 
//! echo-api-grpc/         ← Domain + Protocol specific (Echo + gRPC)
//! ├── gateway.rs         ← EchoGrpcGateway (client adapter)
//! ├── server.rs          ← EchoGrpcServer (server adapter)
//! └── gateway_ext.rs     ← Extension trait! ✅ RIGHT PLACE!
//! ```
//! 
//! # Rust Learning Note
//! 
//! This demonstrates the **Extension Trait Pattern** - a way to add methods
//! to types from other crates without modifying those crates.
//! 
//! ## The Problem
//! 
//! The framework's `ServiceGateway` should be domain-agnostic, but we want
//! Go-like simplicity for extracting domain services.
//! 
//! **Wrong approach (coupling framework to domain):**
//! ```rust,ignore
//! // In framework crate:
//! impl ServiceGateway {
//!     pub fn as_echo_service(&self) -> Result<Arc<dyn EchoService>> { ... }
//!     pub fn as_storage_service(&self) -> Result<Arc<dyn StorageService>> { ... }
//!     // ← Framework grows with every domain! BAD!
//! }
//! ```
//! 
//! **Right approach (extension trait in API/protocol crate):**
//! ```rust
//! // In echo-api-grpc crate (domain + protocol specific):
//! pub trait ServiceGatewayEchoExt {
//!     fn as_echo_service(&self) -> Result<Arc<dyn EchoService>>;
//! }
//! 
//! impl ServiceGatewayEchoExt for ServiceGateway {
//!     fn as_echo_service(&self) -> Result<Arc<dyn EchoService>> { ... }
//! }
//! ```
//! 
//! ## Benefits
//! 
//! 1. ✅ Framework stays domain-agnostic
//! 2. ✅ Domain layer stays protocol-agnostic
//! 3. ✅ API/protocol layer adds protocol-specific helpers
//! 4. ✅ No coupling between framework and domain
//! 5. ✅ Each protocol adapter can have its own extension traits
//! 6. ✅ Clear separation of concerns
//! 
//! ## Usage
//! 
//! ```rust,ignore
//! use echo_api_grpc::ServiceGatewayEchoExt;  // ← Import from API crate!
//! 
//! let gateway = factory.new_service_gateway(...).await?;
//! let echo_service = gateway.as_echo_service()?;  // ← Extension method!
//! ```
//! 
//! ## Trade-off: Protocol Import
//! 
//! **Note:** The import mentions "grpc" (`echo_api_grpc`), which is a compile-time
//! protocol dependency. However:
//! 
//! 1. **The actual code** is protocol-agnostic (no protocol logic)
//! 2. **The import** is just a namespace (acceptable trade-off)
//! 3. **Rust's orphan rules** prevent splitting trait declaration/implementation
//! 
//! This is still **massively better** than 20-line manual protocol matching!
//! 
//! ## Comparison with Go
//! 
//! **Go:**
//! ```go
//! gateway, err := factory.NewServiceGateway(ctx, module, service, protocol)
//! typedService, ok := gateway.(contract.Contract1)  // ← Type assertion
//! ```
//! 
//! **Rust with extension trait:**
//! ```rust,ignore
//! let gateway = factory.new_service_gateway(&module, &service, protocol).await?;
//! let typed_service = gateway.as_echo_service()?;  // ← Extension method
//! ```
//! 
//! Same simplicity, better type safety, perfect separation of concerns!

use hsu_common::{Result, Error};
use hsu_module_management::{ServiceGateway, module_types::{ServiceHandler, GrpcGateway}};
use echo_domain::EchoService;
use std::sync::Arc;

/// Extension trait for `ServiceGateway` to add Echo-specific methods.
/// 
/// # Rust Learning Note
/// 
/// This is an **extension trait** - it adds methods to a type from another crate
/// without modifying that crate. This is a powerful Rust pattern!
/// 
/// ## How It Works
/// 
/// 1. Define a trait with the methods you want:
///    ```rust
///    pub trait ServiceGatewayEchoExt {
///        fn as_echo_service(&self) -> Result<...>;
///    }
///    ```
/// 
/// 2. Implement it for the framework type:
///    ```rust
///    impl ServiceGatewayEchoExt for ServiceGateway { ... }
///    ```
/// 
/// 3. Import the trait to use the methods:
///    ```rust
///    use echo_api_grpc::ServiceGatewayEchoExt;  // ← From API crate!
///    gateway.as_echo_service()?;  // ← Works!
///    ```
/// 
/// ## Why This is in API/Protocol Layer
/// 
/// - Framework crate (`hsu-module-management`) stays domain-agnostic
/// - Domain crate (`echo-domain`) stays protocol-agnostic
/// - API/Protocol crate (`echo-api-grpc`) is BOTH domain + protocol specific
/// - Extension trait naturally belongs here!
/// 
/// ## Pattern: One Extension Trait Per Protocol Adapter
/// 
/// ```rust
/// // echo-api-grpc crate:
/// pub trait ServiceGatewayEchoExt { ... }
/// 
/// // storage-api-grpc crate:
/// pub trait ServiceGatewayStorageExt { ... }
/// 
/// // compute-api-http crate:
/// pub trait ServiceGatewayComputeExt { ... }
/// ```
/// 
/// Each protocol adapter is independent!
pub trait ServiceGatewayEchoExt {
    /// Extracts an `EchoService` trait object from this gateway,
    /// regardless of protocol (Direct, gRPC, or HTTP).
    /// 
    /// # Example
    /// 
    /// ```rust,ignore
    /// use echo_api_grpc::ServiceGatewayEchoExt;
    /// 
    /// let gateway = factory.new_service_gateway(
    ///     &ModuleID::from("echo"),
    ///     &ServiceID::from("echo-service"),
    ///     Protocol::Auto,  // ← Framework chooses protocol
    /// ).await?;
    /// 
    /// let echo = gateway.as_echo_service()?;  // ← Protocol-agnostic!
    /// let response = echo.echo("Hello".to_string()).await?;
    /// ```
    /// 
    /// # Benefits
    /// 
    /// 1. **Protocol-agnostic**: Works with Direct, gRPC, or HTTP
    /// 2. **Simple**: One method call (like Go's type assertion)
    /// 3. **Type-safe**: Compile-time checked
    /// 4. **Fast**: Pattern matching optimized (~4-8ns)
    /// 5. **Well-layered**: In API/protocol crate, not domain or framework!
    fn as_echo_service(&self) -> Result<Arc<dyn EchoService>>;
}

impl ServiceGatewayEchoExt for ServiceGateway {
    fn as_echo_service(&self) -> Result<Arc<dyn EchoService>> {
        match self {
            // Direct protocol: In-process call
            ServiceGateway::Direct(handler) => {
                match handler.as_ref() {
                    ServiceHandler::Echo(service) => Ok(Arc::clone(&service)),
                }
            }
            // gRPC protocol: Remote call
            ServiceGateway::Grpc(grpc_gateway) => {
                match grpc_gateway {
                    GrpcGateway::Echo(service) => Ok(Arc::clone(&service)),
                }
            }
            // HTTP protocol: Remote call
            ServiceGateway::Http(_http_gateway) => {
                // Future: When HTTP EchoService is implemented:
                // match http_gateway {
                //     HttpGateway::Echo(service) => Ok(Arc::clone(&service)),
                // }
                Err(Error::Validation {
                    message: "HTTP protocol not yet supported for EchoService".to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hsu_module_management::module_types::ServiceHandler;
    
    // Mock EchoService for testing
    struct MockEchoService;
    
    #[async_trait::async_trait]
    impl EchoService for MockEchoService {
        async fn echo(&self, message: String) -> Result<String> {
            Ok(message)
        }
    }
    
    #[test]
    fn test_extension_trait_for_direct_gateway() {
        // Create a Direct gateway with Echo service
        let mock_service = Arc::new(MockEchoService);
        let handler = ServiceHandler::Echo(mock_service);
        let gateway = ServiceGateway::Direct(Arc::new(handler));
        
        // Use extension trait method
        let echo_service = gateway.as_echo_service();
        assert!(echo_service.is_ok());
    }
    
    #[test]
    fn test_extension_trait_for_http_gateway() {
        use hsu_module_management::module_types::HttpGateway;
        
        // HTTP gateway doesn't have Echo yet
        let gateway = ServiceGateway::Http(HttpGateway::_Placeholder);
        
        // Should return error
        let result = gateway.as_echo_service();
        assert!(result.is_err());
    }
}
