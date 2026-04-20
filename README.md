# Dvorak Typing Practice in the Terminal

A terminal-based typing practice application designed specifically for the
Dvorak keyboard layout. This tool helps users improve their typing speed and
accuracy by providing interactive practice sessions with performance tracking.

## Features

- **Interactive Terminal UI**: Full-screen terminal interface using ratatui
    (formerly tui-rs)
- **Multiple Word Lists**: Different difficulty levels to challenge your skills
- **Performance Tracking**: Real-time statistics including WPM, accuracy, and
    problem words
- **Struggle Detection**: Identifies words you frequently mistype or struggle
    with
- **Problem Word Practice**: Automatically adds difficult words back into the
    queue for extra practice
- **Keyboard Navigation**: Use Tab/Shift+Tab to switch between word lists
- **Exit on Ctrl+C**: Clean exit handling with final statistics display

## Installation

### Prerequisites

- Rust (1.95 or later)
- A terminal that supports ANSI escape codes

### Building from Source

```bash
# Clone the repository
git clone https://github.com/graelo/dvoratt.git
cd dvoratt

# Build and install
cargo install --path .
```

Or run directly:

```bash
cargo run
```

## Usage

### Basic Practice Session

1. Run the application: `dvoratt` or `cargo run`
2. Start typing the displayed word
3. Press Space to submit your answer
4. The next word will appear automatically
5. Press Ctrl+C to exit and see your final statistics

### Switching Word Lists

- **Tab**: Cycle forward through available word lists
- **Shift+Tab**: Cycle backward through available word lists

Each word list has a different difficulty level, allowing you to progressively
challenge yourself.

## Performance Metrics

Upon exiting (Ctrl+C), the application displays:

- **Average WPM**: Words per minute over your session
- **Accuracy**: Percentage of correct keystrokes
- **Words Typed**: Total count of words completed
- **Problem Words**: Words you struggled with or mistyped frequently
- **Fastest/Slowest Words**: Your best and worst performances

## Configuration

The application uses default word lists included in the repository. No external
configuration is currently required.

## Development

### Running Tests

```bash
cargo test
```

### Code Quality Checks

```bash
# Run clippy for linting
cargo clippy --all-targets --all-features

# Check for security vulnerabilities
cargo deny check

# Format code
cargo fmt
```

## Architecture

The application is structured into several modules:

- **`app.rs`**: Main application state and logic
- **`ui.rs`**: Terminal user interface rendering
- **`word_lists.rs`**: Word list definitions for different difficulty levels
- **`word_queue.rs`**: Manages the queue of words to type, including problem
    word handling
- **`performance/`**: Performance tracking and statistics (fastest/slowest
    words, struggle detection, etc.)

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Any character | Type the current word |
| Space | Submit the current word |
| Backspace | Delete the last character |
| Tab | Next word list |
| Shift+Tab | Previous word list |
| Ctrl+C | Exit application and show statistics |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.
