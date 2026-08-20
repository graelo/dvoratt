//! Dvorak typing practice application for the terminal.
//!
//! End-user documentation lives in the
//! [README](https://github.com/graelo/dvoratt#readme).

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::time::{Duration, Instant};

mod app;
mod performance;
mod ui;
mod word_lists;
mod word_queue;

use crate::app::App;

/// Run the Dvorak typing practice application.
pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    match res {
        Ok(true) => println!("{}", app.generate_final_scores()),
        Ok(false) => {}
        Err(err) => println!("{err:?}"),
    }

    Ok(())
}

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
            .unwrap_or(Duration::ZERO);

        if crossterm::event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(true);
                }
                KeyCode::Tab => {
                    let next_index = (app.current_list_index + 1) % app.word_lists.len();
                    app.change_word_list(next_index);
                }
                KeyCode::BackTab => {
                    let next_index =
                        (app.current_list_index + app.word_lists.len() - 1) % app.word_lists.len();
                    app.change_word_list(next_index);
                }
                _ => app.on_key(key.code),
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
