# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Updated route search controls: `📍` map-pick buttons are now inside origin/destination inputs, and the swap button is centered in the whitespace beside the two fields.
- Changed isochrones default last-leg time (`Sista sträckan`) from 15 minutes to 5 minutes when no explicit `maxPostTransitTime` is set.
- Improved tab carry-over between Trips, Departures, and Isochrones by synchronizing selected location state in both directions on tab switch.
- Improved Departures behavior for carried-over locations: when a `STOP` location is already selected, Departures now auto-resolves by stop ID without forcing manual retyping/reselection.
- Fixed repeated tab switching loss of Departures `stopId` by aligning auto-select dedupe with live `page.state.selectedStop`; added Playwright regression coverage for random Trips/Departures/Isochrones switch sequences.
- Added `bin/test-gate` to run docs validation, UI lint, UI typecheck, and UI unit tests.
- Fixed linux-debug CI coverage generation by removing non-POSIX `source` usage in GitHub Actions venv setup.
- Fixed linux-debug CI coverage pipeline by installing required tools in CI (`llvm-21-tools`, `curl`, `gnupg`) and selecting versioned coverage executables (`llvm-cov-21 gcov` / `gcov-13`) when needed.
- Added `bin/validate-docs` to validate docs metadata, `docs/README.md` index consistency, and strict `docs/TODO.md` formatting (`Task/Scope/Done when`, `## N --- DONE` support).
- Added release/session workflow docs: `docs/RELEASING.md`, `docs/HANDOFF.md`, and `docs/WORKFLOW.md`.
- Standardized front matter across legacy docs and added explicit read order in `docs/README.md`.
- Reworked `docs/TODO.md` to sectioned task format and moved status tracking responsibility to `docs/HANDOFF.md`.
- Removed unused `Scripts/docs-list.mjs` and aligned lockfile docs-list reference to `Scripts/docs-list-local.mjs`.
- Added shared `scripts/committer` helper from `~/projects/agent-scripts`.
- Replaced local `AGENTS.md` with English-only MOTIS-specific addendum based on affisch2 local layout.
