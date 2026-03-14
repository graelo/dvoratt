# Dvorak Typing Practice in the Terminal

A terminal-based application for practicing and improving your
Dvorak keyboard layout typing skills.

## Features

- **Interactive typing practice** with real-time feedback
- **Multiple word lists** for different difficulty levels
- **Performance tracking** including:
  - Words per minute (WPM) calculation
  - Problem words identification
  - Fastest and slowest typed words
  - Struggle combinations analysis
- **Visual feedback** with color-coded input (mistyped characters in red)
- **Problem word repetition** to help master difficult words
- **Tab navigation** between different word lists

## Installation

### Prerequisites

- Rust (1.88 or later)
- A terminal that supports ANSI escape codes

### Building from Source

```bash
# Clone the repository
git clone https://github.com/graelo/dvoratt.git
cd dvoratt

# Build and install
cargo install --path .
```

### Running

```bash
# Run the application
dvoratt

# Exit the application
Press Ctrl+C
```

## Usage

### Basic Typing Practice

1. Start the application with `dvoratt`
2. Type the displayed word character by character
3. Press Space to submit your input
4. The next word will automatically appear
5. Mistyped characters are shown in red for visual feedback

### Navigating Word Lists

- **Tab**: Cycle forward through available word lists
- **Shift+Tab**: Cycle backward through available word lists

### Performance Metrics

The application displays several performance metrics:

- **Avg Speed (Last 10 Words)**: Your current typing speed in WPM
- **Problem Words**: Words you've struggled with, showing average
  speed and backspace usage
- **Fastest Words**: Your 10 fastest-typed words
- **Slowest Words**: Your 10 slowest-typed words
- **Struggle Combinations**: Character combinations that slow you down

### Final Scores

When you exit the application (Ctrl+C), it displays a JSON summary
of your session including:

- Average typing speed
- Problem words with statistics
- Fastest and slowest words
- Struggle combinations

## Word Lists

The application includes multiple word lists of varying difficulty:

- **Easy**: Short, common words
- **Medium**: Medium-length words
- **Hard**: Longer, less common words
- **Problem Words**: Dynamically generated based on your mistakes during the
    session

## Configuration

No configuration file is currently required. The application uses default
settings.

## Development

### Building

```bash
# Build in debug mode (default)
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy for linting
cargo clippy
```

### Code Structure

- `src/main.rs`: Main entry point and application loop
- `src/app.rs`: Core application logic and state management
- `src/ui.rs`: Terminal user interface rendering
- `src/word_lists.rs`: Word list definitions and loading
- `src/word_queue.rs`: Word queue management for typing practice
- `src/performance/`: Performance tracking modules:
  - `performance_tracker.rs`: Main performance tracking
  - `problem_words.rs`: Problem word identification and management
  - `fastest_slowest_words.rs`: Tracking fastest and slowest words
  - `struggle_combinations.rs`: Identifying character combinations that cause
      slowdowns
  - `word_speed_tracker.rs`: WPM calculation and tracking

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for
guidelines.

## License

This project is licensed under the MIT License. See the
[LICENSE](LICENSE) file for details.

## Release Notes

See [CHANGELOG.md](CHANGELOG.md) for version history and release notes.
