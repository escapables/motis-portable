#pragma once

#include <string>
#include <vector>
#include <optional>
#include <memory>

namespace motis::native {

// Geo coordinate
struct coord {
  double lat;
  double lon;
};

// Simple route leg
struct leg {
  std::string mode;
  std::string from_name;
  std::string to_name;
  coord from;
  coord to;
  int duration_seconds;
  int distance_meters;
  std::optional<std::string> route_short_name;
  std::optional<std::string> headsign;
};

// Route result
struct route {
  int duration_seconds;
  int transfers;
  std::vector<leg> legs;
};

// Geocode result
struct location {
  std::string name;
  std::string place_id;
  coord pos;
  std::optional<std::string> type;
};

// Opaque handle to MOTIS instance
class native_instance;

// Initialize MOTIS native API
// Note: returns raw pointer to avoid unique_ptr with incomplete type issues
native_instance* init(const std::string& data_path);

// Cleanup
void destroy(native_instance* inst);

// Route planning
std::vector<route> plan_route(native_instance& inst,
                               coord from,
                               coord to,
                               std::optional<std::string> departure_time = std::nullopt);

// Geocoding
std::vector<location> geocode(native_instance& inst, const std::string& query);

// Reverse geocoding
std::optional<location> reverse_geocode(native_instance& inst, coord pos);

}  // namespace motis::native
