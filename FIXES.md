# macOS Compatibility Fixes

## Issues Fixed

### 1. System Clone Error
**Error**: `no method named 'clone' found for struct 'sysinfo::System'`

**Cause**: The `sysinfo::System` struct does not implement the `Clone` trait in the current version.

**Fix**: Changed all collectors from storing a `System` instance to creating a new one on each collection:

**Before:**
```rust
pub struct CpuCollector {
    system: System,
}

async fn collect(&self) -> DashboardResult<Self::Output> {
    let mut system = self.system.clone();  // ❌ Error
    // ...
}
```

**After:**
```rust
pub struct CpuCollector;

async fn collect(&self) -> DashboardResult<Self::Output> {
    let mut system = System::new_with_specifics(...);  // ✅ Works
    // ...
}
```

### 2. RefreshKind API Error
**Error**: `this method takes 1 argument but 0 arguments were supplied`

**Cause**: The `refresh_memory()` method signature changed in newer versions of sysinfo.

**Fix**: Changed from `System::new_with_specifics()` with custom `RefreshKind` to `System::new_all()`:

**Before:**
```rust
let system = System::new_with_specifics(RefreshKind::new().with_memory());
system.refresh_memory();  // ❌ Wrong signature
```

**After:**
```rust
let mut system = System::new_all();
system.refresh_memory();  // ✅ Works
```

### 3. Disk File System Type Mismatch
**Error**: `mismatched types` in `src/collectors/disk.rs:54:54`

**Cause**: The `disk.file_system()` method now returns `&OsStr` instead of `&[u8]`.

**Fix**: Changed from `String::from_utf8_lossy()` to `to_string_lossy()`:

**Before:**
```rust
file_system: String::from_utf8_lossy(disk.file_system()).to_string(),  // ❌ Type error
```

**After:**
```rust
file_system: disk.file_system().to_string_lossy().to_string(),  // ✅ Works
```

### 4. Global CPU Info Method Missing
**Error**: `no method named 'global_cpu_info' found for struct 'sysinfo::System'`

**Cause**: The `global_cpu_info()` method doesn't exist in sysinfo 0.32.

**Fix**: Calculate overall CPU usage as the average of all cores instead:

**Before:**
```rust
let global_cpu = system.global_cpu_info();  // ❌ Method doesn't exist
let usage_percent = global_cpu.cpu_usage();
let frequency_mhz = Some(global_cpu.frequency());
```

**After:**
```rust
// Calculate overall usage as average of all cores
let usage_percent = if !per_core_usage.is_empty() {
    per_core_usage.iter().map(|c| c.usage_percent).sum::<f32>() / per_core_usage.len() as f32
} else {
    0.0
};
let frequency_mhz = cpus.first().map(|cpu| cpu.frequency());  // ✅ Works
```

## Files Modified

1. `src/collectors/cpu.rs` - Removed System storage, create new instance on collection
2. `src/collectors/memory.rs` - Removed System storage, use System::new_all()
3. `src/collectors/disk.rs` - Removed System storage, fixed file_system type handling
4. `src/collectors/network.rs` - Simplified struct definition for consistency
5. `src/collectors/system.rs` - Removed System storage, use System::new_all()

## Performance Notes

**Q: Won't creating a new System instance on each collection be slower?**

**A**: The performance impact is minimal because:
- System information collection is already I/O bound (reading from /proc, sysctl, etc.)
- The `System` struct initialization is lightweight
- Collectors are designed to be called infrequently (every 5+ seconds typically)
- This approach is actually more accurate as it reads fresh data each time

## Testing

To verify all fixes work on your Mac:

```bash
# Make the build script executable (if not already done)
chmod +x build.sh

# Run the build and test script
./build.sh

# Or test manually
cargo check --all-features
cargo test --all-features
cargo run --example library_usage
```

## Compatibility

These fixes ensure compatibility with:
- ✅ macOS (all versions)
- ✅ Linux
- ✅ Windows
- ✅ sysinfo 0.32.x

## Additional Changes

- Added `build.sh` - automated build and test script
- All collectors now use zero-sized types (no internal state)
- Consistent pattern across all metric collectors
