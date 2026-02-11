//! Application state and core logic for the Dvorak typing practice.
//!
//! This module contains the main `App` struct that manages the application state,
//! including the word queue, performance tracking, and user input handling.

use crossterm::event::KeyCode;
use std::time::Instant;

use crate::performance::PerformanceTracker;
use crate::word_lists::{load_word_lists, WordList};
use crate::word_queue::WordQueue;

/// The main application state.
///
/// This struct contains all the state needed to run the typing practice application,
/// including the current word queue, performance metrics, available word lists,
/// and user input state.
pub(crate) struct App {
    /// Performance tracking and statistics
    pub(crate) performance: PerformanceTracker,
    /// Queue of words to type, including problem words
    pub(crate) word_queue: WordQueue,
    /// Available word lists for different difficulty levels
    pub(crate) word_lists: Vec<WordList>,
    /// Index of the currently selected word list
    pub(crate) current_list_index: usize,
    /// Current user input for the word being typed
    pub(crate) user_input: String,
}

impl App {
    /// Create a new `App` instance with default settings.
    pub(crate) fn new() -> Self {
        let word_lists = load_word_lists();
        let word_queue = WordQueue::new(word_lists[1].words.clone());
        App {
            performance: PerformanceTracker::default(),
            word_queue,
            word_lists: word_lists.clone(),
            current_list_index: 1,
            user_input: String::new(),
        }
    }

    pub(crate) fn on_key(&mut self, key: KeyCode) {
        let current_time = Instant::now();

        if key != KeyCode::Backspace {
            self.performance.start_word_if_needed(current_time);
        }

        if let Some(last_time) = self.performance.last_keypress_time() {
            let duration = current_time.duration_since(last_time);
            self.performance
                .update_struggle_combinations(duration, &self.user_input);
        }

        self.performance.set_last_keypress_time(current_time);

        match key {
            KeyCode::Char(c) => {
                if c == ' ' {
                    self.on_word_completed();
                } else {
                    let current_word = self.word_queue.current_word();
                    if self.user_input.len() < current_word.len() {
                        let expected_char =
                            current_word.chars().nth(self.user_input.len()).unwrap();
                        if c != expected_char {
                            self.performance.record_mistype(self.user_input.len());
                        }
                    }
                    self.user_input.push(c);
                }
            }
            KeyCode::Backspace => {
                if !self.user_input.is_empty() {
                    self.user_input.pop();
                    self.performance.undo_mistype_at(self.user_input.len());
                    self.performance.record_backspace();
                    self.add_problem_word();
                }
            }
            _ => {}
        }
    }

    fn on_word_completed(&mut self) {
        if self.user_input == self.word_queue.current_word() {
            let speed = self.calculate_word_speed();
            self.performance.update_recent_word_speeds(speed);
            let user_input_clone = self.user_input.clone();
            self.performance
                .update_fastest_slowest_words(&user_input_clone, speed);
            self.performance
                .record_word_completed(self.word_queue.current_word().len() as u32);

            if self.word_queue.is_current_word_problem() {
                self.word_queue.update_problem_word_correct_attempt();
                if self.word_queue.get_current_problem_word_repetitions() >= 3 {
                    self.performance
                        .update_problem_word_correct_attempts(self.word_queue.current_word());
                }
            } else if self.performance.backspace_used() {
                self.add_problem_word();
            } else {
                self.performance
                    .update_problem_word_correct_attempts(self.word_queue.current_word());
            }

            self.performance.remove_learned_words();
            self.word_queue.next_word();
        } else {
            self.add_problem_word();
        }
        self.user_input.clear();
        self.performance.reset_word_state();
    }

    fn add_problem_word(&mut self) {
        let speed = self.calculate_word_speed();
        let current_word = self.word_queue.current_word().to_string();
        self.performance
            .add_problem_word(current_word.clone(), speed);
        self.word_queue.add_problem_word(current_word);
    }

    fn calculate_word_speed(&self) -> f32 {
        if let Some(start_time) = self.performance.word_start_time() {
            let elapsed = start_time.elapsed();
            let minutes = elapsed.as_secs_f32() / 60.0;
            (self.word_queue.current_word().len() as f32 / 5.0) / minutes
        } else {
            0.0
        }
    }

    pub(crate) fn average_speed_last_10_words(&self) -> f32 {
        self.performance.average_speed_last_10_words()
    }

    pub(crate) fn generate_final_scores(&self) -> String {
        self.performance.generate_final_scores()
    }

    pub(crate) fn on_tick(&mut self) {
        // This method can be used for periodic updates, such as updating the timer
        // or refreshing the struggle combinations list
    }

    pub(crate) fn change_word_list(&mut self, index: usize) {
        if index < self.word_lists.len() {
            self.current_list_index = index;
            let new_words = self.word_lists[index].words.clone();
            self.word_queue.change_word_list(new_words);
            self.performance.reset_word_state();
            self.user_input.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(!app.word_lists.is_empty());
        assert_eq!(app.current_list_index, 1);
        assert!(app.user_input.is_empty());
        assert!(!app.word_queue.current_word().is_empty());
    }

    #[test]
    fn test_on_key_char() {
        let mut app = App::new();
        let initial_word = app.word_queue.current_word().to_string();

        if let Some(c) = initial_word.chars().next() {
            app.on_key(KeyCode::Char(c));
            assert_eq!(app.user_input, c.to_string());
        }
    }

    #[test]
    fn test_on_key_backspace() {
        let mut app = App::new();
        let initial_word = app.word_queue.current_word().to_string();

        if let Some(c) = initial_word.chars().next() {
            app.on_key(KeyCode::Char(c));
            app.on_key(KeyCode::Backspace);
            assert!(app.user_input.is_empty());
            assert!(app.performance.backspace_used());
        }
    }

    #[test]
    fn test_on_key_space() {
        let mut app = App::new();
        let current_word = app.word_queue.current_word().to_string();

        for c in current_word.chars() {
            app.on_key(KeyCode::Char(c));
        }
        app.on_key(KeyCode::Char(' '));

        assert!(app.user_input.is_empty());
    }

    #[test]
    fn test_change_word_list() {
        let mut app = App::new();
        let original_word = app.word_queue.current_word().to_string();

        if app.word_lists.len() > 1 {
            app.change_word_list(0);
            assert_eq!(app.current_list_index, 0);
            assert_ne!(app.word_queue.current_word(), original_word);
            assert!(app.user_input.is_empty());
        }
    }

    #[test]
    fn test_average_speed_last_10_words() {
        let app = App::new();
        let speed = app.average_speed_last_10_words();
        assert!(speed >= 0.0);
    }

    #[test]
    fn test_generate_final_scores() {
        let app = App::new();
        let scores = app.generate_final_scores();
        assert!(!scores.is_empty());
        assert!(scores.contains("average_speed"));
    }
}
