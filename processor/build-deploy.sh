#!/bin/bash
# Build and Deploy Script for SCM Processor
# Builds ARM64 binary on macOS and deploys to Raspberry Pi

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== SCM Processor Build & Deploy ===${NC}\n"

# Configuration
TARGET="aarch64-unknown-linux-gnu"
BINARY_NAME="processor"
PI_HOST="${PI_HOST:-pi@raspberrypi.local}"
PI_DIR="${PI_DIR:-~/gidence-scm/processor}"

# Check if cross is installed
if ! command -v cross &> /dev/null; then
    echo -e "${YELLOW}Warning: 'cross' not found. Install it with:${NC}"
    echo "  cargo install cross"
    echo -e "${YELLOW}Falling back to cargo (may not work on macOS -> ARM64)${NC}\n"
    BUILD_CMD="cargo"
else
    echo -e "${GREEN}Using 'cross' for cross-compilation${NC}\n"
    BUILD_CMD="cross"
fi

# Step 1: Build the binary
echo -e "${GREEN}[1/4] Building binary for ${TARGET}...${NC}"
$BUILD_CMD build --release --target $TARGET

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful!${NC}\n"
else
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi

# Step 2: Check binary size
BINARY_PATH="target/${TARGET}/release/${BINARY_NAME}"
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo -e "${GREEN}Binary size: ${BINARY_SIZE}${NC}\n"

# Step 3: Create deployment package
echo -e "${GREEN}[2/4] Creating deployment package...${NC}"
DEPLOY_DIR="deploy-package"
rm -rf $DEPLOY_DIR
mkdir -p $DEPLOY_DIR

# Copy binary
cp "$BINARY_PATH" "$DEPLOY_DIR/"
chmod +x "$DEPLOY_DIR/$BINARY_NAME"

# Copy Python files
cp -r inference "$DEPLOY_DIR/"
cp setup.sh "$DEPLOY_DIR/" 2>/dev/null || true
cp install.sh "$DEPLOY_DIR/" 2>/dev/null || true
cp pyproject.toml "$DEPLOY_DIR/" 2>/dev/null || true

# Create tarball
tar -czf scm-deploy.tar.gz -C $DEPLOY_DIR .
echo -e "${GREEN}✓ Package created: scm-deploy.tar.gz${NC}\n"

# Step 4: Deploy to Raspberry Pi (optional)
read -p "Deploy to Raspberry Pi? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${GREEN}[3/4] Deploying to ${PI_HOST}...${NC}"

    # Copy tarball
    scp scm-deploy.tar.gz ${PI_HOST}:~/ || {
        echo -e "${RED}✗ Failed to copy to Raspberry Pi${NC}"
        echo -e "${YELLOW}Make sure SSH is enabled and ${PI_HOST} is reachable${NC}"
        exit 1
    }

    # Extract and setup on Pi
    echo -e "${GREEN}[4/4] Setting up on Raspberry Pi...${NC}"
    ssh ${PI_HOST} << 'EOF'
        cd ~
        mkdir -p gidence-scm/processor
        cd gidence-scm/processor
        tar -xzf ~/scm-deploy.tar.gz
        chmod +x processor
        echo "✓ Deployment complete!"
        echo "Binary location: ~/gidence-scm/processor/processor"
EOF

    echo -e "${GREEN}✓ Deployment successful!${NC}\n"
    echo -e "${YELLOW}To run on Raspberry Pi:${NC}"
    echo "  ssh ${PI_HOST}"
    echo "  cd ~/gidence-scm/processor"
    echo "  ./processor"
else
    echo -e "${YELLOW}Skipping deployment. Manual deployment:${NC}"
    echo "  scp scm-deploy.tar.gz ${PI_HOST}:~/"
    echo "  ssh ${PI_HOST}"
    echo "  cd ~/gidence-scm/processor && tar -xzf ~/scm-deploy.tar.gz"
fi

echo -e "\n${GREEN}=== Done ===${NC}"
