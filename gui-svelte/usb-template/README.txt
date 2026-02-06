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

Important files:
- `RUN.sh`             FAT32-safe launcher (recommended)
- `motis-gui-svelte`  Native GUI executable
- `motis-ipc`         IPC backend executable
- `data/`             Your imported MOTIS data

Troubleshooting:
- "Data folder not found":
  Import data first with `motis-import.sh`, then verify `data/` exists.
- "Missing .../data/config.yml":
  Data exists but is incomplete; import again with `motis-import.sh`.
- "Permission denied":
  Use `./RUN.sh` (it copies executables to `/tmp` and sets permissions).
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

Viktiga filer:
- `RUN.sh`             FAT32-säker startskript (rekommenderas)
- `motis-gui-svelte`  Native GUI-program
- `motis-ipc`         IPC-backend
- `data/`             Din importerade MOTIS-data

Felsökning:
- "Data folder not found":
  Importera data först med `motis-import.sh`, kontrollera sedan att `data/` finns.
- "Missing .../data/config.yml":
  Data finns men är ofullständig; importera igen med `motis-import.sh`.
- "Permission denied":
  Använd `./RUN.sh` (kopierar körbara filer till `/tmp` och sätter rättigheter).
- Inga resor hittas:
  Kontrollera att GTFS + OSM-området matchar platsen du söker i.
