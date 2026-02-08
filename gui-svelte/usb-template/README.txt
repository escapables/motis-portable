Lysande Linjetrafik
==============================

ENGLISH
-------
No terminal needed.
Use these steps in your file manager (Files/Nautilus/Dolphin/etc.).

Start the app (recommended):
1. Open the USB folder.
2. Double-click `RUN.sh`.
3. If Linux asks, choose `Run` (not `Open`).
4. Wait until the MOTIS window appears.

Before first start:
1. Open the `data` folder.
2. Check that it contains `config.yml`.
3. If `config.yml` is missing, data is not imported yet.
   Ask the person who prepared the USB to import data first.

If double-click does not start the app:
1. Right-click `RUN.sh`.
2. Choose `Run as Program` (or similar).
3. If that option is missing, open Properties for `RUN.sh` and allow executing
   it as a program, then try again.

If something goes wrong:
1. Open `error.txt` in the same folder (if it exists).
2. Send its contents to support/the person who gave you this USB.
3. You can also send `launcher.log` for more details.

Important files:
- `RUN.sh`             Main launcher (best for USB/FAT32)
- `motis-gui-svelte`   App program
- `motis-ipc`          Backend process
- `data/`              Transit data
- `error.txt`          Short error message (created on failure)
- `launcher.log`       Detailed launcher log

Advanced (terminal users):
- Start in debug mode: `./RUN.sh --debug`
- Launcher self test only: `./RUN.sh --launcher-self-test`
- External data path: `MOTIS_DATA_PATH=/path/to/data ./RUN.sh`

More info:
- https://github.com/escapables/motis-portable
- https://github.com/motis-project/motis


SVENSKA
-------
Ingen terminal behövs.
Använd stegen nedan i filhanteraren (Filer/Nautilus/Dolphin osv.).

Starta appen (rekommenderas):
1. Öppna USB-mappen.
2. Dubbelklicka på `RUN.sh`.
3. Om Linux frågar, välj `Kör` (inte `Öppna`).
4. Vänta tills MOTIS-fönstret visas.

Innan första start:
1. Öppna mappen `data`.
2. Kontrollera att `config.yml` finns där.
3. Om `config.yml` saknas är data inte importerad ännu.
   Be personen som förberedde USB-minnet att importera data först.

Om dubbelklick inte startar appen:
1. Högerklicka på `RUN.sh`.
2. Välj `Kör som program` (eller liknande).
3. Om valet saknas, öppna Egenskaper för `RUN.sh` och tillåt att filen får
   köras som program. Försök sedan igen.

Om något går fel:
1. Öppna `error.txt` i samma mapp (om filen finns).
2. Skicka innehållet till support/personen som gav dig USB-minnet.
3. Du kan också skicka `launcher.log` för mer detaljer.

Viktiga filer:
- `RUN.sh`             Huvudstartare (bäst för USB/FAT32)
- `motis-gui-svelte`   Appens programfil
- `motis-ipc`          Backend-process
- `data/`              Trafikdata
- `error.txt`          Kort felmeddelande (skapas vid fel)
- `launcher.log`       Detaljerad startlogg

Avancerat (för terminalanvändare):
- Starta i debug-läge: `./RUN.sh --debug`
- Endast launcher-test: `./RUN.sh --launcher-self-test`
- Extern datasökväg: `MOTIS_DATA_PATH=/sökväg/till/data ./RUN.sh`
