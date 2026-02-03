MOTIS Transit - Portable USB Edition
====================================

WHAT IS THIS?
A portable public transit planning application that runs entirely
from a USB stick. No installation required!

QUICK START:
1. Copy your MOTIS data to the 'data/' folder
   (or run the setup steps below)

2. Double-click or run: ./start.sh

3. Open http://localhost:8080 in your browser

SETUP - Creating Data for Your Region:
--------------------------------------
You need two files:
1. GTFS feed (transit schedules) - from your local transit agency
2. OSM extract (.osm.pbf) - from openstreetmap.org

Steps:
  1. Place files in this directory
  2. Run: ./motis-transit config your_region.osm.pbf your_gtfs.zip
  3. Run: ./motis-transit import
  4. Wait for import to complete (5-30 minutes depending on size)
  5. Run: ./start.sh

USB PORTABILITY:
- All data stored in ./data/ folder
- No registry entries
- No system dependencies
- Works on any x86_64 Linux system

TROUBLESHOOTING:
- Port 8080 already in use? Edit start.sh to use different port
- Missing libraries? Install: libssl1.1 (for OpenSSL 1.1 compatibility)

MORE INFO:
https://github.com/motis-project/motis
