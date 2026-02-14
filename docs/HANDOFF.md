---
summary: 'Ephemeral per-session handoff state for cross-agent pickup continuity.'
read_when:
  - Starting work (`/pickup`).
  - Ending a work session.
  - Taking over from another agent.
---

# HANDOFF

## Session

- Updated: `2026-02-14 10:56 UTC`
- Agent: `codex`
- Branch: `master`
- HEAD: `7953dc98`
- Scope: UI routing/tab-sync fixes, isochrones defaults tuning, repeated USB bundle rebuilds for local validation.

## Completed

- CI workflow Linux venv install fix landed in working tree (`.github/workflows/ci.yml`): switched to `python3-venv` install path on Ubuntu 24.04.
- Routing UI control layout updates:
  - Pin buttons moved inside origin/destination inputs and right-anchored.
  - Swap button moved into right-side whitespace, centered between inputs.
  - Files: `ui/src/lib/SearchMask.svelte`, `ui/src/lib/AddressTypeahead.svelte`.
- Isochrones default tuning:
  - `maxPostTransitTime` default in isochrones context reduced to `5 * 60`.
  - File: `ui/src/routes/+page.svelte`.
- Cross-tab location/state sync updates:
  - Shared `from` between Trips and Departures.
  - Tab switch sync between Trips and Isochrones (`from <-> one`) fixed for both directions with previous-tab guard.
  - Departures auto-select by carried `STOP` id when tab becomes active; dedupe key avoids repeat requests.
  - Files: `ui/src/routes/+page.svelte`, `ui/src/lib/DeparturesMask.svelte`.
- TODO updated with new pending task for random tab-switch stop-id loss:
  - Added as next pending item in `docs/TODO.md` (`## 8`) and renumbered subsequent sections.
- Clean-clone verification completed in `/home/dator/tmp/motis-portable-clean-20260214-095914`:
  - Native build completed using `CMAKE_BUILD_PARALLEL_LEVEL=2` (OOM-safe).
  - USB bundle build completed.
- Workspace USB bundle rebuilt multiple times; latest successful build at `11:52 UTC`.

## Verification Run

- `pnpm --dir ui run check` -> pass (multiple reruns; 0 errors, 0 warnings).
- `PLAYWRIGHT_BROWSERS_PATH=/tmp/ms-playwright pnpm --dir ui run test:integration:smoke` -> pass (multiple reruns; `1 passed`).
- `./bin/validate-docs` -> pass (including TODO format/index checks).
- `CMAKE_BUILD_PARALLEL_LEVEL=2 cmake --build build --target motis motis-ipc` in clean clone -> pass.
- `./gui-svelte/build-usb.sh` in workspace -> pass; bundle at `/home/dator/projects/motis-portable/usb-bundle` (latest timestamp `11:52 UTC`).

## Open Risks / Blockers

- Working tree is intentionally dirty (pre-existing + current session edits); no new commit created in this session.
- Random repeated tab switching stop-id persistence needs user-facing stress confirmation in desktop runtime despite logic fixes.
- CI status for current working tree is not re-validated post-push because no push was requested in this session.

## Next Actions

- Run manual stress test in `usb-bundle` by rapidly switching Trips/Departures/Isochrones and confirming stop IDs persist in both Departures and Isochrones.
- If persistence bug still appears, add deterministic regression coverage for tab-switch state sync.
- Before commit/push, run full local gate (`./bin/test-gate`) from current working tree.
- Commit/push only on explicit user instruction.
