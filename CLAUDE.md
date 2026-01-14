# Claude Code Instructions

This file contains instructions for Claude Code when working on this project.

## Project Overview

pyanylist is a Python binding for the AnyList API, built using Rust and PyO3/maturin. The Rust code in `src/lib.rs` wraps the `anylist_rs` crate and exposes it to Python.

## Development Commands

```bash
# Install dependencies
uv sync

# Build the Rust extension in development mode
uv run maturin develop

# Run tests (excluding integration tests)
uv run pytest -v -m "not integration"

# Run all tests (requires ANYLIST_EMAIL and ANYLIST_PASSWORD env vars)
uv run pytest -v

# Lint
uv run ruff check tests/
uv run ruff format tests/

# Type check
uv run pyright tests/
```

## Updating the Changelog

When making changes to this project, update `CHANGELOG.md` following the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format:

1. **Add entries under `[Unreleased]`** for all notable changes
2. **Use these section headers** (only include sections with changes):
   - `### Added` - new features
   - `### Changed` - changes to existing functionality
   - `### Deprecated` - features that will be removed
   - `### Removed` - removed features
   - `### Fixed` - bug fixes
   - `### Security` - security fixes
3. **Write entries in imperative mood**: "Add feature" not "Added feature"
4. **Include relevant context**: mention affected classes/functions

### Example entry

```markdown
## [Unreleased]

### Added

- Add `get_item_by_id()` method to `AnyListClient` for fetching individual items

### Fixed

- Fix `cross_off_item()` not syncing immediately in some cases
```

### On release

When preparing a release:

1. Move `[Unreleased]` content to a new version section with the release date
2. Update version in `pyproject.toml` and `Cargo.toml` (must match)
3. Add comparison links at the bottom of the file

## Releasing to PyPI

Releases are automated via GitHub Actions when a GitHub Release is published:

1. Update versions in `pyproject.toml` and `Cargo.toml`
2. Update `CHANGELOG.md` with the new version and date
3. Commit and push to main
4. Create a git tag: `git tag -a vX.Y.Z -m "Release X.Y.Z"`
5. Push the tag: `git push origin vX.Y.Z`
6. Create a GitHub Release from the tag

The release workflow builds wheels for all supported platforms and publishes to PyPI using trusted publishing.

## Architecture Notes

- **src/lib.rs**: All Python bindings - wraps `anylist_rs` types for PyO3
- **tests/**: Python tests for the bindings
- **Cargo.toml**: Rust dependencies - `anylist_rs` is fetched from git
- **pyproject.toml**: Python package metadata and tool configuration
