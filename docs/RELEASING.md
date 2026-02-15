---
summary: 'Release checklist for changelog, test gate, build verification, tagging, and GitHub release quality.'
read_when:
  - Preparing a release.
  - Tagging and publishing a release.
  - Curating release notes.
---

# Releasing

## Versioning

Use Semantic Versioning:
- `MAJOR`: breaking behavior/API changes.
- `MINOR`: new features and improvements.
- `PATCH`: bug fixes, docs/build fixes.

Tag format: `vX.Y.Z`.

## Changelog

Release notes source is `CHANGELOG.md`.

Before release:
1. Curate `## Unreleased` with user-visible changes only.
2. Move those bullets into `## [X.Y.Z] - YYYY-MM-DD`.
3. Keep a fresh empty `## Unreleased` section at top.

## Release Policy

- Distribution channel: GitHub Releases only.
- Release gate: remediation/review process finished and CI green on `master`.

## Release Checklist

1. Confirm latest CI status on `master` is green:
```bash
gh run list --workflow CI --branch master --limit 5
```

2. Run local gate:
```bash
./bin/test-gate
```

3. Build USB bundle:
```bash
./gui-svelte/build-usb.sh
```

4. Smoke check bundle startup:
```bash
cd usb-bundle
./RUN.sh
```

5. Verify expected bundle artifacts:
- `usb-bundle/motis`
- `usb-bundle/motis-ipc`
- `usb-bundle/motis-gui-svelte`
- `usb-bundle/ui/`
- `usb-bundle/RUN.sh`
- `usb-bundle/motis-import.sh`

6. Commit release metadata:
```bash
git add CHANGELOG.md docs/RELEASING.md docs/HANDOFF.md
git commit -m "chore: release vX.Y.Z"
```

7. Tag:
```bash
git tag vX.Y.Z
```

8. Push branch + tag (when approved):
```bash
git push origin master
git push origin vX.Y.Z
```

9. Publish GitHub release (triggers CI release asset upload):
```bash
gh release create vX.Y.Z --verify-tag --title "MOTIS Portable vX.Y.Z" --notes-file /tmp/release-notes.md
```

## Post-Release

- Update `docs/HANDOFF.md` with release SHA/tag plus gate/build outcomes.
- Verify CI release run completed and attached linux artifact to release:
```bash
gh run list --workflow CI --limit 5
gh release view vX.Y.Z
```
- Confirm tag points to intended commit:
```bash
git show --no-patch --decorate vX.Y.Z
```

## GitHub Release Guardrails

- Title format: `<Project> <version>` (not version-only).
- Release body: curated changelog bullets for that version, verbatim.
- Attach all shipping artifacts expected by downstream users.
- Verify tag, title, body, and assets after publish; fix mismatches immediately.
