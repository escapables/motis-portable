# Native MOTIS API - Direct library interface without HTTP

set(native_sources
  api.cc
)

set(native_headers
  api.h
)

# Native API library
add_library(motis-native ${native_sources})
target_include_directories(motis-native PUBLIC ${CMAKE_SOURCE_DIR})
target_compile_features(motis-native PUBLIC cxx_std_23)
target_link_libraries(motis-native
  motislib
  nigiri
  osr
  adr
  boost-json
  motis-api
)

# Example executable (demo mode)
add_executable(motis-native-example example.cc)
target_link_libraries(motis-native-example motis-native)

# IPC example (for Tauri GUI)
add_executable(motis-ipc example_ipc.cc)
target_include_directories(motis-ipc PRIVATE ${CMAKE_SOURCE_DIR}/deps/json/single_include)
target_link_libraries(motis-ipc motis-native)
