# Troubleshooting Guide

*Quick reference for common problems and solutions*

## 🔍 Quick Problem Lookup

### Connection Issues
- `curl: (56) Recv failure: Connection reset by peer` → [localhost vs IP Resolution](#connection-reset-by-peer-localhost-vs-ip)
- `"endpoint offline"` errors → [ngrok Tunnel Offline](#ngrok-tunnel-offline)
- `Failed to bind to port 8080` → [Port Already in Use](#port-already-in-use)

### Rust Compilation Issues  
- `cargo: command not found` → [Rust Not Installed](#rust-not-installed)
- `use of undeclared crate` → [Missing Dependencies](#missing-dependencies)
- `edition = "2024"` errors → [Invalid Rust Edition](#invalid-rust-edition)

### Webhook Issues
- Repeated POST requests → [Pub/Sub Retry Loops](#pubsub-retry-loops)
- Handler not called → [Webhook Handler Not Called](#webhook-handler-not-called)

---

## Day 1 Issues

### ngrok Tunnel Offline
**Error Messages**: `"endpoint offline"`, `failed to connect to ngrok tunnel`  
**Symptoms**: Pub/Sub can't deliver messages to webhook  
**Quick Fix**: 
```bash
# Restart ngrok
ngrok http 8080

# Update subscription with new URL
gcloud pubsub subscriptions update gmail-webhook-sub \
  --push-endpoint=https://NEW_NGROK_URL.ngrok-free.app/gmail-event
```

### Pub/Sub Retry Loops
**Error Messages**: Continuous POST requests in logs  
**Symptoms**: Same message delivered repeatedly  
**Root Cause**: Webhook not returning HTTP 200 OK  
**Solution**: **Normal behavior** - webhook must return 200 to stop retries  
**Verification**: Check webhook returns `StatusCode::OK` or equivalent

---

## Day 2 Issues

### Connection Reset by Peer (localhost vs IP)
**Error Messages**: 
- `curl: (56) Recv failure: Connection reset by peer`
- `Connection refused` 
- `No connection could be made`

**Symptoms**: Server starts successfully but curl requests fail  
**Root Cause**: Network DNS resolution interfering with localhost  
**Quick Fix**:
```bash
# Change server binding from localhost to IP
TcpListener::bind("127.0.0.1:8080")  # ✅ Use this

# Test with IP instead of localhost  
curl http://127.0.0.1:8080/gmail-event  # ✅ Use this
curl http://localhost:8080/gmail-event   # ❌ Avoid this
```
**Prevention**: Always use direct IP addresses in development

### Invalid Rust Edition
**Error Messages**: 
- `error: edition "2024" is not valid`
- Compilation errors with modern syntax

**Symptoms**: Unexpected compilation failures  
**Root Cause**: `Cargo.toml` has non-existent edition  
**Quick Fix**: Edit `Cargo.toml`:
```toml
[package]
edition = "2021"  # ✅ Latest stable
```
**Follow-up**: `cargo clean && cargo build`

### Rust Not Installed  
**Error Messages**: 
- `cargo: command not found`
- `rustc: command not found`

**Symptoms**: No Rust commands available  
**Root Cause**: Rust not installed system-wide  
**Quick Fix**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart terminal, then verify
cargo --version
rustc --version
```
**Important**: Don't install in Python virtual environment

### Port Already in Use
**Error Messages**: 
- `Failed to bind to port 8080`
- `Address already in use`

**Symptoms**: Server won't start  
**Diagnosis**: `lsof -i :8080` to see what's using the port  
**Quick Fix**:
```bash
# Option 1: Kill the process
kill <PID>

# Option 2: Use different port
TcpListener::bind("127.0.0.1:3000")  # Change in code
```

### Missing Dependencies
**Error Messages**: 
- `use of undeclared crate 'axum'`
- `no such crate: serde`

**Symptoms**: Compilation fails on import statements  
**Root Cause**: Dependencies not listed in `Cargo.toml`  
**Quick Fix**: Add to `[dependencies]` section:
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
base64 = "0.22"
```

### Cloudflare WARP Interference (False Alarm)
**Error Messages**: Network connectivity issues  
**Initial Suspicion**: VPN interfering with localhost  
**Actual Cause**: localhost vs 127.0.0.1 binding issue  
**Investigation Steps**:
1. Temporarily disable WARP
2. Test with `127.0.0.1` instead of `localhost`
3. Check split tunneling if WARP needed
**Resolution**: Use `127.0.0.1` - WARP was not the problem

### Webhook Handler Not Called
**Error Messages**: No `println!` output despite server running  
**Symptoms**: 
- Server starts successfully
- No handler logs appear on requests
- May see connection errors

**Common Causes**:
- Wrong HTTP method (GET vs POST)
- Wrong URL path (`/gmail-event` typo)
- Server bound to wrong interface

**Debugging Checklist**:
```bash
# ✅ Verify POST method
curl -X POST http://127.0.0.1:8080/gmail-event

# ✅ Check exact path
curl -v http://127.0.0.1:8080/gmail-event

# ✅ Add test route for debugging
.route("/", get(|| async { "Server alive" }))
```

---

## 🛠️ General Debugging Strategies

### Rust Compilation Issues
- **Fast checking**: `cargo check` (faster than full build)
- **Clean build**: `cargo clean` if seeing weird caching issues  
- **Read errors carefully**: Rust error messages are very helpful
- **Incremental testing**: Test simple code before complex logic

### Network Debugging
- **Verbose output**: `curl -v` for detailed HTTP debugging
- **Port monitoring**: `lsof -i :PORT` to see what's listening
- **Simple first**: Test basic endpoints before complex webhooks
- **Process tracking**: `ps aux | grep process-name` to find running servers

### JSON and API Issues
- **Validate JSON**: Use online validators for payload format
- **Debug printing**: `println!("{:#?}", payload)` to inspect structures
- **Known good data**: Test with simple, verified JSON first
- **Method verification**: Ensure POST vs GET requests match expectations

### Development Workflow
- **IP addresses**: Use `127.0.0.1` instead of `localhost` for reliability
- **Local first**: Test locally before external integration
- **Keep infrastructure**: Don't tear down working Day 1 components
- **Systematic isolation**: Test one component at a time when debugging

---

## 📚 Reference Commands

### Common Debugging Commands
```bash
# Check what's using a port
lsof -i :8080

# Kill process by PID  
kill <PID>

# Verbose curl request
curl -v -X POST http://127.0.0.1:8080/gmail-event

# Check Rust installation
cargo --version && rustc --version

# Clean Rust build cache
cargo clean

# Fast Rust syntax check
cargo check
```

### Project-Specific Commands
```bash
# Restart ngrok tunnel
ngrok http 8080

# Update Pub/Sub subscription
gcloud pubsub subscriptions update gmail-webhook-sub \
  --push-endpoint=https://NEW_URL.ngrok-free.app/gmail-event

# Test Pub/Sub message
gcloud pubsub topics publish gmail-events --message="test"

# Start Rust webhook server
cd rust-ingestor && cargo run
```