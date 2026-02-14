---
summary: 'Active task tracker using Task/Scope/Done-when sections.'
read_when:
  - Choosing next tasks.
  - Running pickup/handoff updates.
---

# TODO

### 1. Fresh Clone Setup
Task: Re-clone repository into a clean temp directory under `~`.
Scope:
- Perform clean checkout with no reused local build artifacts.
- Confirm clean status before running validation commands.
Done when:
- Fresh clone is ready for full build/test validation.

### 2. Clean Build Validation
Task: Run clean end-to-end build from fresh clone.
Scope:
- Execute native build and UI build paths used by portable workflow.
- Capture any setup assumptions needed to reproduce the same build.
Done when:
- Build completes successfully without ad hoc manual fixes.

### 3. Bundle Flow Verification
Task: Validate bundle startup and core flows after clean build.
Scope:
- Verify import, startup, and core routing flows in `usb-bundle`.
- Record reproducible failure details for any observed regression.
Done when:
- Core flows pass, or reproducible regressions are captured as follow-up tasks/issues.

### 4. Retest Outcome Notes
Task: Document retest outcomes and follow-up fixes.
Scope:
- Update `docs/HANDOFF.md` and linked docs with verification outcomes.
- Keep unresolved gaps and owners explicit for the next session.
Done when:
- Outcomes, unresolved gaps, and next actions are clearly documented.

### 5. Test Gate Baseline
Task: Verify and maintain local test gate baseline.
Scope:
- Run `./bin/test-gate` before handoff/push.
- Treat any gate failure as blocker until fixed or explicitly documented.
Done when:
- Gate passes on current working branch before push.

### 6. Master CI Recovery
Task: Restore CI on `master` to fully green, then push.
Scope:
- Fix remaining Linux CI failures (Ubuntu 24.04 venv/package step included).
- Push once local gate is green.
- Monitor GitHub Actions and rerun/fix until all required jobs are green.
Done when:
- Latest `master` CI run after push completes with all required jobs green.
