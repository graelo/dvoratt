# Contributing to Dvoratt

Thank you for considering contributing to Dvoratt! This document
provides guidelines and instructions for contributing to the project.

## How to Contribute

### Reporting Issues

Before reporting an issue, please check:

1. The [issue tracker](https://github.com/graelo/dvoratt/issues) to see if it's
    already been reported
2. The documentation in this repository
3. Your local environment configuration

When reporting a bug, please include:

- A clear description of the issue
- Steps to reproduce the problem
- Your operating system and terminal emulator
- The version of Rust you're using (`rustc --version`)
- Any relevant error messages or logs

### Suggesting Features

Feature requests are welcome! Please provide:

- A clear description of the feature
- Why it would be useful
- Any potential implementation approaches

### Submitting Pull Requests

1. Fork the repository and create your branch from `main`
2. Make your changes following the project conventions
3. Ensure all tests pass (`cargo test`)
4. Format your code (`cargo fmt`)
5. Run clippy (`cargo clippy -- -D warnings`)
6. Commit your changes with descriptive messages
7. Push to your fork and submit a pull request

## Development Setup

### Prerequisites

- Rust 1.88 or later
- Git
- A terminal that supports ANSI escape codes

### Building and Testing

```bash
# Clone the repository
git clone https://github.com/graelo/dvoratt.git
cd dvoratt

# Build in debug mode
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy for linting
cargo clippy -- -D warnings

# Build documentation
cargo doc --open
```

## Code Style

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `snake_case` for variables and functions
- Use `CamelCase` for types and enums
- Use `UPPER_CASE` for constants
- Keep lines under 100 characters where reasonable
- Add appropriate documentation comments for all public items

### Documentation Style

- Use Rustdoc comments (`///`) for public items
- First sentence should be a complete summary in imperative mood
- Include examples where helpful
- Document errors that can be returned
- Use Markdown formatting appropriately

## Testing

### Writing Tests

- Place unit tests in the same file as the code being tested, in a `tests`
    module
- Place integration tests in the `tests` directory
- Ensure tests are deterministic and fast-running
- Test edge cases and error conditions

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with verbose output
cargo test -- --nocapture

# Run tests showing output
cargo test -- --show-output
```

## Architecture

### High-Level Overview

The application follows a modular architecture:

1. **Main Loop** (`src/main.rs`): Handles terminal setup, event loop, and
    cleanup
2. **Application State** (`src/app.rs`): Manages the core state and logic
3. **User Interface** (`src/ui.rs`): Renders the terminal UI using ratatui
4. **Performance Tracking** (`src/performance/`): Tracks typing metrics and
    identifies problem areas
5. **Word Management** (`src/word_lists.rs`, `src/word_queue.rs`):
   Handles word lists and queue

### Key Components

- **App**: Main application state struct that coordinates between components
- **PerformanceTracker**: Tracks WPM, problem words, struggle combinations, etc.
- **WordQueue**: Manages the queue of words to type
- **UI Renderer**: Uses ratatui to create an interactive terminal interface

## Documentation

### Updating Documentation

When making changes that affect:

- User-facing behavior: Update README.md
- Version history: Update CHANGELOG.md
- Build or development process: Update CONTRIBUTING.md
- Public APIs: Update Rustdoc comments

### Generating Documentation

```bash
# Generate HTML documentation
cargo doc --open

# Generate documentation for a specific crate
cargo doc --package dvoratt --open
```

## Release Process

The release process is automated through GitHub Actions. To create a new
release:

1. Update the version in `Cargo.toml`
2. Update CHANGELOG.md with release notes
3. Create and push a tag:

    ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
    ```

4. The GitHub Actions workflow will automatically build and
   publish the release

## License

By contributing to Dvoratt, you agree that your contributions
will be licensed under the MIT License.

## Questions?

If you have any questions about contributing, please open an issue or contact
the maintainers.
