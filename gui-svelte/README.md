# MOTIS Transit - Svelte UI Edition

Full-featured Svelte-based MOTIS GUI with custom protocol for IPC communication.

## Architecture

```
┌─────────────────┐     motis://            ┌──────────────────┐
│  Svelte UI      │ ◄─────────────────────► │  Tauri Backend   │
│  (ui/build)     │   (custom protocol)     │  (Rust)          │
│                 │                         │                  │
│  - MapLibre     │                         │  - protocol.rs   │
│  - RailViz      │                         │  - native.rs     │
│  - Isochrones   │                         │                  │
└─────────────────┘                         └────────┬─────────┘
                                                     │
                                            Subprocess IPC
                                                     │
                                              ┌──────▼──────┐
                                              │  motis-ipc  │
                                              │  (C++)      │
                                              └─────────────┘
```

## How It Works

1. **Tauri Detection**: Svelte UI detects Tauri via `window.__TAURI__`
2. **Protocol Switch**: Sets API base URL to `motis://localhost`
3. **Request Interception**: Tauri custom protocol handler intercepts `motis://` requests
4. **IPC Bridge**: Protocol handler routes requests to `motis-ipc` subprocess
5. **No HTTP Server**: Direct stdin/stdout communication - no localhost needed

## File Structure

```
gui-svelte/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs       # Tauri entry, CLI handling
│   │   ├── native.rs     # IPC subprocess management
│   │   └── protocol.rs   # motis:// request routing
│   └── Cargo.toml
├── build-usb.sh          # Build script
└── README.md

usb-bundle-svelte/        # USB output directory
├── motis-gui-svelte      # GUI executable
├── motis-ipc             # IPC backend
├── motis-transit         # Import tool
├── RUN.sh                # FAT32 launcher
└── data/                 # Transit data
```

## Building

```bash
# Build Svelte UI
cd ui && pnpm install && pnpm build

# Build Tauri app
cd gui-svelte/src-tauri
cargo tauri build

# Or use the build script
cd gui-svelte && ./build-usb.sh
```

`cargo tauri build` is required for portable distribution because it embeds frontend assets
and builds with the custom protocol setup used by the IPC transport.

## Running

```bash
cd usb-bundle-svelte

# With data imported:
./RUN.sh

# Or directly (requires executable permissions):
./motis-gui-svelte --data-path ./data
```

## API Endpoints Implemented

| Endpoint | Status | Description |
|----------|--------|-------------|
| `/api/v1/geocode` | ✅ Working | Address search |
| `/api/v1/reverse-geocode` | ✅ Working | Coords to address |
| `/api/v1/plan` | ✅ Working | Route planning |
| `/api/v5/plan` | ✅ Working | Route planning (v5) |
| `/api/v1/trip` | 🟡 Stub | Trip details |
| `/api/v5/stoptimes` | 🟡 Stub | Stop departures |
| `/api/v1/map/trips` | 🟡 Stub | RailViz trips |
| `/api/v1/tiles/*` | 🟡 **PARTIAL** | Tiles served (200KB+) but not rendering visually |
| `/api/v1/one-to-many` | 🟡 Stub | Street routing |
| `/api/v1/one-to-all` | 🟡 Stub | Isochrones |

**Legend:**
- ✅ Working: Full IPC implementation
- 🟡 Stub: Returns empty/placeholder data
- 🟡 Partial: Backend works but frontend rendering issue

## Known Issues

### Vector Tiles Not Rendering (Investigating)
The tile backend is working correctly (serving 200KB+ tiles via IPC), but tiles are not appearing on the map. This is likely due to:

1. **Tauri v2 CSP restrictions** on custom protocols in web workers
2. **MapLibre GL JS** workers unable to access `motis://` custom protocol
3. **Response headers** may need adjustment for webview compatibility

**Evidence tiles ARE working:**
```
[MOTIS-PROTOCOL] Tile request: /tiles/10/563/300.mvt
[MOTIS-PROTOCOL] Got base64 data: 275240 chars  
[MOTIS-PROTOCOL] Tile decoded: 206429 bytes
```

**Potential solutions:**
- Switch to `http://motis.localhost` format (Tauri v2 style)
- Use Tauri `asset` protocol instead of custom protocol
- Proxy tiles through Tauri invoke() commands
- Fall back to raster tiles (OpenStreetMap) instead of vector tiles

## Extending

To add a new endpoint:

1. **Add handler in `protocol.rs`**:
```rust
"/api/v1/myendpoint" => handle_myendpoint(&params)
```

2. **Add sync function in `native.rs`**:
```rust
pub fn myendpoint_sync(param: &str) -> Result<MyData, Box<dyn Error>> {
    let cmd = format!(r#"{{"cmd":"my_cmd","param":"{}"}}"#, param);
    // ... send via IPC
}
```

3. **Add command in C++ backend** (`native/example_ipc.cc`):
```cpp
else if (cmd == "my_cmd") {
    auto result = my_api_function(req["param"]);
    send_response(result);
}
```

## USB Portability

- ✅ No installation required
- ✅ No external dependencies
- ✅ FAT32 compatible (uses /tmp copy)
- ✅ Offline capable (all data local)
- ✅ Cross-platform (Linux binary included)

## Debugging

```bash
# Enable debug console
./motis-gui-svelte --data-path ./data --debug

# Check IPC communication
tail -f /tmp/motis-ipc.log
```
