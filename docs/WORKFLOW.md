---
summary: 'Unified pickup/handoff workflow and execution checklist for cross-agent continuity.'
read_when:
  - Starting a session.
  - Handing off work.
  - Reviewing cross-agent continuity expectations.
---

# Workflow

`HANDOFF.md` — short continuity snapshot for cross-agent pickup.

## Handoff Contract

- Max **2 TODO items** per coding session before requesting approval.
- `HANDOFF.md` under **60 lines** (incl. front matter).
- Replace stale content; no historical accumulation.
- Section shape: `Session`, `Completed`, `Verification Run`, `Open Risks / Blockers`, `Next Actions`.
- `Completed`: short session deltas only (max 4 bullets).
- `Verification Run`: concrete command + result.
- `Next Actions`: 2–3 bullets, concrete commands/checks.

## Milestone Completion

All TODO items DONE → reviewer's turn:
- Evaluate deliverables
- Propose next items/milestone in `PRIMARY_TODO.md` + `TODO.md`
- Update `HANDOFF.md` with new directives

## TODO Lifecycle

Finishing item — collapse to single comment line:
- `// ### DONE 3. Harden Security`
- Delete Task/Scope/Done-when block entirely
- Never renumber

Adding item:
- Append with next sequential number
- Never renumber existing
