---
summary: 'Contribution guidelines including quality gates and PR workflow.'
read_when:
  - Preparing your first contribution.
  - Opening a pull request.
  - Understanding project conventions.
---

# Contributing

Thank you for your interest in contributing! This guide covers how to contribute effectively.

## Quick Start

1. Fork and clone the repository:
```bash
git clone https://github.com/escapables/motis-portable.git
cd motis-portable
```

2. Build the project (see `docs/linux-dev-setup.md` for dependencies):
```bash
cmake --preset=linux-x86_64-release
cmake --build build/linux-x86_64-release
```

3. Run tests:
```bash
bin/test-gate
```

4. Make your changes and test locally.

## Pull Request Workflow

### Before Submitting

- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] No linter warnings
- [ ] Code formatted
- [ ] Documentation updated (if behavior changes)
- [ ] Commit messages are clear and descriptive

### Opening a PR

1. Create a feature branch:
```bash
git checkout -b feature/my-feature
```

2. Make commits with clear messages:
```bash
git commit -m "feat: add new feature"
```

3. Push and open PR:
```bash
git push origin feature/my-feature
```

4. In your PR description:
   - Explain what changed and why
   - Reference any related issues
   - Note any breaking changes

### PR Review Process

- All PRs require at least one review
- Address review feedback promptly
- CI must pass before merge
- Maintainers may squash commits on merge

## Local Quality Gates

Run these before pushing:

```bash
bin/test-gate
```

## Code Style

- Follow C++ conventions in `docs/STYLE.md`
- Run formatters and fix all warnings
- Keep functions focused and small
- Use meaningful variable names

## Documentation

If your change affects user-facing behavior:

- Update `README.md` for user-facing changes
- Update `docs/PRIMARY_TODO.md` if architecture changes
- Add/update examples if applicable

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in existing issues
- Check `docs/` for architecture and setup information

## License

By contributing, you agree that your contributions will be licensed under the project's license.
