#!/bin/bash
set -Eeuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_FILE="$SCRIPT_DIR/launcher.log"
ERROR_FILE="$SCRIPT_DIR/error.txt"

SELF_TEST=0
KEEP_TMP=0
APP_ARGS=()

for arg in "$@"; do
  case "$arg" in
    --launcher-self-test)
      SELF_TEST=1
      ;;
    --launcher-keep-tmp)
      KEEP_TMP=1
      ;;
    *)
      APP_ARGS+=("$arg")
      ;;
  esac
done

LOG_ENABLED=0
if : >"$LOG_FILE" 2>/dev/null; then
  LOG_ENABLED=1
fi

ts() {
  date -u +"%Y-%m-%dT%H:%M:%SZ"
}

log() {
  local msg="$1"
  printf '[RUN.sh] %s\n' "$msg"
  if [ "$LOG_ENABLED" -eq 1 ]; then
    printf '%s [RUN.sh] %s\n' "$(ts)" "$msg" >>"$LOG_FILE"
  fi
}

fail() {
  local msg="$1"
  log "ERROR: $msg"
  printf 'ERROR: %s\n' "$msg" >&2
  exit 1
}

TMP_DIR=""
DATA_PATH="${MOTIS_DATA_PATH:-$SCRIPT_DIR/data}"
cleanup() {
  local exit_code=$?

  if [ "$exit_code" -ne 0 ]; then
    {
      echo "MOTIS launcher failed."
      echo "Date (UTC): $(ts)"
      echo "Exit code: $exit_code"
      echo "Bundle path: $SCRIPT_DIR"
      echo "Temp dir: ${TMP_DIR:-<not-created>}"
      echo "See launcher.log for details."
    } >"$ERROR_FILE" 2>/dev/null || true
  else
    rm -f "$ERROR_FILE" 2>/dev/null || true
  fi

  if [ "$KEEP_TMP" -eq 1 ]; then
    [ -n "$TMP_DIR" ] && log "Keeping temp directory: $TMP_DIR"
    return
  fi

  if [ -n "$TMP_DIR" ] && [ -d "$TMP_DIR" ]; then
    rm -rf "$TMP_DIR" 2>/dev/null || true
    log "Cleaned temp directory"
  fi
}
trap cleanup EXIT

check_exec_mount() {
  local dir="$1"
  local probe="$dir/.exec-probe.sh"

  cat >"$probe" <<'EOF'
#!/bin/sh
exit 0
EOF

  chmod 755 "$probe" || return 1
  "$probe" >/dev/null 2>&1
}

pick_tmp_dir() {
  local candidates=()
  local candidate
  local dir

  if [ -n "${XDG_RUNTIME_DIR:-}" ]; then
    candidates+=("$XDG_RUNTIME_DIR")
  fi
  candidates+=("${TMPDIR:-/tmp}" "/tmp")

  for candidate in "${candidates[@]}"; do
    [ -d "$candidate" ] || continue
    [ -w "$candidate" ] || continue

    dir="$(mktemp -d "$candidate/motis-svelte-XXXXXX" 2>/dev/null || true)"
    [ -n "$dir" ] || continue

    if check_exec_mount "$dir"; then
      TMP_DIR="$dir"
      return 0
    fi

    rm -rf "$dir" 2>/dev/null || true
  done

  return 1
}

log "MOTIS Transit (Svelte UI) - USB Launcher"
log "Bundle path: $SCRIPT_DIR"
log "Arguments: ${APP_ARGS[*]:-<none>}"

[ -f "$SCRIPT_DIR/motis-gui-svelte" ] || fail "Missing executable: $SCRIPT_DIR/motis-gui-svelte"
[ -f "$SCRIPT_DIR/motis-ipc" ] || fail "Missing executable: $SCRIPT_DIR/motis-ipc"
[ -d "$DATA_PATH" ] || fail "Data folder not found at $DATA_PATH"
[ -f "$DATA_PATH/config.yml" ] || {
  fail "Missing $DATA_PATH/config.yml (run ./motis-import.sh <gtfs.zip> <osm.pbf>)"
}

if ! pick_tmp_dir; then
  fail "Could not find an executable temp directory. Check /tmp (or XDG_RUNTIME_DIR) mount options and free space."
fi

log "Using temp directory: $TMP_DIR"

TMP_GUI="$TMP_DIR/motis-gui-svelte"
TMP_IPC="$TMP_DIR/motis-ipc"

cp -f "$SCRIPT_DIR/motis-gui-svelte" "$TMP_GUI" || fail "Failed to copy motis-gui-svelte to temp directory"
cp -f "$SCRIPT_DIR/motis-ipc" "$TMP_IPC" || fail "Failed to copy motis-ipc to temp directory"
chmod 755 "$TMP_GUI" "$TMP_IPC" || fail "Failed to set executable permissions in temp directory"

export MOTIS_DATA_PATH="$DATA_PATH"
export MOTIS_IPC_PATH="$TMP_IPC"

log "Data path: $MOTIS_DATA_PATH"
log "IPC path: $MOTIS_IPC_PATH"

if [ "$SELF_TEST" -eq 1 ]; then
  log "Launcher self-test succeeded"
  exit 0
fi

log "Starting MOTIS Transit..."
set +e
"$TMP_GUI" --data-path "$MOTIS_DATA_PATH" "${APP_ARGS[@]}"
APP_STATUS=$?
set -e

log "GUI exited with status $APP_STATUS"
exit "$APP_STATUS"
