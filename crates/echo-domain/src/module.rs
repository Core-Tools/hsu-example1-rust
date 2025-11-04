//! Echo module implementation.
//!
//! # Rust Learning Note
//!
//! This demonstrates the **Module pattern** in HSU!
//!
//! ## Module Responsibilities
//!
//! 1. **Identity**: Unique module ID
//! 2. **Services**: Map of service handlers
//! 3. **Lifecycle**: Start/stop methods
//! 4. **Dependencies**: Gateway factory injection

use async_trait::async_trait;
use hsu_common::{ModuleID, ServiceID, Result};
use hsu_module_management::{Module, ServiceGatewayFactory, ServiceHandler, ServiceHandlersMap};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::{info, debug};

use crate::service::EchoServiceImpl;

/// Echo module.
///
/// # Architecture
///
/// ```
/// EchoModule
/// ├── ID: "echo"
/// ├── Services:
/// │   └── "echo-service" → EchoServiceImpl
/// └── Lifecycle:
///     ├── start() - Initialize
///     └── stop() - Cleanup
/// ```
pub struct EchoModule {
    /// Module ID
    id: ModuleID,
    
    /// Echo service implementation
    service: Arc<EchoServiceImpl>,
    
    /// Gateway factory (injected by runtime)
    gateway_factory: Option<Arc<dyn ServiceGatewayFactory>>,
}

impl EchoModule {
    /// Creates a new echo module.
    ///
    /// # Example
    ///
    /// ```rust
    /// use echo_domain::EchoModule;
    ///
    /// let module = EchoModule::new();
    /// println!("Module ID: {}", module.id());
    /// ```
    pub fn new() -> Self {
        info!("Creating EchoModule");
        
        Self {
            id: ModuleID::from("echo"),
            service: Arc::new(EchoServiceImpl::new()),
            gateway_factory: None,
        }
    }
}

impl Default for EchoModule {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Module for EchoModule {
    /// Returns the module ID.
    fn id(&self) -> &ModuleID {
        &self.id
    }

    /// Returns service handlers provided by this module.
    ///
    /// # Rust Learning Note
    ///
    /// ## Service Handlers Map
    ///
    /// ```rust
    /// fn service_handlers_map(&self) -> Option<ServiceHandlersMap> {
    ///     let mut map = HashMap::new();
    ///     map.insert(
    ///         ServiceID::from("echo-service"),
    ///         ServiceHandler::Echo(self.service.clone()),
    ///     );
    ///     Some(map)
    /// }
    /// ```
    ///
    /// **Key insight:** The map uses **enums** for type safety:
    /// - `ServiceID` - unique service identifier
    /// - `ServiceHandler` - enum with Arc-wrapped services
    ///
    /// **No casts needed!** Pattern matching handles types.
    fn service_handlers_map(&self) -> Option<ServiceHandlersMap> {
        let mut map = HashMap::new();
        
        // Register echo service
        map.insert(
            ServiceID::from("echo-service"),
            ServiceHandler::Echo(self.service.clone()),
        );
        
        Some(map)
    }

    /// Sets the gateway factory.
    ///
    /// # Rust Learning Note
    ///
    /// ## Dependency Injection
    ///
    /// The runtime injects the factory:
    ///
    /// ```rust
    /// // Runtime:
    /// module.set_service_gateway_factory(factory);
    ///
    /// // Module:
    /// fn set_service_gateway_factory(&mut self, factory: Arc<dyn ServiceGatewayFactory>) {
    ///     self.gateway_factory = Some(factory);
    /// }
    /// ```
    ///
    /// **Benefits:**
    /// - Module can call other modules
    /// - Testable (mock factory)
    /// - Loosely coupled
    fn set_service_gateway_factory(&mut self, factory: Arc<dyn ServiceGatewayFactory>) {
        debug!("Setting gateway factory for module: {}", self.id);
        self.gateway_factory = Some(factory);
    }

    /// Starts the module.
    ///
    /// # Rust Learning Note
    ///
    /// ## Async Lifecycle
    ///
    /// ```rust
    /// async fn start(&mut self) -> Result<()> {
    ///     // Initialize resources
    ///     // Start background tasks
    ///     // Connect to dependencies
    ///     Ok(())
    /// }
    /// ```
    ///
    /// **Called by runtime** during startup!
    async fn start(&mut self) -> Result<()> {
        info!("Starting module: {}", self.id);
        
        // In a real module, you might:
        // - Connect to database
        // - Start background workers
        // - Load configuration
        // - Initialize cache
        
        Ok(())
    }

    /// Stops the module.
    ///
    /// # Rust Learning Note
    ///
    /// ## Graceful Shutdown
    ///
    /// ```rust
    /// async fn stop(&mut self) -> Result<()> {
    ///     // Close connections
    ///     // Stop background tasks
    ///     // Flush buffers
    ///     Ok(())
    /// }
    /// ```
    ///
    /// **Called by runtime** during shutdown (in reverse order)!
    async fn stop(&mut self) -> Result<()> {
        info!("Stopping module: {}", self.id);
        
        // In a real module, you might:
        // - Close database connections
        // - Stop background workers
        // - Flush metrics
        // - Clean up resources
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let module = EchoModule::new();
        assert_eq!(module.id().as_str(), "echo");
    }

    #[test]
    fn test_module_has_services() {
        let module = EchoModule::new();
        let handlers = module.service_handlers_map();
        
        assert!(handlers.is_some());
        let handlers = handlers.unwrap();
        assert_eq!(handlers.len(), 1);
        assert!(handlers.contains_key(&ServiceID::from("echo-service")));
    }

    #[tokio::test]
    async fn test_module_lifecycle() {
        let mut module = EchoModule::new();
        
        // Start
        module.start().await.unwrap();
        
        // Stop
        module.stop().await.unwrap();
    }
}

