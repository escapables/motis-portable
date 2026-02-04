#!/bin/bash
# MOTIS Transit USB Launcher
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/data"

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
echo ""
echo "Once started, open in your browser:"
echo "  http://localhost:8080"
echo ""
echo "Press Ctrl+C to stop"
echo ""

# MOTIS expects data directory in current working directory
# or uses default 'data' folder
export MOTIS_DATA_PATH="$DATA_DIR"
cd "$SCRIPT_DIR" || exit 1

# Try different invocations
if ./motis-transit server --help 2>&1 | grep -q "data-path"; then
    # Newer MOTIS with --data-path flag
    ./motis-transit server --data-path "$DATA_DIR"
elif ./motis-transit server --help 2>&1 | grep -q "\-d"; then
    # MOTIS with -d flag
    ./motis-transit server -d "$DATA_DIR"
else
    # Older MOTIS - expects data/ in current directory
    # Create symlink if needed
    if [ ! -d "data" ] && [ -d "$DATA_DIR" ]; then
        ln -sf "$DATA_DIR" data
    fi
    ./motis-transit server
fi
