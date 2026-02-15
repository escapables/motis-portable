mod commands;
mod ipc;
mod types;

pub use commands::{
    api_get_sync, geocode, geocode_sync, get_glyph_sync, get_tile_sync, plan_route,
    plan_route_sync, reverse_geocode, reverse_geocode_sync,
};
pub use ipc::{
    auto_init, destroy, get_mode, get_startup_diagnostics, init_ipc, is_ipc_initialized,
    try_auto_init, BackendMode,
};
pub use types::{Area, LatLon, LocationResult, Match, MatchArea, RouteLeg, RouteResult, Token};

#[cfg(test)]
mod tests {
    use super::commands::build_geocode_command;
    use super::ipc::{
        init_ipc, recover_ipc_backend, send_ipc_json_command, validate_ipc_command,
        MAX_IPC_COMMAND_BYTES,
    };
    #[cfg(unix)]
    use super::ipc::paths::create_private_temp_dir;
    use super::destroy;
    #[cfg(unix)]
    use std::fs;
    use std::io;
    #[cfg(unix)]
    use std::path::{Path, PathBuf};
    #[cfg(unix)]
    use std::time::{SystemTime, UNIX_EPOCH};

    #[cfg(unix)]
    fn unique_test_dir(prefix: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "{}-{}-{}",
            prefix,
            std::process::id(),
            timestamp
        ))
    }

    #[cfg(unix)]
    fn write_recovery_test_backend(script_path: &Path) {
        use std::os::unix::fs::PermissionsExt;

        let script = r#"#!/bin/sh
set -eu
data_path="${1:-}"
state_file="$data_path/.recovery_state"
if [ ! -f "$state_file" ]; then
  echo "first" > "$state_file"
  first_run=1
else
  first_run=0
fi
if IFS= read -r _line; then
  if [ "$first_run" -eq 1 ]; then
    exit 42
  fi
  printf '{"status":"ok","data":{"recovered":true}}\n'
fi
"#;

        fs::write(script_path, script).expect("write fake ipc backend");
        fs::set_permissions(script_path, fs::Permissions::from_mode(0o755))
            .expect("chmod fake ipc backend");
    }

    #[test]
    fn validate_ipc_command_rejects_newlines() {
        let err = validate_ipc_command("{\"cmd\":\"geocode\"}\n{\"cmd\":\"api_get\"}")
            .expect_err("newline command rejected");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn validate_ipc_command_rejects_oversized_payloads() {
        let payload = "x".repeat(MAX_IPC_COMMAND_BYTES + 1);
        let err = validate_ipc_command(&payload).expect_err("oversized command rejected");
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn geocode_command_serializes_control_chars_safely() {
        let query = "line1\nline2\\\"quoted\"";
        let cmd = build_geocode_command(query);
        assert!(!cmd.contains('\n'));

        let parsed: serde_json::Value = serde_json::from_str(&cmd).expect("parse geocode cmd");
        assert_eq!(parsed["cmd"], "geocode");
        assert_eq!(parsed["query"], query);
    }

    #[cfg(unix)]
    #[test]
    fn secure_temp_dir_is_private() {
        use std::os::unix::fs::PermissionsExt;

        let dir = create_private_temp_dir("motis-ipc-test").expect("create secure temp dir");
        let mode = std::fs::metadata(&dir).expect("dir metadata").permissions().mode();
        assert_eq!(mode & 0o077, 0, "group/other permissions must be removed");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn destroy_is_idempotent_without_backend() {
        destroy();
        destroy();
    }

    #[test]
    fn recovery_without_launch_config_returns_false() {
        destroy();
        let recovered = recover_ipc_backend("unit-test").expect("recover call");
        assert!(!recovered);
    }

    #[cfg(unix)]
    #[test]
    fn command_retries_after_ipc_crash_and_recovers() {
        destroy();

        let root = unique_test_dir("motis-ipc-recovery");
        let data_dir = root.join("data");
        let ipc_script = root.join("fake-ipc.sh");
        fs::create_dir_all(&data_dir).expect("create test data dir");
        fs::write(data_dir.join("config.yml"), "dataset: test\n").expect("write config.yml");
        write_recovery_test_backend(&ipc_script);

        let ipc_path = ipc_script.to_str().expect("ipc script path");
        let data_path = data_dir.to_str().expect("data dir path");
        init_ipc(ipc_path, data_path).expect("init fake backend");

        let response =
            send_ipc_json_command(r#"{"cmd":"health_check"}"#).expect("command recovers");
        assert_eq!(response["recovered"], true);

        destroy();
        let _ = fs::remove_dir_all(root);
    }
}
