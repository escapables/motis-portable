#!/bin/bash
# MOTIS Transit USB Launcher
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/data"
UI_DIR="${SCRIPT_DIR}/ui"

if [ ! -f "$DATA_DIR/config.yml" ]; then
    echo "ERROR: MOTIS data not found!"
    echo "Expected: $DATA_DIR/config.yml"
    echo ""
    echo "To create data:"
    echo "  1. cd to this directory"
    echo "  2. ./motis-transit config your_region.osm.pbf your_gtfs.zip"
    echo "  3. ./motis-transit import"
    echo "  4. ./start.sh"
    exit 1
fi

echo "=================================="
echo "MOTIS Transit - Starting Server..."
echo "=================================="
echo "Data: $DATA_DIR"
echo "UI: $UI_DIR"
echo ""
echo "Once started, open in your browser:"
echo "  http://localhost:8080"
echo ""
echo "Press Ctrl+C to stop"
echo ""

# Change to script directory
cd "$SCRIPT_DIR" || exit 1

# Create data symlink for MOTIS
if [ ! -d "data" ] && [ -d "$DATA_DIR" ]; then
    ln -sf "$DATA_DIR" data
fi

# Set web folder environment variable for MOTIS to find UI
export MOTIS_WEB_FOLDER="$UI_DIR"

# Start MOTIS server
# The server will serve static files from the web folder
./motis-transit server
