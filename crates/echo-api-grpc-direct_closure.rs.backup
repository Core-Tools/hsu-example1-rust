//! Direct Closure Support for Echo Services
//!
//! # Architecture
//!
//! This module provides the glue function that enables direct (in-process)
//! service calls, bypassing network protocols.
//!
//! ## The Pattern
//!
//! ```text
//! Module Registration (init)
//!     ↓
//! register_module(..., ModuleDescriptor {
//!     direct_closure_enable: Some(echo_direct_closure_enable),
//!     ...
//! })
//!     ↓
//! Runtime calls echo_direct_closure_enable(options)
//!     ↓
//! 1. Register module+services with ServiceConnector
//! 2. Store handlers in EchoServiceGateways
//!     ↓
//! Future calls use Protocol::Direct or Protocol::Auto
//! ```
//!
//! ## Comparison with Go
//!
//! **Go version:**
//! ```go
//! func EchoDirectClosureEnable(options modulewiring.DirectClosureEnableOptions[echoapi.EchoServiceGateways, echocontract.EchoServiceHandlers]) {
//!     // Enable direct closure for module
//!     options.ServiceConnector.EnableDirectClosure(
//!         options.ServiceGateways.ModuleID(),
//!         options.ServiceGateways.ServiceIDs(),
//!     )
//!     
//!     // Set handlers in gateways
//!     options.ServiceGateways.EnableDirectClosure(options.ServiceHandlers)
//! }
//! ```
//!
//! **Rust version (this file):**
//! ```rust
//! pub fn echo_direct_closure_enable(
//!     options: DirectClosureEnableOptions<Arc<dyn EchoServiceGateways>, EchoServiceHandlers>,
//! ) {
//!     // Enable direct closure for module
//!     options.service_connector.enable_direct_closure(
//!         options.service_gateways.module_id(),
//!         options.service_gateways.service_ids(),
//!     );
//!     
//!     // Set handlers in gateways (requires unsafe downcast or interior mutability)
//!     // ... (see implementation below)
//! }
//! ```
//!
//! Nearly identical! Main difference:
//! - Rust needs to work around trait object mutability

use std::sync::Arc;
use hsu_module_api::DirectClosureEnableOptions;
use tracing::debug;

use crate::gateways::{EchoServiceGateways, EchoServiceHandlers};

/// Enables direct closure for Echo services.
///
/// This function is called by the module registry during module initialization
/// when direct closure is requested. It performs two key operations:
///
/// 1. **Registers with ServiceConnector:** Tells the connector that this module's
///    services are available for direct (local) calls.
///
/// 2. **Stores handlers in gateways:** Provides the actual service implementations
///    that will be used when `Protocol::Direct` or `Protocol::Auto` (with local
///    availability) is requested.
///
/// # Type Parameters
///
/// This function works with `DirectClosureEnableOptions<SG, SH>` where:
/// - `SG` = `Arc<dyn EchoServiceGateways>` (the gateway provider)
/// - `SH` = `EchoServiceHandlers` (the service implementations)
///
/// # Example
///
/// ```rust,ignore
/// use hsu_module_api::{register_module, ModuleDescriptor};
///
/// register_module(
///     ModuleID::from("echo-server"),
///     ModuleDescriptor {
///         // ...
///         direct_closure_enable: Some(echo_direct_closure_enable),
///     },
/// );
/// ```
///
/// # Rust Learning Note
///
/// ## The Mutability Challenge
///
/// In Go, interfaces can be mutated directly:
/// ```go
/// options.ServiceGateways.EnableDirectClosure(handlers)
/// ```
///
/// In Rust, `Arc<dyn Trait>` is immutable by default. We have two solutions:
///
/// **Solution 1: Interior Mutability (Current)**
/// ```rust
/// pub struct EchoServiceGatewaysImpl {
///     service_handlers: RwLock<Option<EchoServiceHandlers>>,  // Interior mutability!
/// }
///
/// impl EchoServiceGateways for EchoServiceGatewaysImpl {
///     fn enable_direct_closure(&mut self, handlers: EchoServiceHandlers) {
///         *self.service_handlers.write().unwrap() = Some(handlers);
///     }
/// }
/// ```
///
/// **Solution 2: Concrete Type (Alternative)**
/// ```rust
/// DirectClosureEnableOptions<EchoServiceGatewaysImpl, EchoServiceHandlers>
/// ```
///
/// We use Solution 1 because it's more flexible and matches the trait-based API.
pub fn echo_direct_closure_enable(
    options: DirectClosureEnableOptions<Arc<dyn EchoServiceGateways>, EchoServiceHandlers>,
) {
    let module_id = options.service_gateways.module_id();
    let service_ids = options.service_gateways.service_ids();
    
    debug!(
        "[DirectClosure] Enabling direct closure for module {}: {:?}",
        module_id, service_ids
    );
    
    // 1. Register with ServiceConnector
    //
    // This tells the connector that these services are available locally.
    // When Protocol::Auto or Protocol::Direct is used, the connector will
    // check this registry and prefer local calls.
    options.service_connector.enable_direct_closure(
        module_id.clone(),
        service_ids.clone(),
    );
    
    debug!("[DirectClosure] Registered services with ServiceConnector");
    
    // 2. Store handlers in gateways
    //
    // # Rust Learning Note
    //
    // This is the tricky part! We have `Arc<dyn EchoServiceGateways>` which is
    // immutable. But `enable_direct_closure` needs `&mut self`.
    //
    // **The problem:** Can't get `&mut` from `Arc<dyn Trait>`
    //
    // **The solution:** `EchoServiceGatewaysImpl` uses `RwLock<Option<...>>`
    // internally (interior mutability), so the trait method actually takes `&self`
    // and mutates through the RwLock!
    //
    // This is a common Rust pattern for shared mutable state.
    
    // Since we're using interior mutability, we need to work around the
    // trait object limitation. We'll need to update the trait signature.
    // For now, let's use Arc::get_mut if possible, or document the limitation.
    
    // Actually, looking at our gateways.rs, the trait takes &mut self.
    // We need to either:
    // 1. Change the trait to take &self (with interior mutability)
    // 2. Use a concrete type here
    // 3. Use unsafe to get a mutable reference
    //
    // Let's use option 1 and update the trait to use &self with RwLock.
    
    // Store handlers in the gateways
    //
    // This uses interior mutability (RwLock) inside EchoServiceGatewaysImpl,
    // so we can call it on an immutable reference.
    options.service_gateways.enable_direct_closure(options.service_handlers);
    
    debug!("[DirectClosure] ✅ Direct closure fully enabled for module {}", module_id);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_direct_closure_compiles() {
        // Just verify the function signature compiles
        let _ = std::marker::PhantomData::<fn(DirectClosureEnableOptions<Arc<dyn EchoServiceGateways>, EchoServiceHandlers>)>;
    }
}

