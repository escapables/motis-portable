#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TMP_DIR="/tmp/motis-svelte-$$"

echo "MOTIS Transit (Svelte UI) - USB Launcher"
echo "========================================"

mkdir -p "$TMP_DIR"

cleanup() {
    echo "Cleaning up..."
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

echo "Copying executables to /tmp..."
cp "$SCRIPT_DIR/motis-gui-svelte" "$TMP_DIR/"
cp "$SCRIPT_DIR/motis-ipc" "$TMP_DIR/motis-ipc-usb" 2>/dev/null || true

chmod +x "$TMP_DIR/motis-gui-svelte"
chmod +x "$TMP_DIR/motis-ipc-usb" 2>/dev/null || true

if [ -d "$SCRIPT_DIR/data" ]; then
    DATA_PATH="$SCRIPT_DIR/data"
else
    echo "ERROR: Data folder not found at $SCRIPT_DIR/data"
    echo ""
    echo "Please import transit data first:"
    echo "  ./motis-import.sh /path/to/gtfs.zip /path/to/osm.pbf"
    exit 1
fi

if [ ! -f "$DATA_PATH/config.yml" ]; then
    echo "ERROR: Missing $DATA_PATH/config.yml"
    echo ""
    echo "This data folder is empty or incomplete."
    echo "Import data first:"
    echo "  ./motis-import.sh /path/to/gtfs.zip /path/to/osm.pbf"
    exit 1
fi

echo "Data path: $DATA_PATH"
echo "Starting MOTIS Transit..."
echo ""

export MOTIS_DATA_PATH="$DATA_PATH"
export MOTIS_IPC_PATH="$TMP_DIR/motis-ipc-usb"

exec "$TMP_DIR/motis-gui-svelte" --data-path "$DATA_PATH" "$@"
