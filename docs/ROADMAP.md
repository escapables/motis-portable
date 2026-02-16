---
summary: 'Prioritized roadmap for the portable IPC-first app.'
read_when:
  - Planning upcoming work.
  - Tracking what ships next.
  - Deciding near-term implementation priorities.
---

# Portable App Roadmap

## Current Status (2026-02-16)

- Latest release: `v2.9.1` published on GitHub Releases.
- Native runtime remains IPC-first with no localhost fallback.
- CI includes native, UI, and linux matrix coverage for release paths.

## Immediate Priority (Next Session)

1. Triage new issues/regressions reported after `v2.9.1`.
2. Prioritize next portable-runtime improvements (startup reliability, bundle UX, offline diagnostics).
3. Keep release flow healthy by monitoring `master` CI and fixing failures quickly.
4. Capture only current-session deltas in `docs/HANDOFF.md`.

## Completed Foundations

- IPC-only native runtime model with no localhost fallback.
- UI stability fixes for tab-switch/state persistence and stale-request handling.
- Native IPC crash/restart recovery coverage and boundary hardening.
- LOC reduction refactor for large UI/native files into focused modules.
- CI improvements with stronger regression slices and Rust protocol/native coverage.

## Guardrails

- Do not reintroduce localhost/server fallback in the desktop runtime.
- Keep docs aligned with actual shipped GitHub release behavior.
