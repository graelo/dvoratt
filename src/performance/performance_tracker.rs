use super::{FastestSlowestWords, ProblemWords, StruggleCombinations, WordSpeedTracker};
use std::time::{Duration, Instant};

pub struct PerformanceTracker {
    pub word_speed_tracker: WordSpeedTracker,
    pub fastest_slowest_words: FastestSlowestWords,
    pub problem_words: ProblemWords,
    pub struggle_combinations: StruggleCombinations,
    pub last_keypress_time: Option<Instant>,
    pub word_start_time: Option<Instant>,
    pub total_time: Duration,
    pub total_correct_chars: u32,
    pub backspace_count: u32,
    pub mistyped_chars: Vec<usize>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        PerformanceTracker {
            word_speed_tracker: WordSpeedTracker::new(),
            fastest_slowest_words: FastestSlowestWords::new(),
            problem_words: ProblemWords::new(),
            struggle_combinations: StruggleCombinations::new(),
            last_keypress_time: None,
            word_start_time: None,
            total_time: Duration::new(0, 0),
            total_correct_chars: 0,
            backspace_count: 0,
            mistyped_chars: Vec::new(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_performance_tracker() {
        let tracker = PerformanceTracker::new();
        assert_eq!(tracker.total_time, Duration::new(0, 0));
        assert_eq!(tracker.total_correct_chars, 0);
        assert_eq!(tracker.backspace_count, 0);
        assert!(tracker.mistyped_chars.is_empty());
    }

    #[test]
    fn test_backspace_used() {
        let mut tracker = PerformanceTracker::new();
        assert!(!tracker.backspace_used());

        tracker.backspace_count = 1;
        assert!(tracker.backspace_used());
    }

    #[test]
    fn test_update_recent_word_speeds() {
        let mut tracker = PerformanceTracker::new();
        tracker.update_recent_word_speeds(60.0);
        tracker.update_recent_word_speeds(70.0);

        let avg = tracker.average_speed_last_10_words();
        assert!(avg > 0.0);
    }

    #[test]
    fn test_average_wpm() {
        let mut tracker = PerformanceTracker::new();
        tracker.total_time = Duration::from_secs(60); // 1 minute
        tracker.total_correct_chars = 250; // 50 words * 5 chars

        let avg_wpm = tracker.average_wpm();
        assert_eq!(avg_wpm, 50.0);
    }

    #[test]
    fn test_generate_final_scores() {
        let tracker = PerformanceTracker::new();
        let result = tracker.generate_final_scores();

        assert!(result.contains("average_speed"));
        assert!(result.contains("problem_words"));
        assert!(result.contains("fastest_words"));
        assert!(result.contains("slowest_words"));
        assert!(result.contains("struggle_combinations"));
    }
}
