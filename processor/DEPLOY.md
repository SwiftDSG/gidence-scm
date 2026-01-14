# Deployment Guide - Cross-Compiling for Raspberry Pi 5

This guide explains how to compile the Rust binary on macOS and deploy it to Raspberry Pi 5 without installing Rust on the target machine.

---

## Quick Start (Option 1: Cross-Compile from macOS)

### 1. Install Cross-Compilation Tools

```bash
# Install the ARM64 target for Rust
rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation toolchain (via Homebrew)
brew install filosottile/musl-cross/musl-cross
```

### 2. Build the Binary

```bash
cd processor

# Build for Raspberry Pi 5 (ARM64)
cargo build --release --target aarch64-unknown-linux-gnu

# The binary will be at:
# target/aarch64-unknown-linux-gnu/release/processor
```

### 3. Deploy to Raspberry Pi

```bash
# Copy binary to Raspberry Pi
scp target/aarch64-unknown-linux-gnu/release/processor pi@raspberrypi.local:~/gidence-scm/processor/

# SSH into Raspberry Pi and make it executable
ssh pi@raspberrypi.local
chmod +x ~/gidence-scm/processor/processor
```

---

## Option 2: Use Cross (Easier, Docker-based)

If native cross-compilation has issues, use the `cross` tool:

### 1. Install Cross

```bash
cargo install cross
```

### 2. Build with Cross

```bash
cd processor

# Cross automatically handles the toolchain
cross build --release --target aarch64-unknown-linux-gnu
```

### 3. Deploy (same as above)

```bash
scp target/aarch64-unknown-linux-gnu/release/processor pi@raspberrypi.local:~/gidence-scm/processor/
```

---

## Option 3: Build Directly on Raspberry Pi

If cross-compilation is problematic, build on the Pi itself:

```bash
# On Raspberry Pi, install Rust (one-time setup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the binary
cd ~/gidence-scm/processor
cargo build --release

# Binary at: target/release/processor
```

---

## What Needs to Be on the Raspberry Pi?

### ✅ **Pre-compiled Rust Binary**
- `processor` executable (cross-compiled or built on Pi)
- No Rust installation needed if cross-compiled!

### ✅ **Python Environment**
- Python 3.8+ with dependencies
- Virtual environment (created by `setup.sh`)
- The binary calls `bash -c "source setup.sh && python3 inference/main.py"`

### ✅ **System Dependencies**
- `bash` (for running setup.sh)
- GStreamer libraries (for video pipeline)
- Hailo runtime libraries (for inference)

### ✅ **Project Files**
```
gidence-scm/processor/
├── processor              # The Rust binary (cross-compiled)
├── processor.json         # Configuration (auto-generated on first run)
├── setup.sh              # Python venv activation script
├── inference/
│   ├── main.py           # Python inference entry point
│   ├── pipeline.py
│   ├── association.py
│   ├── udp.py
│   └── model/
│       └── yolov8n.hef   # AI model file
└── ...
```

---

## Deployment Workflow

### Step 1: Build on macOS
```bash
cd processor
cargo build --release --target aarch64-unknown-linux-gnu
```

### Step 2: Create Deployment Package
```bash
# From project root
tar -czf scm-deploy.tar.gz \
    processor/target/aarch64-unknown-linux-gnu/release/processor \
    processor/inference/ \
    processor/setup.sh \
    processor/pyproject.toml \
    processor/install.sh
```

### Step 3: Transfer to Raspberry Pi
```bash
scp scm-deploy.tar.gz pi@raspberrypi.local:~/
ssh pi@raspberrypi.local

# Extract
tar -xzf scm-deploy.tar.gz
cd processor

# Setup Python environment
./install.sh

# Run the processor
./processor
```

### Step 4: Run at Boot (Optional)
Create a systemd service:

```bash
sudo nano /etc/systemd/system/scm-processor.service
```

```ini
[Unit]
Description=SCM Processor
After=network.target

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi/gidence-scm/processor
ExecStart=/home/pi/gidence-scm/processor/processor
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable scm-processor
sudo systemctl start scm-processor
sudo systemctl status scm-processor
```

---

## Troubleshooting

### Issue: "cannot execute binary file"
**Cause:** Wrong architecture (built for macOS instead of ARM64)
**Fix:** Make sure you used `--target aarch64-unknown-linux-gnu`

### Issue: Cross-compilation linker errors
**Cause:** Missing cross-compilation toolchain
**Fix:** Use `cross` instead of `cargo build`

### Issue: Python script fails
**Cause:** Virtual environment not activated
**Fix:** Ensure `setup.sh` exists and works: `source setup.sh && python3 --version`

### Issue: "libhailo.so not found"
**Cause:** Hailo runtime not installed
**Fix:** Install Hailo runtime on Raspberry Pi (see processor/README.md)

---

## Binary Size Optimization (Optional)

To reduce binary size:

```toml
# Add to Cargo.toml
[profile.release]
strip = true        # Strip symbols (smaller binary)
lto = true         # Link-time optimization
codegen-units = 1  # Better optimization
opt-level = "z"    # Optimize for size
```

Then rebuild:
```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

---

## Summary

**Recommended Approach:**
1. Install `cross`: `cargo install cross`
2. Build: `cross build --release --target aarch64-unknown-linux-gnu`
3. Deploy binary + Python files to Raspberry Pi
4. No Rust needed on target machine!

**What runs where:**
- **macOS:** Rust compilation only
- **Raspberry Pi:** Pre-compiled binary + Python inference engine + Hailo runtime
