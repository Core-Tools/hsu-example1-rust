//! Service Provider for Echo Server Module
//!
//! # Architecture
//!
//! This is the **server-specific** service provider!
//! - Provides: EchoServiceHandlers (for registration)
//! - Does NOT provide: EchoServiceGateways (server doesn't need them!)

/// Service provider for Echo server module.
///
#[derive(Clone)]
pub struct EchoServerServiceProvider {}
