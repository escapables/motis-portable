---
summary: 'Ephemeral per-session handoff state for cross-agent pickup continuity.'
read_when:
  - Starting work (`/pickup`).
  - Ending a work session.
  - Taking over from another agent.
---

# HANDOFF

## Session

- Updated: `2026-02-14 11:23 UTC`
- Agent: `codex`
- Branch: `master`
- HEAD: `979eb769`
- Scope: Fixed repeated tab-switch stop-id loss in Departures, added regression coverage, rebuilt USB bundle, and synced docs.

## Completed

- Fixed root cause for repeated tab-switch stop-id loss:
  - Removed stale local dedupe key from Departures auto-select logic.
  - Auto-select now compares against `page.state.selectedStop` key (`stopId|time`) before deciding to skip.
  - File: `ui/src/lib/DeparturesMask.svelte`.
- Added deterministic integration regression:
  - New Playwright test `@regression preserves departures stopId across repeated tab switches`.
  - Mocks `initial`, `stoptimes`, and `one-to-all`; drives random-like tab switch sequence; asserts `stopId` remains stable.
  - File: `ui/tests/test.ts`.
- Updated task tracking and release notes:
  - `docs/TODO.md` item `## 8` marked `DONE`.
  - `CHANGELOG.md` `Unreleased` updated with stop-id persistence fix and regression test note.
- Rebuilt portable runtime bundle:
  - `./gui-svelte/build-usb.sh` completed successfully.
  - Fresh bundle artifacts stamped `12:21` in `/home/dator/projects/motis-portable/usb-bundle`.
- User-reported manual validation: “Problem solved, works perfectly.”

## Verification Run

- `pnpm --dir ui run lint` -> pass.
- `pnpm --dir ui run check` -> pass (`0 errors`, `0 warnings`).
- `PLAYWRIGHT_BROWSERS_PATH=/tmp/ms-playwright pnpm --dir ui exec playwright test tests/test.ts --workers=1 --grep "@regression"` -> pass (`1 passed`).
- `PLAYWRIGHT_BROWSERS_PATH=/tmp/ms-playwright pnpm --dir ui run test:integration:smoke` -> pass (`1 passed`).
- `./gui-svelte/build-usb.sh` -> pass; bundle refreshed in `/home/dator/projects/motis-portable/usb-bundle` (artifact timestamp `12:21`).

## Open Risks / Blockers

- Working tree intentionally dirty (not committed): `ui/src/lib/DeparturesMask.svelte`, `ui/tests/test.ts`, `docs/TODO.md`, `docs/HANDOFF.md`, `CHANGELOG.md`.
- CI was intentionally skipped in this session per user request.
- Full project gate `./bin/test-gate` has not been run after this fix chain.

## Next Actions

- Run `./bin/test-gate` before handoff/commit to satisfy local gate policy.
- Commit docs + UI regression fix (Conventional Commit) when approved.
- Optionally run CI follow-up (`gh run list/view`) once user wants remote status again.
