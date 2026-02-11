use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.area());

    draw_word_list_tabs(f, app, chunks[0]);
    draw_typing_area(f, app, chunks[1], chunks[2]);
    draw_stats(f, app, chunks[3]);
}

fn draw_word_list_tabs(f: &mut Frame, app: &App, area: Rect) {
    let list_names: Vec<String> = app
        .word_lists
        .iter()
        .map(|list| list.name.clone())
        .collect();
    let tabs = Tabs::new(list_names)
        .select(app.current_list_index)
        .block(Block::default().borders(Borders::ALL).title("Word Lists"))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider("|");

    f.render_widget(tabs, area);
}

fn generate_styled_input<'a>(input: &'a str, mistyped_chars: &'a [usize]) -> Vec<Span<'a>> {
    input
        .char_indices()
        .map(|(i, c)| {
            if mistyped_chars.contains(&i) {
                Span::styled(
                    c.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw(c.to_string())
            }
        })
        .collect()
}

fn draw_typing_area(f: &mut Frame, app: &App, current_chunk: Rect, input_chunk: Rect) {
    let typing_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(current_chunk);

    let current_word = app.word_queue.current_word().to_string();
    let next_word = if app.word_queue.is_current_word_problem() {
        String::new()
    } else {
        app.word_queue
            .next_words()
            .first()
            .cloned()
            .unwrap_or_default()
    };

    let repetition_count = if app.word_queue.is_current_word_problem() {
        format!(
            " ({})",
            app.word_queue.get_current_problem_word_repetitions() + 1
        )
    } else {
        String::new()
    };

    let words_to_type = vec![
        if app.word_queue.is_current_word_problem() {
            Span::styled(
                format!("{current_word}{repetition_count}"),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            )
        } else {
            Span::styled(current_word, Style::default().fg(Color::Yellow))
        },
        Span::raw(" "),
        Span::styled(next_word, Style::default().add_modifier(Modifier::DIM)),
    ];

    let words_paragraph = Paragraph::new(Line::from(words_to_type)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Words to Type"),
    );
    f.render_widget(words_paragraph, typing_area[0]);

    let avg_speed = format!("{:.2} WPM", app.average_speed_last_10_words());
    let avg_speed_paragraph = Paragraph::new(avg_speed)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Avg Speed (Last 10 Words)"),
        );
    f.render_widget(avg_speed_paragraph, typing_area[1]);
    let styled_input = generate_styled_input(&app.user_input, &app.performance.mistyped_chars);
    let user_input = Paragraph::new(Line::from(styled_input))
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Your Input"));
    f.render_widget(user_input, input_chunk);
}

fn draw_stats(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    draw_problem_words(f, app, chunks[0]);
    draw_struggle_combinations(f, app, chunks[1]);

    let speed_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    draw_slowest_words(f, app, speed_chunks[0]);
    draw_fastest_words(f, app, speed_chunks[1]);
}

fn draw_fastest_words(f: &mut Frame, app: &App, area: Rect) {
    let fastest_words: Vec<ListItem> = app
        .performance
        .get_fastest_words()
        .iter()
        .take(10)
        .map(|(word, speed)| {
            ListItem::new(Line::from(vec![Span::raw(format!(
                "{word}: {speed:.2} WPM"
            ))]))
        })
        .collect();

    let fastest_words_list = List::new(fastest_words).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Fastest Words"),
    );
    f.render_widget(fastest_words_list, area);
}

fn draw_slowest_words(f: &mut Frame, app: &App, area: Rect) {
    let slowest_words: Vec<ListItem> = app
        .performance
        .get_slowest_words()
        .iter()
        .take(10)
        .map(|(word, speed)| {
            ListItem::new(Line::from(vec![Span::raw(format!(
                "{word}: {speed:.2} WPM"
            ))]))
        })
        .collect();

    let slowest_words_list = List::new(slowest_words).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Slowest Words"),
    );
    f.render_widget(slowest_words_list, area);
}

fn draw_problem_words(f: &mut Frame, app: &App, area: Rect) {
    let problem_words: Vec<ListItem> = app
        .performance
        .get_problem_words()
        .iter()
        .take(10)
        .map(|(word, speed, backspaces, correct_attempts)| {
            ListItem::new(Line::from(vec![Span::raw(format!(
                "{word}: {speed:.2} WPM, {backspaces} backspaces, {correct_attempts} correct"
            ))]))
        })
        .collect();

    let problem_words_list = List::new(problem_words).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Problem Words"),
    );
    f.render_widget(problem_words_list, area);
}
fn draw_struggle_combinations(f: &mut Frame, app: &App, area: Rect) {
    let struggle_combinations: Vec<ListItem> = app
        .performance
        .get_struggle_combinations()
        .iter()
        .take(20)
        .map(|(combo, speed)| {
            ListItem::new(Line::from(vec![Span::raw(format!(
                "{combo}: {speed:.2} WPM"
            ))]))
        })
        .collect();

    let struggle_combinations_list = List::new(struggle_combinations).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Struggle Combinations"),
    );
    f.render_widget(struggle_combinations_list, area);
}
