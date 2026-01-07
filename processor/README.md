# Hailo Apps Minimal - Detection Simple

This is a minimal version of the Hailo Apps repository containing only the `detection_simple` application and its core dependencies.

## Features

- **detection_simple**: Lightweight object detection pipeline for Hailo AI accelerators
- Minimal dependency footprint
- Easy installation on Raspberry Pi and x86_64 systems

## Quick Start

### Installation
```bash
./install.sh
```

### Usage
```bash
# Activate the environment
source ./setup_env.sh

# Run detection_simple
hailo-detect-simple
```

## Requirements

- **Hardware**: Hailo AI accelerator (Hailo-8, Hailo-8L, or Hailo-10H)
- **OS**: Ubuntu 20.04+ or Raspberry Pi OS
- **Python**: 3.8+
- **HailoRT**: Must be installed separately (contact Hailo for installation)

## Directory Structure

```
processor/
├── hailo_apps/                    # Main Python package
│   ├── python/
│   │   ├── core/                  # Core utilities
│   │   │   ├── common/            # Common utilities (logging, config, etc.)
│   │   │   └── gstreamer/         # GStreamer integration
│   │   └── pipeline_apps/
│   │       └── detection_simple/  # Detection simple application
│   └── config/                    # Configuration files
├── install.sh                     # Installation script
├── setup_env.sh                   # Environment activation (created by install)
├── pyproject.toml                 # Python package configuration
└── README.md                      # This file
```

## Notes

This minimal version:
- Contains only the `detection_simple` application
- Removes all other applications (pose estimation, segmentation, GenAI, etc.)
- Keeps only essential core utilities
- Maintains full functionality of the detection_simple pipeline

## Original Repository

This is derived from the full [Hailo Apps](https://github.com/hailo-ai/hailo-apps) repository.