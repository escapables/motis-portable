option(NO_BUILDCACHE "Disable build caching using buildcache" Off)

set(buildcache-bin ${CMAKE_CURRENT_BINARY_DIR}/buildcache/bin/buildcache)
get_property(rule-launch-set GLOBAL PROPERTY RULE_LAUNCH_COMPILE SET)

function(motis_try_enable_buildcache candidate_path)
    if (NOT EXISTS "${candidate_path}")
        return()
    endif ()

    execute_process(
        COMMAND "${candidate_path}" --version
        RESULT_VARIABLE buildcache_result
        OUTPUT_QUIET
        ERROR_VARIABLE buildcache_error
        ERROR_STRIP_TRAILING_WHITESPACE
    )

    if (buildcache_result EQUAL 0)
        message(STATUS "Using buildcache: ${candidate_path}")
        set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE "${candidate_path}")
    else ()
        set(NO_BUILDCACHE ON CACHE BOOL "Disable build caching using buildcache" FORCE)
        message(WARNING
            "buildcache exists but is not executable (${candidate_path}). "
            "Disabling buildcache globally for this configure run. "
            "Runtime error: ${buildcache_error}")
    endif ()
endfunction()

if (NO_BUILDCACHE)
    message(STATUS "NO_BUILDCACHE set, buildcache disabled")
elseif (rule-launch-set)
    message(STATUS "Global property RULE_LAUNCH_COMPILE already set - skipping buildcache")
else ()
    find_program(buildcache_program buildcache HINTS ${CMAKE_CURRENT_BINARY_DIR}/buildcache/bin)
    if (buildcache_program)
        motis_try_enable_buildcache("${buildcache_program}")
    else ()
        message(STATUS "buildcache not found - downloading")
        if (UNIX AND ${CMAKE_HOST_SYSTEM_PROCESSOR} STREQUAL "aarch64")
            set(buildcache-archive "buildcache-linux-arm64.tar.gz")
        elseif (UNIX)
            set(buildcache-archive "buildcache-linux-amd64.tar.gz")
        else ()
            message(FATAL_ERROR "This fork supports Linux-only builds; unsupported platform for buildcache bootstrap")
        endif ()

        set(buildcache-url "https://gitlab.com/bits-n-bites/buildcache/-/releases/v0.31.5/downloads/${buildcache-archive}")
        message(STATUS "Downloading buildcache binary from ${buildcache-url}")
        file(DOWNLOAD "${buildcache-url}" ${CMAKE_CURRENT_BINARY_DIR}/${buildcache-archive})
        execute_process(
            COMMAND ${CMAKE_COMMAND} -E tar xf ${CMAKE_CURRENT_BINARY_DIR}/${buildcache-archive}
            WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}
            RESULT_VARIABLE extract_result
            ERROR_VARIABLE extract_error
            ERROR_STRIP_TRAILING_WHITESPACE
        )

        if (extract_result EQUAL 0)
            motis_try_enable_buildcache("${buildcache-bin}")
        else ()
            set(NO_BUILDCACHE ON CACHE BOOL "Disable build caching using buildcache" FORCE)
            message(WARNING
                "Failed to extract downloaded buildcache archive. "
                "Disabling buildcache globally for this configure run. "
                "Extract error: ${extract_error}")
        endif ()
    endif ()
endif ()
