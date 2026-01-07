#!/usr/bin/env bash
#===============================================================================
# Minimal Hailo Detection Simple - Installation Script
#===============================================================================
# Minimal installation script for the detection_simple application only
#
# Features:
#   - Installs only dependencies needed for detection_simple
#   - Creates virtual environment
#   - Installs hailo-apps package in minimal mode
#
# Usage:
#   sudo ./install.sh
#
#===============================================================================

set -euo pipefail

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly VENV_NAME="scm"
readonly VENV_PATH="${SCRIPT_DIR}/${VENV_NAME}"

# Terminal colors
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

#===============================================================================
# LOGGING FUNCTIONS
#===============================================================================

log() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

#===============================================================================
# UTILITY FUNCTIONS
#===============================================================================

check_root() {
    if [[ $EUID -eq 0 ]]; then
        log_error "This script should NOT be run as root"
        log_error "Please run as regular user: ./install.sh"
        exit 1
    fi
}

detect_architecture() {
    local arch
    arch=$(uname -m)
    case "$arch" in
        aarch64|arm64)
            echo "aarch64"
            ;;
        x86_64|amd64)
            echo "x86_64"
            ;;
        *)
            log_error "Unsupported architecture: $arch"
            log_error "Supported: aarch64, x86_64"
            exit 1
            ;;
    esac
}

install_system_dependencies() {
    log "Installing system dependencies..."
    
    # Update package list
    sudo apt-get update
    
    # Install basic dependencies
    sudo apt-get install -y \
        python3 \
        python3-pip \
        python3-venv \
        python3-dev \
        build-essential \
        cmake \
        pkg-config \
        libgstreamer1.0-dev \
        libgstreamer-plugins-base1.0-dev \
        libgstreamer-plugins-bad1.0-dev \
        gstreamer1.0-plugins-base \
        gstreamer1.0-plugins-good \
        gstreamer1.0-plugins-bad \
        gstreamer1.0-plugins-ugly \
        gstreamer1.0-libav \
        gstreamer1.0-tools \
        gstreamer1.0-x \
        gstreamer1.0-alsa \
        gstreamer1.0-gl \
        gstreamer1.0-gtk3 \
        gstreamer1.0-qt5 \
        gstreamer1.0-pulseaudio \
        libgirepository1.0-dev \
        python3-gi \
        python3-gi-cairo \
        gir1.2-gstreamer-1.0 \
        gir1.2-gst-plugins-base-1.0
        
    log_success "System dependencies installed"
}

create_virtual_environment() {
    log "Creating virtual environment at ${VENV_PATH}..."
    
    if [[ -d "${VENV_PATH}" ]]; then
        log_warning "Virtual environment already exists. Removing..."
        rm -rf "${VENV_PATH}"
    fi
    
    # Create venv with system site packages for GStreamer bindings
    python3 -m venv --system-site-packages "${VENV_PATH}"
    
    # Activate virtual environment
    source "${VENV_PATH}/bin/activate"
    
    # Upgrade pip
    python -m pip install --upgrade pip
    
    log_success "Virtual environment created and activated"
}

install_python_dependencies() {
    log "Installing Python dependencies..."
    
    pip install -e .
    
    log_success "Python dependencies installed"
}

create_environment_script() {
    log "Creating environment activation script..."
    
    cat > "${SCRIPT_DIR}/setup.sh" << 'EOF'
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
EOF
    
    chmod +x "${SCRIPT_DIR}/setup.sh"
    
    log_success "Environment script created: setup.sh"
}

main() {
    local arch
    
    log "Starting Optense — SCM Installation..."
    log "Script directory: ${SCRIPT_DIR}"
    
    # Pre-flight checks
    check_root
    arch=$(detect_architecture)
    log "Detected architecture: ${arch}"
    
    # Installation steps
    install_system_dependencies
    create_virtual_environment
    install_python_dependencies
    create_environment_script
    
    log_success "Installation completed successfully!"
    echo ""
    log "To use Optense — SCM:"
    log "  1. Activate the environment: source ./setup.sh"
    log "  2. Run detection: run"
    echo ""
    log_warning "Note: You'll need to install HailoRT and have proper HEF model files"
    log_warning "This minimal installation only includes the application framework"
}

# Run main function
main "$@"
