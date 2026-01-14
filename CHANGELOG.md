# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.2] - 2025-01-14

### Added

- Initial public release of pyanylist
- `AnyListClient` for authenticating with email/password or saved tokens
- Shopping list operations: create, read, update, delete lists
- List item operations: add, check off, uncheck, delete items
- Favourites management: get, add, remove favourite items
- Recipe operations: full CRUD with ingredient scaling
- Meal planning: iCalendar URL export for calendar integration
- Real-time sync via WebSocket with `RealtimeSync` and `SyncEvent`
- Token persistence with `SavedTokens` for session restoration
- Support for Python 3.12 and 3.13
- Platform support: Linux x86_64 (glibc and musl), Linux aarch64 (musl), macOS (x86_64 and aarch64)

[Unreleased]: https://github.com/ozonejunkieau/pyanylist/compare/v0.0.2...HEAD
[0.0.2]: https://github.com/ozonejunkieau/pyanylist/releases/tag/v0.0.2
