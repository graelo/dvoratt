# Architecture Overview

This document describes the architecture of Dvoratt, a terminal-based Dvorak
typing practice application.

## High-Level Structure

Dvoratt is organized as a modular Rust CLI application with clear separation of
concerns:

```text
src/
├── main.rs              # Application entry point and event loop
├── app.rs               # Core application state and logic
├── ui.rs                # Terminal user interface rendering
├── word_lists.rs        # Word list management and loading
├── word_queue.rs        # Word queue for typing practice
└── performance/         # Performance tracking modules
    ├── mod.rs           # Module declarations
    ├── performance_tracker.rs  # Main performance tracker
    ├── problem_words.rs          # Problem word identification
    ├── fastest_slowest_words.rs  # Fast/slow word tracking
    ├── struggle_combinations.rs  # Struggle combination analysis
    └── word_speed_tracker.rs     # WPM calculation
```

## Core Components

### 1. Main Application Loop (`main.rs`)

- Sets up terminal in raw mode with alternate screen buffer
- Initializes the `App` instance
- Runs the main event loop handling:
  - Terminal rendering (250ms intervals)
  - Keyboard input events
  - Cleanup on exit

### 2. Application State (`app.rs`)

The central `App` struct manages:

- **Performance tracking**: WPM, problem words, struggle combinations
- **Word queue**: Current and upcoming words for typing practice
- **Word lists**: Multiple difficulty levels (home row, full alphabet)
- **User input**: Current typed characters with error detection

Key methods:

- `on_key()`: Handles keyboard input and updates state
- `on_word_completed()`: Processes completed words and updates statistics
- `change_word_list()`: Switches between different word lists
- `generate_final_scores()`: Creates JSON summary of session metrics

### 3. User Interface (`ui.rs`)

Uses the [ratatui](https://github.com/ratatui-org/ratatui) crate for terminal
rendering.

The UI is organized into three main sections:

1. **Word List Tabs**: Top section showing available word lists with Tab
   navigation
2. **Typing Area**: Middle section displaying current/next words and user input
3. **Performance Stats**: Bottom section with four panels:
   - Problem Words (left)
   - Struggle Combinations (center-left)
   - Slowest Words (center-right, top)
   - Fastest Words (center-right, bottom)

Visual feedback includes:

- Color-coded input (red for mistyped characters)
- Yellow highlighting for current word
- Dimming for next word preview
- Problem words marked with repetition count

### 4. Word Management (`word_lists.rs` and `word_queue.rs`)

#### Word Lists Module

- Loads compressed word lists from embedded GZIP files
- Provides five difficulty levels:
  - Home Row (8 keys)
  - Home Row (10 keys)
  - Home Row + 8 keys
  - Home Row + 8 more keys
  - Full Alphabet

#### Word Queue Module

Manages the flow of words during practice sessions:

- Maintains a queue of original words
- Tracks problem words separately for repetition
- Handles word cycling and shuffling
- Supports dynamic switching between word lists
- Implements problem word repetition (up to 3 attempts)

### 5. Performance Tracking (`performance/` module)

A composite tracker that aggregates multiple specialized trackers:

#### WordSpeedTracker

- Tracks recent word speeds (last 10 words)
- Calculates average WPM for real-time feedback
- Provides smoothed speed metrics

#### ProblemWords

- Identifies problematic words based on:
  - Typing errors (mistyped characters)
  - Backspace usage
  - Slow typing speed
- Tracks statistics per word:
  - Average speed
  - Backspace count
  - Correct attempt count
- Removes "learned" words after successful attempts

#### FastestSlowestWords

- Maintains top 10 fastest and slowest typed words
- Helps users identify strengths and weaknesses
- Provides visual feedback on typing patterns

#### StruggleCombinations

- Analyzes character combinations that cause slowdowns
- Tracks time between keypresses for each combination
- Identifies finger movement challenges
- Displays top 20 problematic combinations

## Data Flow

1. **Initialization**:
   - `main.rs` creates `App` instance
   - `App::new()` loads word lists and initializes components

2. **Typing Session**:
   - User types characters → `on_key()` processes input
   - Mistyped characters tracked in `PerformanceTracker`
   - Backspaces increment counter
   - Space bar completes words → triggers statistics updates

3. **Word Completion**:
   - Word speed calculated from start time to completion
   - Updates recent word speeds
   - Adds to fastest/slowest word lists
   - Identifies problem words based on errors/backspaces
   - Moves to next word (or repeats problem word)

4. **Performance Tracking**:
   - Struggle combinations updated on each keypress
   - Problem words tracked across multiple attempts
   - Statistics displayed in real-time UI

5. **Session End**:
   - Ctrl+C triggers cleanup
   - `generate_final_scores()` creates JSON summary
   - Terminal restored to normal state

## Key Features

### Adaptive Learning

- Dynamically identifies problem words
- Repeats difficult words (up to 3 times)
- Removes mastered words from problem list
- Adjusts practice based on user performance

### Real-time Feedback

- Immediate visual feedback on typing errors
- Color-coded input display
- Current WPM calculation
- Performance metrics updated continuously

### Multiple Difficulty Levels

- Progressive word lists from home row to full alphabet
- Tab navigation between lists
- Consistent interface across all levels

### Comprehensive Statistics

- Average typing speed (WPM)
- Problem word analysis with detailed metrics
- Fastest/slowest word identification
- Struggle combination detection
- JSON export of session results

## Technical Details

### Dependencies

```toml
[dependencies]
anyhow = "1.0"           # Error handling
crossterm = "0.29.0"     # Terminal control
flate2 = "1.0.33"        # GZIP decompression
rand = "0.10.0"          # Random shuffling
ratatui = "0.30"         # Terminal UI framework
serde_json = "1.0"       # JSON serialization for final scores
```

### Architecture Patterns

- **Composite Pattern**: `PerformanceTracker` aggregates multiple specialized
  trackers
- **Strategy Pattern**: Different word lists can be swapped dynamically
- **Observer Pattern**: UI updates in response to state changes
- **MVC-like Separation**: Clear division between model (`App`), view (`UI`),
  and controller (`main.rs`)

### Error Handling

- Uses `anyhow::Result` for robust error handling
- Terminal operations wrapped in proper cleanup
- Graceful degradation on errors

## Performance Considerations

1. **Efficient Updates**:
   - Only updates necessary components on each keypress
   - Minimizes terminal redraws (250ms interval)

2. **Memory Management**:
   - Word lists loaded from compressed embedded files
   - Problem words tracked in separate queue
   - Statistics maintained with bounded collections (top 10/20 items)

3. **Real-time Responsiveness**:
   - Non-blocking event handling
   - Fast WPM calculations using precomputed values
   - Optimized data structures for quick lookups

## Testing Strategy

The codebase includes comprehensive unit tests:

- `app.rs`: Tests for application state management
- `word_queue.rs`: Tests for word queue operations
- `performance_tracker.rs`: Tests for performance metrics
- Integration tests verify component interactions

Tests cover:

- Initialization and setup
- Key handling (characters, backspace, space)
- Word completion logic
- Performance tracking accuracy
- UI rendering components
