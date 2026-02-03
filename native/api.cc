#include "native/api.h"

#include <sstream>
#include <iomanip>
#include <cctype>

#include "boost/url/url_view.hpp"

#include "motis/config.h"
#include "motis/data.h"
#include "motis/endpoints/routing.h"
#include "motis/endpoints/adr/geocode.h"
#include "motis/endpoints/adr/reverse_geocode.h"

namespace motis::native {

class native_instance {
public:
  explicit native_instance(const std::string& data_path);
  
  motis::data data_;
  motis::config config_;
};

native_instance::native_instance(const std::string& data_path)
    : data_(data_path, config::read(std::filesystem::path{data_path} / "config.yml")),
      config_(data_.config_) {}

native_instance* init(const std::string& data_path) {
  return new native_instance(data_path);
}

void destroy(native_instance* inst) {
  delete inst;
}

// Helper to URL-encode a string
static std::string url_encode(const std::string& value) {
  std::ostringstream escaped;
  escaped.fill('0');
  escaped << std::hex;
  
  for (char c : value) {
    if (std::isalnum(static_cast<unsigned char>(c)) || c == '-' || c == '_' || c == '.' || c == '~') {
      escaped << c;
    } else {
      escaped << std::uppercase;
      escaped << '%' << std::setw(2) << static_cast<int>(static_cast<unsigned char>(c));
      escaped << std::nouppercase;
    }
  }
  
  return escaped.str();
}

// Helper to build URL query string
static std::string build_route_url(coord from, coord to, const std::optional<std::string>& time) {
  std::ostringstream url;
  url << "/api/v1/plan?fromPlace=" << from.lat << "," << from.lon
      << "&toPlace=" << to.lat << "," << to.lon;
  if (time) {
    url << "&time=" << url_encode(*time);
  }
  return url.str();
}

// Helper to convert API mode enum to string
static std::string mode_to_string(api::ModeEnum mode) {
  switch (mode) {
    case api::ModeEnum::WALK: return "WALK";
    case api::ModeEnum::BIKE: return "BIKE";
    case api::ModeEnum::CAR: return "CAR";
    case api::ModeEnum::CAR_PARKING: return "CAR_PARKING";
    case api::ModeEnum::RENTAL: return "RENTAL";
    case api::ModeEnum::TRANSIT: return "TRANSIT";
    case api::ModeEnum::CABLE_CAR: return "CABLE_CAR";
    case api::ModeEnum::FUNICULAR: return "FUNICULAR";
    case api::ModeEnum::RIDE_SHARING: return "RIDE_SHARING";
    default: return "UNKNOWN";
  }
}

std::vector<route> plan_route(native_instance& inst,
                               coord from,
                               coord to,
                               std::optional<std::string> departure_time) {
  std::vector<route> results;
  
  auto url_str = build_route_url(from, to, departure_time);
  auto url = boost::urls::url_view(url_str);
  
  // Match the exact order expected by routing constructor
  auto router = motis::ep::routing{
    inst.config_,
    inst.data_.w_.get(),
    inst.data_.l_.get(),
    inst.data_.pl_.get(),
    inst.data_.elevations_.get(),
    inst.data_.tt_.get(),
    inst.data_.tbd_.get(),
    inst.data_.tags_.get(),
    inst.data_.location_rtree_.get(),
    inst.data_.flex_areas_.get(),
    inst.data_.matches_.get(),
    inst.data_.way_matches_.get(),
    inst.data_.rt_,
    inst.data_.shapes_.get(),
    inst.data_.gbfs_,
    inst.data_.adr_ext_.get(),
    inst.data_.tz_.get(),
    inst.data_.odm_bounds_.get(),
    inst.data_.ride_sharing_bounds_.get(),
    inst.data_.metrics_.get()
  };
  
  try {
    auto response = router(url);
    
    for (const auto& itin : response.itineraries_) {
      route r;
      r.duration_seconds = static_cast<int>(itin.duration_);
      r.transfers = static_cast<int>(itin.transfers_);
      
      for (const auto& leg_obj : itin.legs_) {
        leg l;
        l.mode = mode_to_string(leg_obj.mode_);
        l.from.lat = leg_obj.from_.lat_;
        l.from.lon = leg_obj.from_.lon_;
        l.to.lat = leg_obj.to_.lat_;
        l.to.lon = leg_obj.to_.lon_;
        l.from_name = leg_obj.from_.name_;
        l.to_name = leg_obj.to_.name_;
        l.duration_seconds = static_cast<int>(leg_obj.duration_);
        l.distance_meters = leg_obj.distance_ ? static_cast<int>(*leg_obj.distance_) : 0;
        if (leg_obj.routeShortName_) {
          l.route_short_name = *leg_obj.routeShortName_;
        }
        if (leg_obj.headsign_) {
          l.headsign = *leg_obj.headsign_;
        }
        r.legs.push_back(std::move(l));
      }
      results.push_back(std::move(r));
    }
  } catch (const std::exception& e) {
    std::cerr << "Route planning error: " << e.what() << "\n";
  }
  
  return results;
}

std::vector<location> geocode(native_instance& inst, const std::string& query) {
  std::vector<location> results;
  
  if (!inst.data_.t_ || !inst.data_.f_ || !inst.data_.tc_) {
    return results;
  }
  
  std::string url_str = "/api/v1/geocode?text=" + url_encode(query);
  auto url = boost::urls::url_view(url_str);
  
  auto geocoder = motis::ep::geocode{
    inst.data_.w_.get(),
    inst.data_.pl_.get(),
    inst.data_.matches_.get(),
    inst.data_.tt_.get(),
    inst.data_.tags_.get(),
    *inst.data_.t_,
    *inst.data_.f_,
    *inst.data_.tc_,
    inst.data_.adr_ext_.get()
  };
  
  try {
    auto response = geocoder(url);
    for (const auto& place : response) {
      location loc;
      loc.name = place.name_;
      loc.place_id = place.id_;
      loc.pos.lat = place.lat_;
      loc.pos.lon = place.lon_;
      results.push_back(std::move(loc));
    }
  } catch (const std::exception& e) {
    std::cerr << "Geocode error: " << e.what() << "\n";
  }
  
  return results;
}

std::optional<location> reverse_geocode(native_instance& inst, coord pos) {
  if (!inst.data_.r_ || !inst.data_.t_ || !inst.data_.f_) {
    return std::nullopt;
  }
  
  std::ostringstream url_str;
  url_str << "/api/v1/reverse-geocode?place=" << pos.lat << "," << pos.lon;
  auto url = boost::urls::url_view(url_str.str());
  
  auto reverse = motis::ep::reverse_geocode{
    inst.data_.w_.get(),
    inst.data_.pl_.get(),
    inst.data_.matches_.get(),
    inst.data_.tt_.get(),
    inst.data_.tags_.get(),
    *inst.data_.t_,
    *inst.data_.f_,
    *inst.data_.r_,
    inst.data_.adr_ext_.get()
  };
  
  try {
    auto response = reverse(url);
    if (response.empty()) {
      return std::nullopt;
    }
    const auto& match = response[0];
    location loc;
    loc.name = match.name_;
    loc.place_id = match.id_;
    loc.pos.lat = match.lat_;
    loc.pos.lon = match.lon_;
    return loc;
  } catch (const std::exception& e) {
    std::cerr << "Reverse geocode error: " << e.what() << "\n";
    return std::nullopt;
  }
}

}  // namespace motis::native
