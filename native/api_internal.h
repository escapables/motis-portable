#pragma once

#include <string>

#include "native/api.h"

#include "motis/config.h"
#include "motis/data.h"

namespace motis::native {

struct native_instance {
  explicit native_instance(std::string const& data_path);

  motis::data data_;
  motis::config config_;
};

std::optional<std::string> dispatch_api_get(
    native_instance& inst, std::string const& path_and_query);

}  // namespace motis::native
