#include <iostream>
#include <iomanip>
#include <sstream>
#include <string>
#include <vector>

#include "native/api.h"
#include <nlohmann/json.hpp>

using json = nlohmann::json;
using namespace motis::native;

json coord_to_json(const coord& c) {
    return json{{"lat", c.lat}, {"lon", c.lon}};
}

json leg_to_json(const leg& l) {
    json j = {
        {"mode", l.mode},
        {"from_name", l.from_name},
        {"to_name", l.to_name},
        {"from", coord_to_json(l.from)},
        {"to", coord_to_json(l.to)},
        {"duration_seconds", l.duration_seconds},
        {"distance_meters", l.distance_meters}
    };
    if (l.route_short_name) {
        j["route_short_name"] = *l.route_short_name;
    }
    if (l.headsign) {
        j["headsign"] = *l.headsign;
    }
    return j;
}

json route_to_json(const route& r) {
    json legs = json::array();
    for (const auto& l : r.legs) {
        legs.push_back(leg_to_json(l));
    }
    return json{
        {"duration_seconds", r.duration_seconds},
        {"transfers", r.transfers},
        {"legs", legs}
    };
}

json location_to_json(const location& loc) {
    json j = {
        {"name", loc.name},
        {"place_id", loc.place_id},
        {"lat", loc.pos.lat},
        {"lon", loc.pos.lon}
    };
    if (loc.type) {
        j["type"] = *loc.type;
    }
    return j;
}

void send_response(const json& data) {
    json resp = {{"status", "ok"}, {"data", data}};
    std::cout << resp.dump() << std::endl;
}

void send_error(const std::string& msg) {
    json resp = {{"status", "error"}, {"message", msg}};
    std::cout << resp.dump() << std::endl;
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <data_path>\n";
        return 1;
    }
    
    std::string data_path = argv[1];
    
    // Initialize MOTIS
    auto* inst = init(data_path);
    if (!inst) {
        send_error("Failed to initialize MOTIS");
        return 1;
    }
    
    // JSON IPC loop
    std::string line;
    while (std::getline(std::cin, line)) {
        try {
            auto req = json::parse(line);
            std::string cmd = req.value("cmd", "");
            
            if (cmd == "geocode") {
                std::string query = req.value("query", "");
                auto locations = geocode(*inst, query);
                
                json result = json::array();
                for (const auto& loc : locations) {
                    result.push_back(location_to_json(loc));
                }
                send_response(result);
            }
            else if (cmd == "plan_route") {
                coord from{req["from_lat"], req["from_lon"]};
                coord to{req["to_lat"], req["to_lon"]};
                
                auto routes = plan_route(*inst, from, to);
                
                json result = json::array();
                for (const auto& r : routes) {
                    result.push_back(route_to_json(r));
                }
                send_response(result);
            }
            else if (cmd == "reverse_geocode") {
                coord pos{req["lat"], req["lon"]};
                auto loc = reverse_geocode(*inst, pos);
                
                if (loc) {
                    send_response(location_to_json(*loc));
                } else {
                    send_response(nullptr);
                }
            }
            else {
                send_error("Unknown command: " + cmd);
            }
        } catch (const std::exception& e) {
            send_error(std::string("Error: ") + e.what());
        }
    }
    
    destroy(inst);
    return 0;
}
