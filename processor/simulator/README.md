# Inference Engine Simulator

Simulates the Inference Engine for development without Raspberry Pi + Hailo-8L hardware.

## Purpose

Allows development and testing of:
- Main Runtime (Rust) violation processing
- UDS communication
- Webhook/notification systems
- iOS app integration

## Setup

1. Add your test images:

```
simulator/
└── images/
    ├── normal.jpg      # Frame with no violations (all PPE worn)
    └── violation.jpg   # Frame with violations (missing PPE)
```

2. Start the Main Runtime first (it creates the UDS socket):

```bash
cd processor
cargo run
```

3. Run the simulator:

```bash
cd processor
python -m simulator.main
```

## Usage

```
============================================================
  INFERENCE ENGINE SIMULATOR
============================================================

  Status: NORMAL
  Frame: 000042 | FPS: 1.0 | Persons: 3
  UDS: connected | Sent: 42 | Failed: 0

  Controls:
    [v] Toggle violation mode
    [q] Quit

  Current violations (0): (none)

============================================================
```

### Controls

| Key | Action |
|-----|--------|
| `v` | Toggle between normal and violation mode |
| `q` | Quit the simulator |

### Command Line Options

```bash
python -m simulator.main --fps 2 --camera cam_test
```

| Option | Default | Description |
|--------|---------|-------------|
| `--fps` | `1.0` | Frames per second to send |
| `--camera` | `cam_sim` | Camera ID in evidence |

## Simulated Data

### 3 Static Persons

| Person | Violation (when enabled) |
|--------|--------------------------|
| 001 | `missing_gloves` |
| 002 | `missing_hardhat` |
| 003 | `missing_safetyvest` |

### Evidence Format

Same format as the real Inference Engine:

```json
{
  "camera_id": "cam_sim",
  "frame_id": "000001",
  "timestamp": 1704672345123,
  "person": [
    {
      "id": "001",
      "bbox": [100, 100, 250, 400],
      "confidence": 0.95,
      "part": [
        {"label": "head", "bbox": [...], "confidence": 0.92},
        {"label": "hand", "bbox": [...], "confidence": 0.88}
      ],
      "equipment": [
        {"label": "hardhat", "bbox": [...], "confidence": 0.91},
        {"label": "safetyvest", "bbox": [...], "confidence": 0.93}
      ],
      "violation": ["missing_gloves"]
    }
  ]
}
```

## Output

- **UDS Messages**: Sent to `/tmp/gidence-scm_uds.sock`
- **Evidence Images**: Saved to `processor/evidence/{camera_id}_{frame_id}_{timestamp}.jpg`

## Development Workflow

1. Start Main Runtime: `cargo run`
2. Start Simulator: `python -m simulator.main`
3. Press `v` to trigger violations
4. Observe Main Runtime processing
5. Test webhook/notification logic
