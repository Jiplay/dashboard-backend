# Verification Guide

## Quick Test

Run this command to verify everything is working on your Mac:

```bash
./build.sh
```

If you don't have cargo installed or want to test step by step:

## Step-by-Step Verification

### 1. Check Compilation

```bash
cargo check
```

Expected output: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in X.XXs`

### 2. Check with All Features

```bash
cargo check --all-features
```

Expected output: `Finished 'dev' profile [unoptimized + debuginfo] target(s) in X.XXs`

### 3. Run Tests

```bash
cargo test
```

Expected output: All tests should pass

### 4. Run Library Example

```bash
cargo run --example library_usage
```

Expected output: JSON output showing your system metrics

Example output:
```json
{
  "system": {
    "os_name": "macOS",
    "os_version": "14.6",
    "hostname": "YourMac",
    ...
  },
  "cpu": {
    "usage_percent": 15.3,
    "physical_cores": 8,
    "logical_cores": 8,
    ...
  },
  "memory": {
    "total_bytes": 17179869184,
    "usage_percent": 65.2,
    ...
  }
}
```

### 5. Run Server (Optional)

```bash
cargo run --features full --bin dashboard-server
```

Expected output:
```
Starting dashboard server on 127.0.0.1:3000
Server listening on 127.0.0.1:3000
```

Then in another terminal:
```bash
curl http://localhost:3000/api/v1/health
# Should return: {"status":"ok","service":"dashboard-backend"}

curl http://localhost:3000/api/v1/cpu | jq
# Should return CPU metrics as JSON
```

## Common Issues

### Issue: `cargo: command not found`

**Solution**: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: Compilation is slow

**Solution**: This is normal for the first build. Rust compiles all dependencies. Subsequent builds will be much faster.

### Issue: `error: linker 'cc' not found`

**Solution**: Install Xcode Command Line Tools
```bash
xcode-select --install
```

### Issue: Port 3000 already in use

**Solution**: Either:
1. Stop the process using port 3000
2. Change the port in `config/default.toml`:
   ```toml
   [server]
   port = 8080
   ```

## What Should Work Now

✅ Project compiles without errors
✅ All tests pass
✅ Library usage example runs
✅ Custom collector example runs
✅ Standalone server starts
✅ API endpoints respond

## Files Fixed

The following files were updated to fix macOS compatibility:

1. `src/collectors/cpu.rs` - Fixed `global_cpu_info()` issue
2. `src/collectors/memory.rs` - Fixed System clone issue
3. `src/collectors/disk.rs` - Fixed file_system type mismatch
4. `src/collectors/network.rs` - Simplified structure
5. `src/collectors/system.rs` - Fixed System clone issue

## Next Steps

Once everything verifies successfully:

1. **Integrate into your API** - See `INTEGRATION_GUIDE.md`
2. **Customize configuration** - Edit `config/default.toml`
3. **Add custom collectors** - Implement `MetricCollector` trait
4. **Deploy** - Build release version: `cargo build --release --all-features`

## Getting Help

If you encounter any issues:

1. Check `FIXES.md` for details on what was changed
2. Review `README.md` for full documentation
3. Look at `examples/` for usage patterns
4. Check the error message carefully - Rust errors are usually very helpful

## Performance Notes

- First build: ~2-5 minutes (compiling all dependencies)
- Subsequent builds: ~5-30 seconds (only changed files)
- Release build: ~3-7 minutes (optimized binary)
- Runtime: Very fast, metrics collected in milliseconds

## Success Criteria

You'll know everything is working when:

```bash
# This command succeeds
cargo run --example library_usage

# And you see output like:
Dashboard Backend - Library Usage Example

=== Collecting All Metrics ===
{
  "system": { ... },
  "cpu": { ... },
  "memory": { ... },
  "disk": { ... },
  "network": { ... }
}
```

🎉 If you see this output, congratulations! The system is fully functional on your Mac.
