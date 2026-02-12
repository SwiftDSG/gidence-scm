#!/bin/bash
set -e

# Navigate to controller-web directory
cd /home/gidence/gidence-uics/controller-web

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing frontend dependencies..."
    /home/gidence/.nvm/versions/node/v24.11.0/bin/npm install
fi

# Build if .output doesn't exist or is older than package.json
if [ ! -d ".output" ] || [ "package.json" -nt ".output" ]; then
    echo "Building frontend..."
    /home/gidence/.nvm/versions/node/v24.11.0/bin/npm run build
fi

# Start the frontend server in background
echo "Starting frontend server on port 3000..."
/home/gidence/.nvm/versions/node/v24.11.0/bin/node .output/server/index.mjs &
SERVER_PID=$!

# Wait for server to start
sleep 5

# Launch chromium in kiosk mode
echo "Starting Chromium in kiosk mode..."
DISPLAY=:0 chromium-browser --kiosk http://localhost:3000/hmi &

# Wait for the server process
wait $SERVER_PID