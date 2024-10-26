use super::{FastestSlowestWords, ProblemWords, StruggleCombinations, WordSpeedTracker};
use std::time::{Duration, Instant};

pub struct PerformanceTracker {
    pub word_speed_tracker: WordSpeedTracker,
    pub fastest_slowest_words: FastestSlowestWords,
    pub problem_words: ProblemWords,
    pub struggle_combinations: StruggleCombinations,
    pub last_keypress_time: Option<Instant>,
    pub total_time: Duration,
    pub total_correct_chars: u32,
    pub backspace_count: u32,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        PerformanceTracker {
            word_speed_tracker: WordSpeedTracker::new(),
            fastest_slowest_words: FastestSlowestWords::new(),
            problem_words: ProblemWords::new(),
            struggle_combinations: StruggleCombinations::new(),
            last_keypress_time: None,
            total_time: Duration::new(0, 0),
            total_correct_chars: 0,
            backspace_count: 0,
        }
    }

    pub fn backspace_used(&self) -> bool {
        self.backspace_count > 0
    }

    pub fn update_recent_word_speeds(&mut self, speed: f32) {
        self.word_speed_tracker.update_recent_word_speeds(speed);
    }

    pub fn average_speed_last_10_words(&self) -> f32 {
        self.word_speed_tracker.average_speed_last_10_words()
    }

    pub fn update_fastest_slowest_words(&mut self, word: &str, speed: f32) {
        self.fastest_slowest_words.update(word, speed);
    }

    pub fn add_problem_word(&mut self, word: String, speed: f32) {
        self.problem_words.add(word, speed, self.backspace_count);
    }

    pub fn update_problem_word_correct_attempts(&mut self, word: &str) {
        self.problem_words.update_correct_attempts(word);
    }

    pub fn remove_learned_words(&mut self) {
        self.problem_words.remove_learned_words();
    }

    pub fn update_struggle_combinations(&mut self, duration: Duration, user_input: &str) {
        self.struggle_combinations.update(duration, user_input);
    }

    pub fn average_wpm(&self) -> f32 {
        let minutes = self.total_time.as_secs_f32() / 60.0;
        (self.total_correct_chars as f32 / 5.0) / minutes
    }

    pub fn get_fastest_words(&self) -> &[(String, f32)] {
        self.fastest_slowest_words.get_fastest_words()
    }

    pub fn get_slowest_words(&self) -> &[(String, f32)] {
        self.fastest_slowest_words.get_slowest_words()
    }

    pub fn get_problem_words(&self) -> &[(String, f32, u32, u8)] {
        self.problem_words.get_words()
    }

    pub fn get_struggle_combinations(&self) -> &[(String, f32)] {
        self.struggle_combinations.get_combinations()
    }

    pub fn generate_final_scores(&self) -> String {
        let json = serde_json::json!({
            "average_speed": self.average_wpm(),
            "problem_words": self.get_problem_words().iter().map(|(word, speed, backspaces, correct_attempts)| {
                serde_json::json!({
                    "word": word,
                    "speed": speed,
                    "backspaces": backspaces,
                    "correct_attempts": correct_attempts
                })
            }).collect::<Vec<_>>(),
            "fastest_words": self.get_fastest_words().iter().map(|(word, speed)| {
                serde_json::json!({
                    "word": word,
                    "speed": speed
                })
            }).collect::<Vec<_>>(),
            "slowest_words": self.get_slowest_words().iter().map(|(word, speed)| {
                serde_json::json!({
                    "word": word,
                    "speed": speed
                })
            }).collect::<Vec<_>>(),
            "struggle_combinations": self.get_struggle_combinations().iter().map(|(combo, speed)| {
                serde_json::json!({
                    "combination": combo,
                    "speed": speed
                })
            }).collect::<Vec<_>>()
        });

        serde_json::to_string_pretty(&json).unwrap_or_else(|_| "{}".to_string())
    }
}
