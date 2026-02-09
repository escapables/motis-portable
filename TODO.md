# Linux-Only Cleanup TODO

- [x] 1. Audit repository for Windows/macOS build/runtime references.
- [x] 2. Remove Windows/macOS CI jobs and artifacts from `.github/workflows/ci.yml`.
- [x] 3. Remove macOS presets from `CMakePresets.json`.
- [x] 4. Restrict Tauri cargo target config to Linux-only targets in `gui-svelte/src-tauri/.cargo/config.toml`.
- [x] 5. Simplify CMake logic for Linux-only tooling/bootstrap (`cmake/pkg.cmake`, `cmake/buildcache.cmake`, `CMakeLists.txt`, `native/CMakeLists.txt`).
- [x] 6. Remove or update Windows/macOS setup docs (`docs/README.md`, `docs/dev-setup-server.md`, remove obsolete setup docs).
- [x] 7. Verify no actionable Windows/macOS support paths remain (excluding lockfile metadata and Tauri "windows" UI key names).
- [x] 8. Final review: summarize changes, risks, and follow-up options.
