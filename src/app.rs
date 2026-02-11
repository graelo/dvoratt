use crossterm::event::KeyCode;
use std::time::Instant;

use crate::performance::PerformanceTracker;
use crate::word_lists::{load_word_lists, WordList};
use crate::word_queue::WordQueue;

pub struct App {
    pub performance: PerformanceTracker,
    pub word_queue: WordQueue,
    pub word_lists: Vec<WordList>,
    pub current_list_index: usize,
    pub user_input: String,
}

impl App {
    pub fn new() -> Self {
        let word_lists = load_word_lists();
        let word_queue = WordQueue::new(word_lists[1].words.clone());
        App {
            performance: PerformanceTracker::new(),
            word_queue,
            word_lists: word_lists.clone(),
            current_list_index: 1,
            user_input: String::new(),
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        let current_time = Instant::now();

        if self.performance.word_start_time.is_none() && key != KeyCode::Backspace {
            self.performance.word_start_time = Some(current_time);
        }

        if let Some(last_time) = self.performance.last_keypress_time {
            let duration = current_time.duration_since(last_time);
            self.performance
                .update_struggle_combinations(duration, &self.user_input);
        }

        self.performance.last_keypress_time = Some(current_time);

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
                            self.performance.mistyped_chars.push(self.user_input.len());
                        }
                    }
                    self.user_input.push(c);
                }
            }
            KeyCode::Backspace => {
                if !self.user_input.is_empty() {
                    self.user_input.pop();
                    if let Some(&last) = self.performance.mistyped_chars.last() {
                        if last == self.user_input.len() {
                            self.performance.mistyped_chars.pop();
                        }
                    }
                    self.performance.backspace_count += 1;
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
            self.update_stats();

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
        self.performance.mistyped_chars.clear();
        self.performance.backspace_count = 0;
        self.performance.word_start_time = None;
    }

    fn update_stats(&mut self) {
        if let Some(start_time) = self.performance.word_start_time {
            let elapsed = start_time.elapsed();
            self.performance.total_time += elapsed;
            self.performance.total_correct_chars += self.word_queue.current_word().len() as u32;
        }
    }

    fn add_problem_word(&mut self) {
        let speed = self.calculate_word_speed();
        let current_word = self.word_queue.current_word().to_string();
        self.performance
            .add_problem_word(current_word.clone(), speed);
        self.word_queue.add_problem_word(current_word);
    }

    fn calculate_word_speed(&self) -> f32 {
        if let Some(start_time) = self.performance.word_start_time {
            let elapsed = start_time.elapsed();
            let minutes = elapsed.as_secs_f32() / 60.0;
            (self.word_queue.current_word().len() as f32 / 5.0) / minutes
        } else {
            0.0
        }
    }

    pub fn average_speed_last_10_words(&self) -> f32 {
        self.performance.average_speed_last_10_words()
    }

    pub fn generate_final_scores(&self) -> String {
        self.performance.generate_final_scores()
    }

    pub fn on_tick(&mut self) {
        // This method can be used for periodic updates, such as updating the timer
        // or refreshing the struggle combinations list
    }

    pub fn change_word_list(&mut self, index: usize) {
        if index < self.word_lists.len() {
            self.current_list_index = index;
            let new_words = self.word_lists[index].words.clone();
            self.word_queue.change_word_list(new_words);

            self.performance.word_start_time = None;
            self.performance.backspace_count = 0;
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

        // Type the first character
        if let Some(c) = initial_word.chars().next() {
            app.on_key(KeyCode::Char(c));
            assert_eq!(app.user_input, c.to_string());
        }
    }

    #[test]
    fn test_on_key_backspace() {
        let mut app = App::new();
        let initial_word = app.word_queue.current_word().to_string();

        // Type a character then backspace
        if let Some(c) = initial_word.chars().next() {
            app.on_key(KeyCode::Char(c));
            app.on_key(KeyCode::Backspace);
            assert!(app.user_input.is_empty());
            assert_eq!(app.performance.backspace_count, 1);
        }
    }

    #[test]
    fn test_on_key_space() {
        let mut app = App::new();
        let current_word = app.word_queue.current_word().to_string();

        // Type the full word and press space
        for c in current_word.chars() {
            app.on_key(KeyCode::Char(c));
        }
        app.on_key(KeyCode::Char(' '));

        // Word should be completed and input cleared
        assert!(app.user_input.is_empty());
    }

    #[test]
    fn test_change_word_list() {
        let mut app = App::new();
        let original_word = app.word_queue.current_word().to_string();

        // Change to a different word list
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
