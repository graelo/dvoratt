use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::time::{Duration, Instant};

mod app;
mod performance;
mod ui;
mod word_lists;
mod word_queue;

use crate::app::App;

/// Main entry point for the Dvorak typing practice application.
///
/// This function sets up the terminal, initializes the application, runs the main loop,
/// and handles cleanup. It returns a `Result` indicating success or failure.
///
/// # Errors
///
/// This function can return errors from:
/// - Terminal setup/teardown operations
/// - Terminal drawing operations
/// - Event handling operations
fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(quit) = res {
        if quit {
            println!("{}", app.generate_final_scores());
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

/// Main application loop that handles terminal drawing and user input.
///
/// This function runs continuously, drawing the UI and processing key events
/// until the user presses Ctrl+C to quit. It returns `true` when the user
/// quits normally, or propagates any errors that occur.
///
/// # Type Parameters
///
/// * `B`: The terminal backend type
///
/// # Returns
///
/// Returns `Ok(true)` when the user quits the application normally.
/// Returns `Err` if any terminal or drawing errors occur.
fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool>
where
    B::Error: 'static + Send + Sync,
{
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal
            .draw(|f| ui::draw(f, app))
            .map_err(anyhow::Error::new)?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true)
                    }
                    KeyCode::Tab => {
                        let next_index = (app.current_list_index + 1) % app.word_lists.len();
                        app.change_word_list(next_index);
                    }
                    KeyCode::BackTab => {
                        let next_index = (app.current_list_index + app.word_lists.len() - 1)
                            % app.word_lists.len();
                        app.change_word_list(next_index);
                    }
                    _ => app.on_key(key.code),
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
