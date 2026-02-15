#include "gtest/gtest.h"

#include <sys/wait.h>
#include <cstdio>
#include <array>
#include <filesystem>
#include <optional>
#include <string>
#include <system_error>

#include "boost/json.hpp"

#include "../native/api.h"
#include "motis/config.h"
#include "motis/import.h"

using namespace std::string_view_literals;

namespace fs = std::filesystem;
namespace json = boost::json;

namespace {

constexpr auto const kGTFS = R"(
# agency.txt
agency_id,agency_name,agency_url,agency_timezone
DB,Deutsche Bahn,https://deutschebahn.com,Europe/Berlin

# stops.txt
stop_id,stop_name,stop_lat,stop_lon,location_type,parent_station,platform_code
DA,DA Hbf,49.87260,8.63085,1,,
DA_10,DA Hbf,49.87336,8.62926,0,DA,10
FFM,FFM Hbf,50.10701,8.66341,1,,
FFM_12,FFM Hbf,50.10658,8.66178,0,FFM,12

# routes.txt
route_id,agency_id,route_short_name,route_long_name,route_desc,route_type
ICE,DB,ICE,,,101

# trips.txt
route_id,service_id,trip_id,trip_headsign,block_id
ICE,S1,ICE,,

# stop_times.txt
trip_id,arrival_time,departure_time,stop_id,stop_sequence,pickup_type,drop_off_type
ICE,00:35:00,00:35:00,DA_10,0,0,0
ICE,00:45:00,00:45:00,FFM_12,1,0,0

# calendar_dates.txt
service_id,date,exception_type
S1,20190501,1
)"sv;

fs::path const kNativeApiDataPath = "test/native-ipc-regression-data";

struct command_result {
  int exit_code_{-1};
  std::string stdout_{};
};

command_result run_command_capture_stdout(std::string const& cmd) {
  command_result result;
  auto* pipe = popen(cmd.c_str(), "r");
  if (pipe == nullptr) {
    return result;
  }

  std::array<char, 256> buffer{};
  while (fgets(buffer.data(), static_cast<int>(buffer.size()), pipe) !=
         nullptr) {
    result.stdout_ += buffer.data();
  }

  auto const status = pclose(pipe);
  if (status != -1 && WIFEXITED(status)) {
    result.exit_code_ = WEXITSTATUS(status);
  }
  return result;
}

void prepare_native_data(fs::path const& data_path) {
  auto ec = std::error_code{};
  fs::remove_all(data_path, ec);

  auto const c = motis::config{
      .server_ = {{.web_folder_ = "ui/build", .n_threads_ = 1U}},
      .osm_ = {"test/resources/test_case.osm.pbf"},
      .timetable_ = motis::config::timetable{
          .first_day_ = "2019-05-01",
          .num_days_ = 2,
          .datasets_ = {{"test", {.path_ = std::string{kGTFS}}}}}};
  static_cast<void>(motis::import(c, data_path.string(), true));
}

std::string trim_newline(std::string value) {
  while (!value.empty() && (value.back() == '\n' || value.back() == '\r')) {
    value.pop_back();
  }
  return value;
}

std::optional<fs::path> find_ipc_binary() {
  std::array<fs::path, 3> const candidates = {
      fs::path{"build-local-debug/native/motis-ipc"},
      fs::path{"build/native/motis-ipc"},
      fs::path{"build/motis-ipc"},
  };
  for (auto const& candidate : candidates) {
    if (fs::exists(candidate)) {
      return candidate;
    }
  }
  return std::nullopt;
}

class native_wrapper_regression_test : public ::testing::Test {
public:
  void SetUp() override {
    prepare_native_data(kNativeApiDataPath);
    inst_ = motis::native::init(kNativeApiDataPath.string());
    ASSERT_NE(nullptr, inst_);
  }

  void TearDown() override {
    motis::native::test_support::clear_fault_injection();
    if (inst_ != nullptr) {
      motis::native::destroy(inst_);
      inst_ = nullptr;
    }
    auto ec = std::error_code{};
    fs::remove_all(kNativeApiDataPath, ec);
  }

protected:
  motis::native::native_instance* inst_{nullptr};
};

}  // namespace

TEST(motis_native_ipc, init_throw_returns_structured_json_error) {
  auto const ipc_binary = find_ipc_binary();
  ASSERT_TRUE(ipc_binary.has_value());
  auto const cmd =
      ipc_binary->string() + " test/does-not-exist-for-ipc-init 2>/dev/null";
  auto const result = run_command_capture_stdout(cmd);

  ASSERT_EQ(1, result.exit_code_);
  auto const line = trim_newline(result.stdout_);
  ASSERT_FALSE(line.empty());
  EXPECT_EQ(std::string::npos, line.find('\n'));

  auto const payload = json::parse(line).as_object();
  ASSERT_TRUE(payload.contains("status"));
  ASSERT_TRUE(payload.contains("message"));
  EXPECT_EQ("error", payload.at("status").as_string());

  auto const message = std::string{payload.at("message").as_string()};
  EXPECT_TRUE(message.starts_with("Failed to initialize MOTIS:"));
}

TEST_F(native_wrapper_regression_test, endpoint_wrapper_fault_contracts_hold) {
  auto const from = motis::native::coord{49.87336, 8.62926};
  auto const to = motis::native::coord{50.10658, 8.66178};

  motis::native::test_support::inject_fault_once("plan_route");
  EXPECT_THROW((void)motis::native::plan_route(*inst_, from, to),
               std::exception);

  motis::native::test_support::inject_fault_once("geocode");
  EXPECT_THROW((void)motis::native::geocode(*inst_, "FFM"), std::exception);

  motis::native::test_support::inject_fault_once("api_get");
  auto const result =
      motis::native::api_get(*inst_, "/api/v1/geocode?text=FFM");
  EXPECT_FALSE(result.has_value());
}
