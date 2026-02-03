Native C++ API for Developers
=============================

Files:
- api.h       : Header file with API declarations
- libmotis-native.a : Static library (link with -lmotis-native)

Usage:
  #include "api.h"
  
  auto* inst = motis::native::init("./data");
  auto routes = motis::native::plan_route(*inst, from, to);
  auto places = motis::native::geocode(*inst, "Central Station");
  motis::native::destroy(inst);

Linking:
  g++ your_app.cpp -L. -lmotis-native -lstdc++ -o your_app

Note: This library requires all MOTIS dependencies to be linked.
For a simpler approach, use the HTTP API on localhost:8080
