<p align="center"><img src="logo.svg" width="196" height="196"></p>

# (`escapables/motis-portable`)

Linux-only offline transit desktop app, built around a portable `usb-bundle/` workflow.

Use case:
- keep app + data on USB/external storage
- import GTFS + OSM PBF once
- run offline via `./RUN.sh` on Linux

Current divergence against upstream (`motis-project/motis`, merge-base `6eb08ad8`, 2026-02-03; upstream `master` last fetched at `d3aeade3`, 2026-02-15):

- `155 files changed`
- `18,838 insertions`
- `2,612 deletions`
- `+16,226 net lines`

This quantifies the code-level migration from upstream server-first workflows toward the portable USB-first Linux application model used in this fork.

## About This Fork

- Runtime model: Tauri UI + `motis://` protocol + `motis-ipc` backend process.
- No localhost fallback in native runtime.
- Deployment target: `usb-bundle/` (portable launcher + native binaries + UI assets).
- This fork is primarily written by OpenAI Codex with human direction/review.

## Quick Start (End User)

### 1. Get a usb-bundle

Preferred:
- download the latest prebuilt bundle artifact from GitHub Releases when available.

Fallback (build locally):

```bash
git clone https://github.com/escapables/motis-portable.git
cd motis-portable
./gui-svelte/build-usb.sh
```

### 2. Import dataset (GTFS + OSM PBF)

```bash
cd usb-bundle
./motis-import.sh /path/to/gtfs.zip /path/to/region.osm.pbf
```

Notes:
- Use files that cover the same geographic area.
- Keep GTFS zipped (`.zip`), do not unpack manually for this script.

### 3. Run app

```bash
./RUN.sh
```

## Build Structure

When building from clone with `./gui-svelte/build-usb.sh`, it compiles/builds:
- Svelte frontend bundle (`ui/build`)
- MOTIS core binary (`build/motis` or `build/motis-transit`)
- IPC bridge binary (`build/native/motis-ipc`)
- Tauri desktop binary (`gui-svelte/src-tauri/target/release/motis-gui-svelte`)

Then it assembles `usb-bundle/` with launcher/import scripts and runtime files.

## usb-bundle Contents

`usb-bundle/` contains:
- `motis-gui-svelte`
- `motis-ipc`
- `motis`
- `RUN.sh`
- `motis-import.sh`
- `sweden-route-fix.lua`
- `ui/`
- `data/` (imported by user)

## Troubleshooting

- `Missing .../data/config.yml`:
  run import first: `./motis-import.sh <gtfs.zip> <osm.pbf>`.
- `Permission denied` when running binary from USB:
  run through launcher: `./RUN.sh`.
- Build-only issue:
  try `./gui-svelte/build-usb.sh --skip-pnpm-install` or `--offline` if dependencies are already cached.

## For Developers

- Docs index: `docs/README.md`
- Validate docs: `./bin/validate-docs`
- Local gate: `./bin/test-gate`

## Upstream Project

- `motis-project/motis`: https://github.com/motis-project/motis
