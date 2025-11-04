# HSU Echo Examples - Rust

Complete demonstration of the HSU microservice framework in Rust!

## 🎯 What This Demonstrates

This project shows **three communication patterns** in the HSU framework:

1. **Direct Communication** (`echo-direct-cli`)
   - In-process, zero-cost function calls
   - ~6 CPU cycles overhead
   - Perfect for monolithic deployments

2. **gRPC Server** (`echo-grpc-srv`)
   - Cross-process communication
   - Full HSU framework integration
   - Service registry publishing
   - Production-ready patterns

3. **gRPC Client** (`echo-grpc-cli`)
   - Service discovery from registry
   - Automatic protocol selection
   - Fallback to direct connection

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Service Registry                       │
│                   (hsu-registry on :8080)                   │
└─────────────────────────────────────────────────────────────┘
            ▲                              │
            │ Publish                      │ Discover
            │                              ▼
┌───────────────────────┐       ┌───────────────────────────┐
│  echo-grpc-srv        │       │  echo-grpc-cli            │
│  ─────────────────    │       │  ─────────────────        │
│  • EchoModule         │       │  • Service Discovery      │
│  • ModuleRuntime      │◄──────┤  • Gateway Factory        │
│  • gRPC Server        │  gRPC │  • Remote Calls           │
└───────────────────────┘       └───────────────────────────┘
```

## 📁 Project Structure

```
hsu-example1-rust/
├── crates/
│   ├── echo-domain/          # Business logic (EchoService)
│   │   ├── src/
│   │   │   ├── service.rs    # EchoService trait & impl
│   │   │   └── module.rs     # EchoModule (HSU module)
│   │   └── Cargo.toml
│   │
│   └── echo-api-grpc/        # gRPC protocol adapters
│       ├── api/proto/        # Protocol buffer definitions
│       ├── src/
│       │   ├── handler.rs    # Server-side adapter
│       │   ├── gateway.rs    # Client-side adapter
│       │   └── server.rs     # Server utilities
│       ├── build.rs          # Proto compilation
│       └── Cargo.toml
│
└── bins/                     # Example applications
    ├── echo-direct-cli/      # Direct communication demo
    ├── echo-grpc-srv/        # gRPC server with framework
    └── echo-grpc-cli/        # gRPC client with discovery
```

## 🚀 Quick Start

### Prerequisites

You need Rust 1.70+ and Protocol Buffers compiler:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install protoc (protocol buffers compiler)
# On macOS:
brew install protobuf

# On Ubuntu/Debian:
sudo apt install protobuf-compiler

# On Windows:
# Download from https://github.com/protocolbuffers/protobuf/releases
```

### Build

```bash
cd hsu-example1-rust
cargo build --workspace --release
```

### Run Examples

#### 1. Direct Communication (Simplest)

```bash
cargo run --release --bin echo-direct-cli
```

**What you'll see:**
- Two modules in one process
- Direct function calls (zero serialization)
- ~6 CPU cycles overhead
- Perfect type safety

---

#### 2. gRPC Communication (Full Framework)

**Terminal 1** - Start service registry:
```bash
cd ../hsu-core/rust
cargo run --release --bin hsu-registry
```

**Terminal 2** - Start gRPC server:
```bash
cd hsu-example1-rust
cargo run --release --bin echo-grpc-srv
```

**Terminal 3** - Run client:
```bash
cd hsu-example1-rust
cargo run --release --bin echo-grpc-cli
```

**What you'll see:**
- Server publishes to registry
- Client discovers from registry
- Cross-process gRPC calls
- Graceful shutdown

---

### Custom Options

#### Server
```bash
# Custom port
cargo run --release --bin echo-grpc-srv -- --port 9090

# Custom registry
cargo run --release --bin echo-grpc-srv -- --registry-url http://localhost:9000
```

#### Client
```bash
# Custom message
cargo run --release --bin echo-grpc-cli -- --message "Hello HSU!"

# Direct connection (skip registry)
cargo run --release --bin echo-grpc-cli -- --direct-address localhost:50051
```

## 🎓 Learning Path

If you're new to the HSU framework or Rust, follow this order:

1. **Start with Direct CLI** (`echo-direct-cli`)
   - Understand modules and services
   - See zero-cost abstraction
   - Learn HSU basics

2. **Read the Code** (in this order)
   - `echo-domain/src/service.rs` - Business logic
   - `echo-domain/src/module.rs` - HSU module
   - `echo-api-grpc/src/handler.rs` - gRPC adapter (server)
   - `echo-api-grpc/src/gateway.rs` - gRPC adapter (client)

3. **Try gRPC Examples**
   - Run server and client
   - See service registry in action
   - Experiment with options

4. **Study the Framework**
   - Read `hsu-core/rust` source code
   - Check documentation in `docs/.more/universal-communication/rust/`

## 📚 Documentation

Complete documentation available in:
- `docs/.more/universal-communication/rust/README.md` - Overview
- `docs/.more/universal-communication/rust/01-core-types-and-traits.md` - Fundamentals
- `docs/.more/universal-communication/rust/02-service-registry.md` - Registry
- `docs/.more/universal-communication/rust/03-protocol-abstraction.md` - Protocols
- `docs/.more/universal-communication/rust/04-module-api-layer.md` - Module API
- `docs/.more/universal-communication/rust/05-echo-example.md` - This example
- `docs/.more/universal-communication/rust/ENHANCED_GRPC_EXAMPLES_COMPLETE.md` - gRPC enhancements

## 🔥 Key Features

### Type-Safe Communication

**No runtime casts!** Everything is checked at compile time:

```rust
// Enums for type-safe protocol selection
enum Protocol {
    Direct,    // In-process
    Grpc,      // Cross-process
    Auto,      // Framework decides
}

// Enums for type-safe service handlers
enum ServiceHandler {
    Echo(Arc<dyn EchoService>),
    // Add more services here
}
```

### Zero-Cost Direct Communication

When modules are in the same process:

```rust
let response = gateway.call(request).await?;
// This compiles to a direct function call!
// No serialization, no network, no overhead
```

### Automatic Protocol Selection

```rust
// Framework chooses best protocol:
// - Direct if in same process
// - gRPC if remote
let gateway = factory.new_service_gateway(
    &module_id,
    &service_id,
    &Protocol::Auto,  // Magic!
).await?;
```

### Service Discovery

```rust
// Find services without hardcoding addresses
let apis = registry_client.discover(&module_id).await?;
let address = apis[0].address;
// Connect to discovered service
```

## 🔧 Development

### Running Tests

```bash
cargo test --workspace
```

### Building Docs

```bash
cargo doc --workspace --open
```

### Linting

```bash
cargo clippy --workspace
```

## 🎯 Comparison with Go Implementation

This Rust implementation maintains **architectural alignment** with the Go version:

| Feature | Go | Rust | Notes |
|---------|------|------|-------|
| Module Framework | ✅ | ✅ | Same architecture |
| Service Registry | ✅ | ✅ | API parity |
| Direct Protocol | ✅ | ✅ | Zero-cost in both |
| gRPC Protocol | ✅ | ✅ | Same protobuf |
| Auto Protocol | ✅ | ✅ | Same logic |
| Type Safety | Interface{} | Enums | Rust more strict! |

See `docs/.more/universal-communication/rust/go-rust-comparison.md` for detailed comparison.

## 🐛 Troubleshooting

### "protoc not found"

Install Protocol Buffers compiler (see Prerequisites).

### "Service registry not found"

Make sure `hsu-registry` is running before starting server/client.

### "Module not found in registry"

Server must publish before client can discover. Start server first!

### Port already in use

Use custom port: `--port 9090`

## 📖 Related Examples

- `hsu-example1-go/` - Same examples in Go
- `hsu-example2-go/` - More complex scenarios
- `hsu-example3-*` - Multi-service examples

## 💡 Tips

1. **Start Simple**: Run direct CLI first to understand basics
2. **Read Logs**: All examples have detailed logging
3. **Experiment**: Try different command-line options
4. **Compare**: Look at Go implementation side-by-side
5. **Ask Questions**: Code is heavily documented!

## 🎉 Success!

If all three examples work, you have:
✅ Complete HSU framework running  
✅ Service registry operational  
✅ gRPC communication working  
✅ Service discovery functional  

**You're ready to build real microservices! 🚀**

---

## 📞 Support

Questions? Check:
1. Documentation in `docs/`
2. Code comments (extensive!)
3. Go examples (same architecture)
4. Protocol buffer definitions

---

*HSU Microservice Framework - Learning by doing! 🦀*
