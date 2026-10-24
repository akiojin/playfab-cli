# Changelog

## [Unreleased]

### Changed

- Upgrade `ureq` to 3.x and migrate the HTTP client, entity token auth, and managed binary downloader to the new API
- Upgrade `sha2` to 0.11
- Bump `clap` to 4.6.1, `tokio` to 1.52.3, `serde_json` to 1.0.150, and `rustls-webpki` to 0.103.16
- Bump GitHub Actions: `actions/checkout` v6, `actions/upload-artifact` v7, `actions/download-artifact` v8, `softprops/action-gh-release` v3

## [0.1.0] - 2026-03-16

### Added

- Initial release of playfab-cli
- 200+ PlayFab API tool definitions across 13 categories
- Economy v2 Catalog and Inventory tools
- Admin API tools (~90 operations)
- Server API tools (~130 operations)
- Authentication, Profiles, and Data tools
- CloudScript and Events tools
- Groups and Progression tools
- Multiplayer tools (Matchmaking, Server Hosting, Lobby, Party)
- Experimentation tools
- HTTP client with configurable retry tiers (Strict/Standard/Bulk)
- Entity Token authentication with automatic caching and refresh
- CLI commands: tool list/schema/call, system ping, config show/set, batch, cli install/doctor
- JSON and Text output formats
- Self-update via GitHub Releases
- CI/CD with cross-compilation for 4 targets
- Claude Code plugin with 12 domain skills
