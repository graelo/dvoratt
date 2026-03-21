# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
