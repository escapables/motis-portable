#!/bin/bash
# MOTIS Transit USB Launcher
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/data"

if [ ! -f "$DATA_DIR/config.yml" ]; then
    echo "ERROR: MOTIS data not found!"
    echo "Please copy your data directory here."
    echo ""
    echo "To create data:"
    echo "  1. Download GTFS feed (transit schedules)"
    echo "  2. Download OSM extract (.osm.pbf) for your region"
    echo "  3. Run: ./motis-transit config region.osm.pbf region.gtfs.zip"
    echo "  4. Run: ./motis-transit import"
    echo "  5. The 'data' folder will be created"
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

"${SCRIPT_DIR}/motis-transit" server --data-path "$DATA_DIR"
