# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.1.5] - 2026-04-20

### Security

- Update rand 0.10.0 → 0.10.1 (RUSTSEC-2026-0097: unsound aliased mutable
  reference in ThreadRng)

### Changed

- Bump MSRV to 1.95 and edition to 2024
- Switch test runner from cargo test to cargo-nextest
- Harden all GitHub Actions workflows: SHA-pinned actions, least-privilege
  permissions, persist-credentials: false, cache poisoning guard, template
  injection fix, tag filter tightening
- Replace ncipollo/release-action with gh CLI
- Add build provenance attestation (Sigstore)
- Switch homebrew job from PAT to GitHub App token
- Add reusable supply chain workflows (cargo-audit, ci-security) with
  conditional and scheduled callers
- Add Renovate with daily runs, SHA pinning, and automerge for patch/minor

### Added

- `.github/workflows/cargo-audit.yml` reusable dependency audit
- `.github/workflows/ci-security.yml` reusable zizmor + poutine scans
- `.github/workflows/supply-chain-schedule.yml` scheduled caller
- `.github/workflows/renovate.yml` daily Renovate via GitHub App
- `.github/zizmor.yml` and `.github/poutine.yml` tool configuration
- `.github/CODEOWNERS`
- `.config/nextest.toml` with CI profile
- `renovate.json`

## [v0.1.4] - 2026-03-23

### Changed

- Bump GitHub Actions versions
- Replace cargo-audit CI step with cargo-pants
- Bump dependencies

### Added

- Comprehensive documentation for all modules and public items

### Fixed

- Replace placeholder dates in CHANGELOG with actual release dates

## [v0.1.3] - 2026-03-09

### Fixed

- Improved clippy configuration in CI workflows
- Removed outdated rustsec configuration from deny.toml

### Other

- Code formatting improvements
- Test fixes for private field access and non-deterministic tests

## [v0.1.2] - 2026-02-12

### Fixed

- Collapsed nested if statements into match arm guards for better code clarity

## [v0.1.1] - 2025-11-24

### Improved

- Enhanced code quality and idiomatic Rust patterns throughout the codebase

### Fixed

- Resolved void tests and non-deterministic test issues

## [v0.1.0] - 2025-02-05

### Added

- Initial release of Dvorak Typing Practice application
- Terminal-based UI using ratatui
- Multiple word lists for different difficulty levels
- Performance tracking with WPM calculation
- Problem word detection and automatic retry
- Struggle combination tracking
- Fastest/slowest word statistics
