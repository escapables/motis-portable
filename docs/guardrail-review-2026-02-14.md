---
summary: 'Guardrail review snapshot with findings and queued follow-ups for the current run.'
read_when:
  - Validating compliance with project guardrails.
  - Starting implementation chain from TODO step 1.
---

# Guardrail Review 2026-02-14

## Findings

- Native runtime guardrail intact: no localhost HTTP fallback in active UI runtime flow; protocol base URL remains `motis://localhost` (`ui/src/routes/+layout.ts`).
- CI reliability gap confirmed: linux job failed on Ubuntu 24.04 due to `python3.12-venv` package mismatch; patched to `python3-venv` in workflow.
- UI interaction gap addressed in this run: map-click origin/destination controls moved next to fields for direct workflow visibility.
- Mode/icon mislabel quality work was partially present; inventory + bulk normalization pass executed in this run.

## Follow-up Queue

- `docs/TODO.md` step `6`: extend CI coverage with explicit UI regression checks.
- `docs/TODO.md` step `7`: triage any issue fallout after normalization pass.
- `docs/TODO.md` steps `8-10`: run clean-clone build and portable bundle validation.
- `docs/TODO.md` step `11`: document retest outcomes in handoff docs.
- `docs/TODO.md` steps `12-13`: run local gate, then push and drive CI to green.
