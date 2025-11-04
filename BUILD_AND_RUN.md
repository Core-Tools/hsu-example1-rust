# Quick Build and Run Guide 🚀

## TL;DR - 3 Commands to See It Work!

```bash
# 1. Build everything
cargo build --workspace --release

# 2. Run direct example (all in one!)
cargo run --release --bin echo-direct-cli

# Done! You just saw HSU framework in action! 🎉
```

---

## Full Demo with Service Registry

### Terminal 1: Service Registry

```bash
cd ..\hsu-core\rust
cargo run --release --bin hsu-registry
```

**Expected output:**
```
✅ Starting registry server: Tcp { port: 8080 }
```

Keep this running!

---

### Terminal 2: gRPC Server

```bash
cd hsu-example1-rust
cargo run --release --bin echo-grpc-srv
```

**Expected output:**
```
🚀 Echo gRPC Server - Starting with FULL HSU framework!
✅ Created EchoModule: echo
✅ Created ModuleRuntime
✅ ModuleRuntime started
✅ gRPC server listening on: 0.0.0.0:50051
✅ Published to service registry at: http://localhost:8080
🎉 Server is ready!
```

Keep this running!

---

### Terminal 3: gRPC Client

```bash
cd hsu-example1-rust
cargo run --release --bin echo-grpc-cli
```

**Expected output:**
```
🚀 Echo gRPC Client - Starting with FULL HSU framework!
🔍 Discovering service from registry: http://localhost:8080
✅ Discovered service!
   Module: echo
   Service: echo-service
   Protocol: Grpc
   Address: localhost:50051
📡 Connecting to: http://localhost:50051
✅ Connected!
📤 Sending message: "Hello from Rust client! 🦀"
📥 Response: "Hello from Rust client! 🦀"
🎉 Demo complete!
```

**SUCCESS! All components working together! 🎊**

---

## What Just Happened?

1. **Service Registry** started on port 8080
2. **gRPC Server** started and published itself to registry
3. **gRPC Client** discovered server from registry and called it

This is the **complete HSU framework** in action!

---

## Quick Options

### Custom Port

```bash
cargo run --release --bin echo-grpc-srv -- --port 9090
cargo run --release --bin echo-grpc-cli -- --direct-address localhost:9090
```

### Custom Message

```bash
cargo run --release --bin echo-grpc-cli -- --message "Hello from Rust!"
```

### Skip Service Registry (Direct Connection)

```bash
# Just start server (Terminal 1)
cargo run --release --bin echo-grpc-srv

# Connect directly (Terminal 2)
cargo run --release --bin echo-grpc-cli -- --direct-address localhost:50051
```

---

## Troubleshooting

### "protoc: command not found"

**Solution:** Install Protocol Buffers compiler

```bash
# Windows (via chocolatey)
choco install protoc

# macOS
brew install protobuf

# Ubuntu/Debian
sudo apt install protobuf-compiler
```

---

### Port already in use

**Solution:** Use a different port

```bash
cargo run --release --bin echo-grpc-srv -- --port 9090
```

---

### "Module not found in registry"

**Solution:** Make sure server started BEFORE client!

1. Start registry first
2. Start server (it will publish)
3. Start client (it will discover)

---

### Binary not found

**Solution:** Build first!

```bash
cargo build --workspace --release
```

---

## Performance Test

Want to see how fast it is? Run the direct example:

```bash
cargo run --release --bin echo-direct-cli
```

**Direct communication overhead: ~6 CPU cycles!** ⚡

---

## Next Steps

1. ✅ **Read the code** - Start with `crates/echo-domain/src/service.rs`
2. ✅ **Read the docs** - Check `../docs/.more/universal-communication/rust/`
3. ✅ **Experiment** - Modify the code, add features
4. ✅ **Compare** - Look at the Go implementation side-by-side
5. ✅ **Build** - Create your own services!

---

## All Examples

| Example | Command | What It Shows |
|---------|---------|---------------|
| **Direct CLI** | `cargo run --release --bin echo-direct-cli` | In-process communication, zero-cost |
| **gRPC Server** | `cargo run --release --bin echo-grpc-srv` | Full framework, registry publishing |
| **gRPC Client** | `cargo run --release --bin echo-grpc-cli` | Service discovery, remote calls |

---

**You now have a working microservice framework! 🚀🦀🎉**

Happy coding!

