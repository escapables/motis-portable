---
summary: 'Prioritized roadmap and release-readiness status for the portable IPC-first app.'
read_when:
  - Planning upcoming work.
  - Tracking release readiness before publishing on GitHub.
  - Deciding what remains before push/tag.
---

# Portable App Roadmap

## Current Status (2026-02-15)

- Core remediation/refactor chain is completed across UI, native IPC/runtime, protocol handling, and CI regression coverage.
- Release path is GitHub-only and tied to review/remediation completion plus green CI.
- Remaining release blocker is execution evidence from the full clean-build verification matrix.

## Immediate Priority (Next Session)

1. Run clean debug configure/build from a fresh build directory.
2. Execute all required verification scenarios listed in `docs/PRIMARY_TODO.md` and tracked in `docs/TODO.md`.
3. Record exact command outcomes and any residual blockers in `docs/HANDOFF.md`.
4. If all scenarios pass, proceed with push/tag/release checklist from `docs/RELEASING.md`.

## Completed Foundations

- IPC-only native runtime model with no localhost fallback.
- UI stability fixes for tab-switch/state persistence and stale-request handling.
- Native IPC crash/restart recovery coverage and boundary hardening.
- LOC reduction refactor for large UI/native files into focused modules.
- CI improvements with stronger regression slices and Rust protocol/native coverage.

## Guardrails

- Do not reintroduce localhost/server fallback in the desktop runtime.
- Do not skip clean-build verification evidence before release.
- Keep docs aligned with actual shipped GitHub release behavior.
