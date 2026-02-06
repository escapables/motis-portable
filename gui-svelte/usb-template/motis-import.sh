#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="$SCRIPT_DIR/data"

if [ $# -lt 2 ]; then
    echo "Usage: $0 <gtfs.zip> <osm.pbf>"
    echo ""
    echo "Example:"
    echo "  $0 ~/downloads/gtfs.zip ~/downloads/city.osm.pbf"
    exit 1
fi

GTFS_FILE="$1"
OSM_FILE="$2"

if [ ! -f "$GTFS_FILE" ]; then
    echo "ERROR: GTFS file not found: $GTFS_FILE"
    exit 1
fi

if [ ! -f "$OSM_FILE" ]; then
    echo "ERROR: OSM file not found: $OSM_FILE"
    exit 1
fi

GTFS_ABS="$(cd "$(dirname "$GTFS_FILE")" && pwd)/$(basename "$GTFS_FILE")"
OSM_ABS="$(cd "$(dirname "$OSM_FILE")" && pwd)/$(basename "$OSM_FILE")"

echo "MOTIS Data Import"
echo "================="
echo "GTFS: $GTFS_ABS"
echo "OSM:  $OSM_ABS"
echo ""

if [ -f "$SCRIPT_DIR/motis" ]; then
    MOTIS_BIN="$SCRIPT_DIR/motis"
elif [ -f "$SCRIPT_DIR/../build/motis" ]; then
    MOTIS_BIN="$SCRIPT_DIR/../build/motis"
else
    echo "ERROR: motis executable not found"
    echo "Please build MOTIS first: cmake --build build --target motis"
    exit 1
fi

echo "Using: $MOTIS_BIN"
echo ""

rm -rf "$DATA_DIR"
mkdir -p "$DATA_DIR"

cat > "$SCRIPT_DIR/config.yml" << EOF
osm: $OSM_ABS
timetable:
  first_day: TODAY
  num_days: 365
  datasets:
    sweden:
      path: $GTFS_ABS
tiles:
  profile: $SCRIPT_DIR/tiles-profile.lua
  db_size: 274877906944
  flush_threshold: 100000
street_routing: true
geocoding: true
reverse_geocoding: true
EOF

if [ ! -f "$SCRIPT_DIR/tiles-profile.lua" ]; then
    if [ -f "$SCRIPT_DIR/../deps/tiles/profile/full.lua" ]; then
        cp "$SCRIPT_DIR/../deps/tiles/profile/full.lua" "$SCRIPT_DIR/tiles-profile.lua"
    elif [ -f "$SCRIPT_DIR/deps/tiles/profile/full.lua" ]; then
        cp "$SCRIPT_DIR/deps/tiles/profile/full.lua" "$SCRIPT_DIR/tiles-profile.lua"
    fi
fi

echo "Config created: $SCRIPT_DIR/config.yml"
echo ""

echo "Starting import (this may take several minutes)..."
cd "$SCRIPT_DIR"
"$MOTIS_BIN" import

echo ""
echo "Organizing data files..."
for item in adr matches flex_areas osr tt tags.bin timetable_metrics.json; do
    if [ -e "$item" ]; then
        mv "$item" "$DATA_DIR/" 2>/dev/null || true
    fi
done

cp "$SCRIPT_DIR/config.yml" "$DATA_DIR/"

echo ""
echo "Import complete!"
echo "Data directory: $DATA_DIR"
echo "Data size: $(du -sh "$DATA_DIR" | cut -f1)"
echo ""
echo "You can now run: ./RUN.sh"
