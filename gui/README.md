# MOTIS Transit GUI

A Tauri-based desktop application for MOTIS transit planning — **Portable USB Edition**.

## Features

- 🚀 **Fully Portable** — Runs from USB stick, no installation required
- 💾 **Self-Contained** — No registry entries, no external dependencies
- 🗺️ **Transit Planning** — Route planning with real-time data
- 🔍 **Geocoding** — Address autocomplete and reverse geocoding
- 📁 **Runtime Data Path** — Configure data directory at runtime

## Project Structure

```
motis-gui/
├── motis-gui              ← executable
├── lib/                   ← bundled libraries (if dynamic)
└── data/                  ← GTFS + OSM data (user provides this)
    ├── gtfs/
    └── osm/
```

## Portable Architecture

### 1. Executable-Relative Paths

All paths are resolved relative to the executable location:

```rust
let exe_dir = std::env::current_exe()?.parent()?;
let data_path = exe_dir.join("data");  // ./data next to exe
```

This ensures the app works regardless of where the USB stick is mounted:
- Windows: `E:\motis-gui\motis-gui.exe` → looks for `E:\motis-gui\data\`
- Linux: `/media/user/USB/motis-gui/motis-gui` → looks for `/media/user/USB/motis-gui/data/`

### 2. Runtime Data Path Configuration

Users can set/change the data path at runtime via:
- **Default**: `./data` folder next to executable
- **Custom**: Any absolute or relative path (relative to executable)

Commands exposed:
- `get_default_data_path_cmd()` → Returns default `./data` path
- `init_native(data_path?: String)` → Initialize with optional custom path
- `reinit_with_data_path(path: String)` → Reinitialize with new path
- `check_data_path_exists(path: String)` → Verify path without initializing

### 3. Static Linking Configuration

**Cargo.toml** includes platform-specific static linking:

```toml
# Windows: Static CRT
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]

# Linux: rpath for bundled libs
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-Wl,-rpath,$ORIGIN/lib"]

# Optional: Fully static with musl
[target.x86_64-unknown-linux-musl]
rustflags = ["-C", "link-arg=-static"]
```

**build.rs** handles C++ static linking:
```rust
// Static libstdc++ on Linux where available
cc_build.flag_if_supported("-static-libstdc++");
```

## Build Instructions

### Prerequisites

- Rust toolchain (1.77.0+)
- C++ compiler (g++, clang++, or MSVC)
- Tauri CLI: `cargo install tauri-cli`
- MOTIS native library built at `../build/native/libmotis-native.a`

### Standard Build

```bash
cd gui/src-tauri
cargo build --release
```

### Fully Static Linux Build (Recommended for USB)

```bash
# Install musl target
rustup target add x86_64-unknown-linux-musl

# Build fully static binary
cargo build --release --target x86_64-unknown-linux-musl
```

### Windows Build

```bash
cargo build --release
# Produces motis-gui.exe with statically linked CRT
```

## USB Deployment

1. **Create folder structure** on USB:
   ```
   /motis-gui/
   ├── motis-gui          (or motis-gui.exe on Windows)
   └── data/
       ├── gtfs/          # GTFS feeds
       │   └── ...
       └── osm/           # OSM extracts
           └── ...
   ```

2. **Run the app**:
   - Double-click `motis-gui` (or `motis-gui.exe`)
   - The app will auto-detect the `./data` folder
   - If not found, the config panel opens to set the path

3. **Optional: Custom data location**:
   - Use the "Data Path Configuration" panel
   - Set absolute path (e.g., `/home/user/motis-data`)
   - Or relative path (e.g., `../../shared-data`)

## API Commands

### Data Path Management

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `get_default_data_path_cmd` | - | `String` | Get default `./data` path |
| `get_current_data_path` | - | `Option<String>` | Get currently set path |
| `check_data_path_exists` | `path: String` | `bool` | Verify path exists |
| `init_native` | `data_path?: String` | `String` | Initialize with path (null = default) |
| `reinit_with_data_path` | `data_path: String` | `String` | Reinitialize with new path |

### Transit Operations

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `plan_route_cmd` | `from_lat, from_lon, to_lat, to_lon` | `Vec<Route>` | Plan routes |
| `geocode_cmd` | `query: String` | `Vec<Location>` | Geocode address |
| `reverse_geocode_cmd` | `lat, lon` | `Option<Location>` | Reverse geocode |
| `destroy_native` | - | `()` | Cleanup instance |

## Data Types

### Route
```rust
{
  duration_seconds: i32,
  transfers: i32,
  legs: [RouteLeg]
}
```

### RouteLeg
```rust
{
  mode: String,              // WALK, BUS, TRAM, RAIL, etc.
  from_name: String,
  to_name: String,
  from_lat, from_lon: f64,
  to_lat, to_lon: f64,
  duration_seconds: i32,
  distance_meters: i32,
  route_short_name?: String,
  headsign?: String
}
```

### Location
```rust
{
  name: String,
  place_id: String,
  lat, lon: f64,
  type?: String
}
```

## FFI Architecture

1. **C++ Library** (`libmotis-native.a`): Native MOTIS functionality
2. **C Wrapper** (`c_wrapper.cpp`): C-compatible FFI layer
3. **Rust FFI** (`native.rs`): Safe Rust bindings
4. **Tauri Commands** (`main.rs`): Frontend-exposed API

## Troubleshooting

### "Data path does not exist"
- Ensure the `data/` folder exists next to the executable
- Use the config panel to set the correct path
- Check that the path is readable

### "Failed to initialize MOTIS"
- Verify the data directory contains valid GTFS/OSM data
- Check that libmotis-native.a was linked correctly
- Run with `RUST_BACKTRACE=1` for details

### Platform-Specific Issues

**Linux**: If `libstdc++` is not available, use the musl build:
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

**macOS**: The app uses `@executable_path/lib` for bundled libraries. Copy any required `.dylib` files to a `lib/` folder next to the executable.

**Windows**: The CRT is statically linked by default. No Visual C++ redistributable needed.

## License

MIT
