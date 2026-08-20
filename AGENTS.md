# AGENTS.md

This file contains instructions for coding agents working in this repository.

- Repository: <https://github.com/graelo/dvoratt>
- Prefer `gh` for GitHub operations.
- Do not mention an agent or assistant in issues, pull requests, comments, or
  commit messages.
- Do not expose private local information, including machine-specific paths.

## Project

`dvoratt` is a terminal application for practicing typing on the Dvorak
keyboard layout. It embeds five progressively difficult word lists and tracks
speed, accuracy, problem words, and difficult letter combinations.

Rust 1.95 or later is required. The crate uses edition 2024.

## Architecture

- `src/lib.rs` contains the application runner and library crate wiring.
- `src/main.rs` is the thin binary entry point.
- `src/app.rs` owns session state, input handling, and final statistics.
- `src/ui.rs` renders the ratatui terminal interface.
- `src/word_lists.rs` loads the compressed lesson files from `lessons/`.
- `src/word_queue.rs` manages upcoming and problem words.
- `src/performance/` tracks WPM, accuracy, problem words, and struggle
  combinations.

The application is intentionally a TUI without command-line options. Keep
user-facing controls synchronized between the implementation, `README.md`, and
`man/dvoratt.1`.

## Verification

The `Makefile` is the canonical definition of local verification tasks. **Read
it before choosing or running verification commands**; do not duplicate its
command implementations here. `make help` lists every target.

The primary targets are:

- `make check`: pre-push gate (formatting, linting, and tests).
- `make check-all`: pre-PR gate (adds dependency, commit-message, Markdown,
  manpage, and GitHub Actions security checks).
- `make fix`: formats code and applies Clippy fixes.
- `make md`: lints Markdown against `rumdl.toml`.
- `make man`: lints the roff manpage.
- `make ci-security`: runs the Poutine and Zizmor GitHub Actions scans.

The check targets mirror the GitHub workflows and use locked dependency
resolution where applicable. They assume their external tools (for example
`cargo-nextest`, `cargo-deny`, `cargo-pants`, `convco`, `poutine`, `zizmor`,
`rumdl`, `mandoc`, and `cargo-llvm-cov`) are already installed locally.

For focused Rust tests, use `cargo nextest run <test_name>` or
`cargo nextest run <module::tests::name>`. The complete CI test sequence is
implemented in `ci/test_full.sh`; its Nextest CI profile is configured in
`.config/nextest.toml`.

## Documentation and releases

Keep user-facing documentation in sync with behavior:

- Update `README.md` and `man/dvoratt.1` when controls, lesson behavior, or
  statistics change.
- Keep crate-level rustdocs short; the README is the source of end-user
  documentation and is rendered by crates.io.
- For a release version bump, update `Cargo.toml`, `Cargo.lock`, the versioned
  section and comparison links in `CHANGELOG.md`, and the manpage `.TH` header.
  Create a `vX.Y.Z` tag; the release workflow derives release versions from it.
- Commit messages must follow `.convco` Conventional Commit rules. Use
  `make commits` to check them.

`Cargo.toml`, `Cargo.lock`, `deny.toml`, and the GitHub workflows define the
release and supply-chain constraints. Preserve `--locked` behavior in Cargo
commands that resolve dependencies.
