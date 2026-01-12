#!/usr/bin/env bash

# Hailo Apps Minimal Environment Setup

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VENV_PATH="${SCRIPT_DIR}/scm"

if [[ -f "${VENV_PATH}/bin/activate" ]]; then
    source "${VENV_PATH}/bin/activate"
    echo "Optense — SCM environment activated"
    echo "Available command: run"
else
    echo "Error: Virtual environment not found at ${VENV_PATH}"
    echo "Please run ./install.sh first"
    exit 1
fi
