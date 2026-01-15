# SCM Processor

Real-time PPE compliance monitoring system for Raspberry Pi 5 + Hailo-8.

## Overview

The processor consists of two components:

| Component | Language | Purpose |
|-----------|----------|---------|
| **Inference Engine** | Python | Detection pipeline using GStreamer + Hailo SDK |
| **Main Runtime** | Rust | Process management, violation handling, webhooks |

## Architecture

```
Camera Streams (RTSP/File)
    ↓
Inference Engine (Python)
    ├─ GStreamer pipeline
    ├─ YOLOv8n detection (Hailo-8)
    ├─ Person tracking
    ├─ Body part & PPE assignment
    └─ Compliance checking
    ↓
UDS (Unix Domain Socket)
    ↓
Main Runtime (Rust)
    ├─ Receive evidence JSON
    ├─ Process violations (TODO)
    └─ Send webhooks (TODO)
```

## Directory Structure

```
processor/
├── src/                          # Main Runtime (Rust)
│   ├── main.rs                   # Entry point - spawns Inference Engine, UDS listener
│   └── models/
│       ├── processor.rs          # Processor config (cameras, webhooks)
│       └── evidence.rs           # Evidence data structures
├── inference/                    # Inference Engine (Python)
│   ├── main.py                   # Entry point - GStreamer callback
│   ├── pipeline.py               # GStreamer pipeline construction
│   ├── association.py            # Compliance checking logic
│   ├── uds.py                    # Unix Domain Socket sender
│   ├── model/                    # Deployment models (.hef)
│   │   └── yolov8n.hef           # Quantized YOLOv8n for Hailo
│   ├── so/                       # Post-processing libraries
│   │   └── yolov8.so             # Hailo post-process
│   └── core/                     # Core utilities
│       ├── common/               # Logging, buffer utils
│       └── gstreamer/            # GStreamer helpers
├── Cargo.toml                    # Rust dependencies
├── pyproject.toml                # Python dependencies
├── processor.json                # Runtime configuration (auto-generated)
├── install.sh                    # Python environment setup
├── setup.sh                      # Activate venv
├── DEPLOY.md                     # Deployment guide
└── README.md                     # This file
```

## Quick Start

### 1. Install Dependencies

```bash
# Install Python environment
./install.sh

# Build Rust binary (on target or cross-compile)
cargo build --release
```

### 2. Configure

Edit `processor.json` (auto-generated on first run):

```json
{
  "id": "auto-generated-uuid",
  "name": "SCM Processor",
  "model": "yolov8n.hef",
  "cameras": [
    {
      "id": "cam_1",
      "address": {
        "ip": "192.168.1.100",
        "port": 554,
        "path": "/stream1"
      },
      "auth": {
        "username": "admin",
        "password": "password"
      },
      "name": "Entrance Camera"
    }
  ],
  "webhooks": [
    {
      "domain": "api.example.com",
      "port": 443,
      "path": "/violations",
      "secure": true
    }
  ]
}
```

### 3. Run

```bash
# Run the Main Runtime (spawns Inference Engine automatically)
./target/release/processor

# Or run Inference Engine directly for testing
source setup.sh
python -m inference.main
```

## Evidence Format

The Inference Engine sends JSON evidence for every frame via UDS:

```json
{
  "camera_id": "cam_1",
  "frame_id": "000123",
  "timestamp": 1704672345123,
  "person": [
    {
      "id": "001",
      "bbox": [100, 150, 200, 350],
      "confidence": 0.95,
      "part": [
        {"label": "hand", "bbox": [120, 280, 160, 320], "confidence": 0.87}
      ],
      "equipment": [
        {"label": "hardhat", "bbox": [108, 150, 192, 205], "confidence": 0.89}
      ],
      "violation": ["missing_gloves"]
    }
  ]
}
```

## Violation Types

| Violation | Condition |
|-----------|-----------|
| `missing_hardhat` | Head visible, no hardhat |
| `missing_gloves` | Hand visible, no gloves |
| `missing_shoes` | Foot visible, no shoes |
| `missing_facemask` | Face visible, no facemask |
| `missing_earmuffs` | Ear visible, no earmuffs |
| `missing_safetyvest` | Person detected, no safetyvest |
| `improperly_worn_gloves` | Hand AND gloves both visible |
| `improperly_worn_shoes` | Foot AND shoes both visible |
| `improperly_worn_facemask` | Face AND facemask both visible |
| `improperly_worn_earmuffs` | Ear AND earmuffs both visible |

## Requirements

### Hardware
- Raspberry Pi 5 (recommended) or x86_64
- Hailo-8 AI Accelerator (26 TOPS)

### Software
- HailoRT (install separately from Hailo)
- GStreamer 1.0
- Python 3.8+
- Rust 1.70+

## Development

### Run Inference Engine Only

```bash
source setup.sh
python -m inference.main
```

### Build Main Runtime

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Cross-compile for Raspberry Pi (from macOS/Linux)
cargo build --release --target aarch64-unknown-linux-gnu
```

### Test UDS Communication

```bash
# Listen for evidence (in another terminal)
nc -lU /tmp/gidence-scm_uds.sock
```

## Deployment

See [DEPLOY.md](DEPLOY.md) for full deployment instructions including:
- Cross-compilation setup
- Raspberry Pi configuration
- Systemd service setup
- Troubleshooting guide
