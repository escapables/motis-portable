---
summary: 'Unified pickup/handoff workflow and execution checklist for cross-agent continuity.'
read_when:
  - Starting a session.
  - Handing off work.
  - Reviewing cross-agent continuity expectations.
---

# Workflow

`HANDOFF.md` is intended to be a summary of work done ephemerally per session, where one agent should be able to pick up directly from another and get proper context.

## Session Start (Pickup)

1. Run docs discovery:
```bash
pnpm run docs:list
```
2. Read `docs/HANDOFF.md`.
3. Check git state:
```bash
git status -sb
git log --oneline --decorate -n 10
```
4. Check PR/CI state:
```bash
gh pr status
gh run list --limit 10 --branch "$(git branch --show-current)"
```
5. Decide first check to run.
6. Plan next 2-3 actions, execute.

### Gate Command Flow

- Local baseline gate: `./bin/test-gate`
- CI UI gate (superset): `build` + `lint` + `check` + `test:unit` + Playwright `test:integration` smoke (`@smoke`, Chromium).
- If Playwright smoke changes, keep one backend-mocked smoke test stable and fast.

## Session End (Handoff)

1. Run full gate:
```bash
./bin/test-gate
```
2. Update docs for behavior/API changes.
3. Update `docs/HANDOFF.md` with:
- exact scope completed
- exact verification commands and outcomes
- unresolved blockers/risks
- concrete next 2-3 actions

## Release Work

When preparing a release, follow `docs/RELEASING.md`.

## Quality Bar

- Keep `docs/HANDOFF.md` current; replace stale content.
- Use concrete references (branch, SHA, run IDs, command outputs).
- Prefer short, factual bullets over narrative text.
