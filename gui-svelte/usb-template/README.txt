MOTIS Portable USB (Svelte UI)
==============================

ENGLISH
-------
What this is:
- Portable, offline MOTIS app for Linux.
- Native app (Tauri + IPC), no browser required.
- IPC-first design: no localhost needed for normal use.

Quick start:
1. Put/import your data in `data/` (same folder as this file).
   Example:
   ./motis-import.sh /path/to/gtfs.zip /path/to/region.osm.pbf

2. Start the app (recommended):
   ./RUN.sh

3. Optional debug mode:
   ./RUN.sh --debug

4. Launcher checks only (no GUI start):
   ./RUN.sh --launcher-self-test
   Optional external data path:
   MOTIS_DATA_PATH=/path/to/data ./RUN.sh

Important files:
- `RUN.sh`             FAT32-safe launcher (recommended)
- `motis-gui-svelte`  Native GUI executable
- `motis-ipc`         IPC backend executable
- `data/`             Your imported MOTIS data
- `launcher.log`      Last launcher run log
- `error.txt`         Launcher failure summary (created on errors)

Troubleshooting:
- "Data folder not found":
  Import data first with `motis-import.sh`, then verify `data/` exists.
- "Missing .../data/config.yml":
  Data exists but is incomplete; import again with `motis-import.sh`.
- "Permission denied":
  Use `./RUN.sh` (it copies executables to `/tmp` and sets permissions).
- "Could not find an executable temp directory":
  `/tmp` (or `XDG_RUNTIME_DIR`) may be mounted with `noexec`. Remount with `exec`
  or run on a system where one temp location allows execution.
- "Missing executable: .../motis-ipc":
  Bundle is incomplete. Re-copy `motis-ipc` next to `RUN.sh`.
- No routes found:
  Confirm your GTFS + OSM region matches the area you are searching.

More info:
- https://github.com/escapables/motis-portable
- https://github.com/motis-project/motis


SVENSKA
-------
Detta är:
- En portabel, offline MOTIS-app för Linux.
- En native-app (Tauri + IPC), ingen webbläsare behövs.
- IPC-först: localhost behövs normalt inte.

Snabbstart:
1. Lägg/importera data i `data/` (samma mapp som denna fil).
   Exempel:
   ./motis-import.sh /sökväg/till/gtfs.zip /sökväg/till/region.osm.pbf

2. Starta appen (rekommenderat):
   ./RUN.sh

3. Valfritt felsökningsläge:
   ./RUN.sh --debug

4. Endast launcher-kontroll (startar inte GUI):
   ./RUN.sh --launcher-self-test
   Valfri extern datasökväg:
   MOTIS_DATA_PATH=/sökväg/till/data ./RUN.sh

Viktiga filer:
- `RUN.sh`             FAT32-säker startskript (rekommenderas)
- `motis-gui-svelte`  Native GUI-program
- `motis-ipc`         IPC-backend
- `data/`             Din importerade MOTIS-data
- `launcher.log`      Logg från senaste launcher-körning
- `error.txt`         Kort felsammanfattning (skapas vid fel)

Felsökning:
- "Data folder not found":
  Importera data först med `motis-import.sh`, kontrollera sedan att `data/` finns.
- "Missing .../data/config.yml":
  Data finns men är ofullständig; importera igen med `motis-import.sh`.
- "Permission denied":
  Använd `./RUN.sh` (kopierar körbara filer till `/tmp` och sätter rättigheter).
- "Could not find an executable temp directory":
  `/tmp` (eller `XDG_RUNTIME_DIR`) kan vara monterad med `noexec`. Montera om med
  `exec` eller kör på ett system där en temporär katalog tillåter körning.
- "Missing executable: .../motis-ipc":
  Paketet är ofullständigt. Kopiera `motis-ipc` bredvid `RUN.sh` igen.
- Inga resor hittas:
  Kontrollera att GTFS + OSM-området matchar platsen du söker i.
