#include "native/api_internal.h"

#include <optional>
#include <string>

#include "boost/json.hpp"
#include "boost/url/url_view.hpp"

#include "motis/endpoints/initial.h"
#include "motis/endpoints/levels.h"
#include "motis/endpoints/stop_times.h"
#include "motis/endpoints/trip.h"
#include "motis/endpoints/one_to_all.h"
#include "motis/endpoints/one_to_many.h"
#include "motis/endpoints/routing.h"
#include "motis/endpoints/map/stops.h"
#include "motis/endpoints/map/trips.h"
#include "motis/endpoints/map/rental.h"
#include "motis/endpoints/adr/geocode.h"
#include "motis/endpoints/adr/reverse_geocode.h"

namespace motis::native {

template <typename EndpointFactory>
static std::optional<std::string> run_api_endpoint(
    bool const preconditions_ok,
    boost::urls::url_view const& url,
    EndpointFactory&& endpoint_factory) {
  if (!preconditions_ok) {
    return std::nullopt;
  }
  auto endpoint = endpoint_factory();
  return boost::json::serialize(boost::json::value_from(endpoint(url)));
}

std::optional<std::string> dispatch_api_get(
    native_instance& inst, std::string const& path_and_query) {
  auto const url = boost::urls::url_view{path_and_query};
  auto const path = std::string{url.path()};

  if (path == "/api/v1/plan" || path == "/api/v5/plan") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.l_ && inst.data_.pl_ && inst.data_.tt_ &&
            inst.data_.tags_,
        url,
        [&] {
          return motis::ep::routing{
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
              inst.data_.metrics_.get()};
        });
  }

  if (path == "/api/v1/geocode" || path == "/api/v5/geocode") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.pl_ && inst.data_.matches_ &&
            inst.data_.tt_ && inst.data_.tags_ && inst.data_.t_ &&
            inst.data_.f_ && inst.data_.tc_,
        url,
        [&] {
          return motis::ep::geocode{
              inst.data_.w_.get(),       inst.data_.pl_.get(),
              inst.data_.matches_.get(), inst.data_.tt_.get(),
              inst.data_.tags_.get(),    *inst.data_.t_, *inst.data_.f_,
              *inst.data_.tc_,           inst.data_.adr_ext_.get()};
        });
  }

  if (path == "/api/v1/reverse-geocode" || path == "/api/v5/reverse-geocode") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.pl_ && inst.data_.matches_ &&
            inst.data_.tt_ && inst.data_.tags_ && inst.data_.t_ &&
            inst.data_.f_ && inst.data_.r_,
        url,
        [&] {
          return motis::ep::reverse_geocode{
              inst.data_.w_.get(),       inst.data_.pl_.get(),
              inst.data_.matches_.get(), inst.data_.tt_.get(),
              inst.data_.tags_.get(),    *inst.data_.t_, *inst.data_.f_,
              *inst.data_.r_,            inst.data_.adr_ext_.get()};
        });
  }

  if (path == "/api/v1/map/initial") {
    return run_api_endpoint(inst.data_.tt_, url, [&] {
      return motis::ep::initial{*inst.data_.tt_, inst.config_};
    });
  }

  if (path == "/api/v1/map/levels") {
    return run_api_endpoint(inst.data_.w_ && inst.data_.l_, url, [&] {
      return motis::ep::levels{*inst.data_.w_, *inst.data_.l_};
    });
  }

  if (path == "/api/v1/stoptimes" || path == "/api/v4/stoptimes" ||
      path == "/api/v5/stoptimes") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.pl_ && inst.data_.matches_ &&
            inst.data_.tz_ && inst.data_.location_rtree_ && inst.data_.tt_ &&
            inst.data_.tags_,
        url,
        [&] {
          return motis::ep::stop_times{
              inst.config_,                inst.data_.w_.get(),
              inst.data_.pl_.get(),        inst.data_.matches_.get(),
              inst.data_.adr_ext_.get(),   inst.data_.tz_.get(),
              *inst.data_.location_rtree_, *inst.data_.tt_, *inst.data_.tags_,
              inst.data_.rt_};
        });
  }

  if (path == "/api/v1/trip" || path == "/api/v5/trip") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.l_ && inst.data_.pl_ && inst.data_.matches_ &&
            inst.data_.tt_ && inst.data_.tags_ && inst.data_.location_rtree_,
        url,
        [&] {
          return motis::ep::trip{
              inst.config_,                inst.data_.w_.get(),
              inst.data_.l_.get(),         inst.data_.pl_.get(),
              inst.data_.matches_.get(),   *inst.data_.tt_,
              inst.data_.shapes_.get(),    inst.data_.adr_ext_.get(),
              inst.data_.tz_.get(),        *inst.data_.tags_,
              *inst.data_.location_rtree_, inst.data_.rt_};
        });
  }

  if (path == "/api/v1/map/trips" || path == "/api/v4/map/trips" ||
      path == "/api/v5/map/trips") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.pl_ && inst.data_.matches_ &&
            inst.data_.tags_ && inst.data_.tt_ && inst.data_.railviz_static_,
        url,
        [&] {
          return motis::ep::trips{
              inst.data_.w_.get(),
              inst.data_.pl_.get(),
              inst.data_.matches_.get(),
              inst.data_.adr_ext_.get(),
              inst.data_.tz_.get(),
              *inst.data_.tags_,
              *inst.data_.tt_,
              inst.data_.rt_,
              inst.data_.shapes_.get(),
              *inst.data_.railviz_static_};
        });
  }

  if (path == "/api/v1/map/stops") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.pl_ && inst.data_.matches_ &&
            inst.data_.location_rtree_ && inst.data_.tags_ && inst.data_.tt_,
        url,
        [&] {
          return motis::ep::stops{
              inst.config_,
              inst.data_.w_.get(),
              inst.data_.pl_.get(),
              inst.data_.matches_.get(),
              inst.data_.adr_ext_.get(),
              inst.data_.tz_.get(),
              *inst.data_.location_rtree_,
              *inst.data_.tags_,
              *inst.data_.tt_};
        });
  }

  if (path == "/api/v1/rentals" || path == "/api/v1/map/rentals") {
    auto const query = api::rentals_params{url.params()};
    return run_api_endpoint(
        !query.point_.has_value() || (inst.data_.tt_ && inst.data_.tags_),
        url,
        [&] {
          return motis::ep::rental{
              inst.data_.gbfs_, inst.data_.tt_.get(), inst.data_.tags_.get()};
        });
  }

  if (path == "/api/v1/one-to-all" || path == "/api/experimental/one-to-all") {
    return run_api_endpoint(
        inst.data_.w_ && inst.data_.l_ && inst.data_.pl_ && inst.data_.tt_ &&
            inst.data_.tags_,
        url,
        [&] {
          return motis::ep::one_to_all{
              inst.config_,
              inst.data_.w_.get(),
              inst.data_.l_.get(),
              inst.data_.pl_.get(),
              inst.data_.elevations_.get(),
              *inst.data_.tt_,
              inst.data_.rt_,
              *inst.data_.tags_,
              inst.data_.flex_areas_.get(),
              inst.data_.location_rtree_.get(),
              inst.data_.matches_.get(),
              inst.data_.adr_ext_.get(),
              inst.data_.tz_.get(),
              inst.data_.way_matches_.get(),
              inst.data_.gbfs_,
              inst.data_.metrics_.get()};
        });
  }

  if (path == "/api/v1/one-to-many") {
    return run_api_endpoint(inst.data_.w_ && inst.data_.l_, url, [&] {
      return motis::ep::one_to_many{
          *inst.data_.w_, *inst.data_.l_, inst.data_.elevations_.get()};
    });
  }

  return std::nullopt;
}

}  // namespace motis::native
