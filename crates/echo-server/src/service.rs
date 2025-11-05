//! Echo service implementation.
//!
//! # Rust Learning Note
//!
//! This is the **business logic** layer - pure domain code!
//!
//! ## Key Characteristics
//!
//! 1. **Protocol-agnostic**: No HTTP, gRPC, or protocol knowledge
//! 2. **Pure async Rust**: Uses async/await naturally
//! 3. **Implements trait**: Type-safe interface
//! 4. **Testable**: Easy to unit test

use async_trait::async_trait;
use hsu_common::Result;
use echo_contract::EchoService;
use tracing::debug;

/// Echo service implementation.
///
/// # Example
///
/// ```rust
/// use echo_server::EchoServiceImpl;
/// use echo_contract::EchoService;
///
/// #[tokio::main]
/// async fn main() {
///     let service = EchoServiceImpl::new();
///     let result = service.echo("Hello!".to_string()).await.unwrap();
///     assert_eq!(result, "Hello!");
/// }
/// ```
pub struct EchoServiceImpl {
    // In a real service, this might have:
    // - Database connections
    // - Cache clients
    // - Configuration
    // - Metrics
}

impl EchoServiceImpl {
    /// Creates a new echo service.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EchoServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EchoService for EchoServiceImpl {
    /// Echoes the input message.
    ///
    /// # Rust Learning Note
    ///
    /// ## Async Trait Implementation
    ///
    /// ```rust
    /// #[async_trait]
    /// impl EchoService for EchoServiceImpl {
    ///     async fn echo(&self, message: String) -> Result<String> {
    ///         // Pure business logic!
    ///         Ok(message)
    ///     }
    /// }
    /// ```
    ///
    /// **No protocol knowledge** - this is protocol-agnostic!
    ///
    /// The same service works with:
    /// - Direct calls (in-process)
    /// - gRPC (cross-process)
    /// - HTTP (future)
    /// - Any other protocol!
    async fn echo(&self, message: String) -> Result<String> {
        debug!("EchoService::echo called with: {}", message);
        
        // Business logic goes here
        // For echo, it's trivial, but imagine:
        // - Validation
        // - Database access
        // - External API calls
        // - Complex computations
        
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo_service() {
        let service = EchoServiceImpl::new();
        
        let result = service.echo("Hello, Rust!".to_string()).await.unwrap();
        assert_eq!(result, "Hello, Rust!");
    }

    #[tokio::test]
    async fn test_echo_empty() {
        let service = EchoServiceImpl::new();
        
        let result = service.echo("".to_string()).await.unwrap();
        assert_eq!(result, "");
    }

    #[tokio::test]
    async fn test_echo_unicode() {
        let service = EchoServiceImpl::new();
        
        let result = service.echo("🦀 Rust! 🚀".to_string()).await.unwrap();
        assert_eq!(result, "🦀 Rust! 🚀");
    }
}

