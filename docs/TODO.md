---
summary: 'Active task tracker using Task/Scope/Done-when sections.'
read_when:
  - Choosing next tasks.
  - Running pickup/handoff updates.
---

# TODO

## 1 --- DONE
# Task: Review codebase against project guardrails.
# Scope:
# - Compare implementation against `~/projects/agent-scripts/AGENTS.md`.
# - Review against repo docs (`docs`, `docs/STYLE.md`); create follow-up tasks.
# Done when:
# - Findings are documented and actionable follow-up items are queued.
# - Done: `docs/guardrail-review-2026-02-14.md` created with findings + queued follow-ups.

## 2 --- DONE
# Task: Add map-click workflow for route start/end selection.
# Scope:
# - Allow map click to set origin or destination directly.
# - Cover interaction regression risk with tests where fitting.
# Done when:
# - Users can set start/end from map click in stable, tested flow.
# - Done: `mapClickTarget` is bound into `SearchMask`; field-side controls toggle map-click target.

## 3 --- DONE
# Task: Move `Till`/`Från` action buttons and update iconography.
# Scope:
# - Move buttons to right side of origin/destination text fields.
# - Replace button text with `📍`.
# Done when:
# - Both fields render right-aligned pin buttons and behavior remains intact.
# - Done: top-right `from/to` map buttons removed; right-side `📍` buttons added per field.

## 4 --- DONE
# Task: Create and maintain mislabel inventory.
# Scope:
# - Catalog wrong mode/icon/line classification findings with examples.
# Done when:
# - Inventory exists with reproducible samples and expected corrections.
# - Done: `docs/mislabel-inventory.md` added with reproducible inventory entries.

## 5 --- DONE
# Task: Execute one bulk normalization pass from mislabel inventory.
# Scope:
# - Implement one coherent labeling/mode/icon normalization batch.
# - Add regression coverage where practical.
# Done when:
# - Inventory items are resolved in one reviewable fix pass.
# - Done: normalization extended for `SUBURBAN -> RAIL` and `BUS -> TRAM` (tram-text constrained), with tests.

## 6 --- DONE
# Task: Strengthen CI test-gate coverage for UI regressions.
# Scope:
# - Add UI automated checks to CI (minimum `test:unit`).
# - Add stable `test:integration` smoke coverage where feasible.
# - Document gate command flow in repo docs.
# Done when:
# - CI fails on UI regressions currently missed by build/lint/check, and gate is documented.
# - Done: CI UI job now runs unit tests + Playwright smoke; gate flow documented in `docs/WORKFLOW.md`.

## 7 --- DONE
# Task: Triage open GitHub issues relevant to this fork.
# Scope:
# - Review actionable issues in `escapables/motis-portable`.
# - Close/fix or defer with explicit rationale.
# Done when:
# - Open issues are resolved or intentionally deferred with clear notes.
# - Done: `gh issue list --state open` returns `[]` (no open issues to triage).

## 8
Task: Fix stop ID loss when switching Trips/Departures/Isochrones repeatedly.
Scope:
- Reproduce by switching tabs in random order multiple times.
- Keep selected stop/location IDs stable across tab switches.
- Add regression coverage where practical.
Done when:
- Repeated random tab switches do not clear or replace stop IDs in Departures/Isochrones.
- State stays synchronized without requiring manual re-typing/re-selection.

## 9
Task: Re-clone repository into a clean temp directory under `~`.
Scope:
- Perform clean checkout with no reused local build artifacts.
Done when:
- Fresh clone is ready for full build/test validation.

## 10
Task: Run clean end-to-end build from fresh clone.
Scope:
- Execute native build and UI build paths used by portable workflow.
Done when:
- Build completes successfully without ad hoc manual fixes.

## 11
Task: Validate bundle startup and core flows after clean build.
Scope:
- Verify import, startup, and core routing flows in `usb-bundle`.
Done when:
- Core flows pass, or reproducible regressions are captured as follow-up tasks/issues.

## 12
Task: Document retest outcomes and follow-up fixes.
Scope:
- Update `docs/HANDOFF.md` and linked docs with verification outcomes.
Done when:
- Outcomes, unresolved gaps, and next actions are clearly documented.

## 13
Task: Verify and maintain local test gate baseline.
Scope:
- Run `./bin/test-gate` before handoff/push.
- Treat any gate failure as blocker until fixed or explicitly documented.
Done when:
- Gate passes on current working branch before push.

## 14
Task: Restore CI on `master` to fully green, then push.
Scope:
- Fix remaining Linux CI failures (Ubuntu 24.04 venv/package step included).
- Push once local gate is green.
- Monitor GitHub Actions and rerun/fix until all required jobs are green.
Done when:
- Latest `master` CI run after push completes with all required jobs green.

## 15 --- DONE
# Task: EXAMPLE TODO
# Scope:
# - Demonstrate completed TODO formatting.
# - Keep this section as a reference template.
# Done when:
# - Section uses `## N --- DONE`.
# - All non-empty lines are prefixed with `#`.
