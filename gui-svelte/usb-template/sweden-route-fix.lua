-- Sweden-specific GTFS route normalization used during import.
-- Fixes SL metro/local-rail lines that are occasionally tagged as ferry route_type.

local SL_AGENCIES = {
  ["500000000000000114"] = true,
  ["505000000000000001"] = true
}

local METRO_LINES = {
  ["10"] = { route_type = 401, clasz = 8, color = 0x005AA3 },
  ["11"] = { route_type = 401, clasz = 8, color = 0x005AA3 },
  ["13"] = { route_type = 401, clasz = 8, color = 0xE31E24 },
  ["14"] = { route_type = 401, clasz = 8, color = 0xE31E24 },
  ["17"] = { route_type = 401, clasz = 8, color = 0x009540 },
  ["18"] = { route_type = 401, clasz = 8, color = 0x009540 },
  ["19"] = { route_type = 401, clasz = 8, color = 0x009540 }
}

local RAIL_LINES = {
  ["25"] = { route_type = 2, clasz = 6, color = 0xF44336 },
  ["26"] = { route_type = 2, clasz = 6, color = 0xF44336 },
  ["27"] = { route_type = 2, clasz = 6, color = 0xF44336 },
  ["28"] = { route_type = 2, clasz = 6, color = 0xF44336 },
  ["29"] = { route_type = 2, clasz = 6, color = 0xF44336 }
}

local WHITE = 0xFFFFFF

local function starts_with(s, prefix)
  return string.sub(s, 1, string.len(prefix)) == prefix
end

local function is_sl_context(route)
  local agency = route:get_agency()
  if agency and SL_AGENCIES[agency:get_id()] then
    return true
  end

  local route_id = route:get_id() or ""
  return starts_with(route_id, "9011114")
end

function process_route(route)
  local line = route:get_short_name() or ""
  local metro = METRO_LINES[line]
  local rail = RAIL_LINES[line]
  local fix = metro or rail
  if not fix then
    return true
  end

  if not is_sl_context(route) then
    return true
  end

  local route_type = route:get_route_type()
  if route_type == 4 or route_type == 1000 or route_type == 1200 or route_type == 401 then
    route:set_route_type(fix.route_type)
    route:set_clasz(fix.clasz)
    route:set_color(fix.color)
    route:set_text_color(WHITE)
  end

  return true
end
