# MOTIS Transit - Svelte GUI (Tauri)

This is the primary desktop app target for the portable offline fork.

## Purpose

Run the Svelte web UI inside Tauri using an IPC-first backend path:

- UI requests use `motis://`.
- Rust protocol handler maps requests to IPC/native calls.
- `motis-ipc` communicates with MOTIS core via native C++ API.

No localhost server is required for normal operation.

## Build

```bash
cd ui
pnpm install
pnpm build

cd ../gui-svelte/src-tauri
cargo tauri build
```

Or use:

```bash
cd gui-svelte
./build-usb.sh
```

`build-usb.sh` now prepares a complete `usb-bundle-svelte/` from tracked templates in `gui-svelte/usb-template/` and will build missing `motis` / `motis-ipc` binaries automatically.

## Run

From a prepared bundle/USB root with `data/` present:

```bash
./RUN.sh
```

Direct run is also possible when executable permissions are available:

```bash
./motis-gui-svelte --data-path ./data
```

## Current Status

- Route planning works.
- Geocoding and reverse geocoding work.
- Vector tiles render.
- Glyph labels render.
- Major map/interactivity endpoints are routed via protocol passthrough.

## Key Files

- `src-tauri/src/main.rs`
- `src-tauri/src/native.rs`
- `src-tauri/src/protocol.rs`
- `src-tauri/tauri.conf.json`
- `build-usb.sh`
