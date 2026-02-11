use super::fastest_slowest_words::FastestSlowestWords;
use super::problem_words::{ProblemWordEntry, ProblemWords};
use super::struggle_combinations::StruggleCombinations;
use super::word_speed_tracker::WordSpeedTracker;
use std::time::{Duration, Instant};

pub(crate) struct PerformanceTracker {
    word_speed_tracker: WordSpeedTracker,
    fastest_slowest_words: FastestSlowestWords,
    problem_words: ProblemWords,
    struggle_combinations: StruggleCombinations,
    last_keypress_time: Option<Instant>,
    word_start_time: Option<Instant>,
    total_time: Duration,
    total_correct_chars: u32,
    backspace_count: u32,
    mistyped_chars: Vec<usize>,
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        PerformanceTracker {
            word_speed_tracker: WordSpeedTracker::default(),
            fastest_slowest_words: FastestSlowestWords::default(),
            problem_words: ProblemWords::default(),
            struggle_combinations: StruggleCombinations::default(),
            last_keypress_time: None,
            word_start_time: None,
            total_time: Duration::ZERO,
            total_correct_chars: 0,
            backspace_count: 0,
            mistyped_chars: Vec::new(),
        }
    }
}

impl PerformanceTracker {
    // --- Keypress tracking ---

    pub(crate) fn update_struggle_combinations(&mut self, duration: Duration, user_input: &str) {
        self.struggle_combinations.update(duration, user_input);
    }

    pub(crate) fn last_keypress_time(&self) -> Option<Instant> {
        self.last_keypress_time
    }

    pub(crate) fn set_last_keypress_time(&mut self, time: Instant) {
        self.last_keypress_time = Some(time);
    }

    // --- Word start timing ---

    pub(crate) fn start_word_if_needed(&mut self, time: Instant) {
        if self.word_start_time.is_none() {
            self.word_start_time = Some(time);
        }
    }

    pub(crate) fn word_start_time(&self) -> Option<Instant> {
        self.word_start_time
    }

    // --- Mistyped characters ---

    pub(crate) fn record_mistype(&mut self, pos: usize) {
        self.mistyped_chars.push(pos);
    }

    pub(crate) fn undo_mistype_at(&mut self, pos: usize) {
        if let Some(&last) = self.mistyped_chars.last() {
            if last == pos {
                self.mistyped_chars.pop();
            }
        }
    }

    pub(crate) fn mistyped_chars(&self) -> &[usize] {
        &self.mistyped_chars
    }

    // --- Backspace ---

    pub(crate) fn record_backspace(&mut self) {
        self.backspace_count += 1;
    }

    pub(crate) fn backspace_used(&self) -> bool {
        self.backspace_count > 0
    }

    // --- Word completion ---

    pub(crate) fn record_word_completed(&mut self, word_len: u32) {
        if let Some(start_time) = self.word_start_time {
            self.total_time += start_time.elapsed();
            self.total_correct_chars += word_len;
        }
    }

    pub(crate) fn reset_word_state(&mut self) {
        self.mistyped_chars.clear();
        self.backspace_count = 0;
        self.word_start_time = None;
    }

    // --- Speed tracking ---

    pub(crate) fn update_recent_word_speeds(&mut self, speed: f32) {
        self.word_speed_tracker.update_recent_word_speeds(speed);
    }

    pub(crate) fn average_speed_last_10_words(&self) -> f32 {
        self.word_speed_tracker.average_speed_last_10_words()
    }

    pub(crate) fn update_fastest_slowest_words(&mut self, word: &str, speed: f32) {
        self.fastest_slowest_words.update(word, speed);
    }

    // --- Problem words ---

    pub(crate) fn add_problem_word(&mut self, word: String, speed: f32) {
        self.problem_words.add(word, speed, self.backspace_count);
    }

    pub(crate) fn update_problem_word_correct_attempts(&mut self, word: &str) {
        self.problem_words.update_correct_attempts(word);
    }

    pub(crate) fn remove_learned_words(&mut self) {
        self.problem_words.remove_learned_words();
    }

    // --- Aggregate stats ---

    pub(crate) fn average_wpm(&self) -> f32 {
        let minutes = self.total_time.as_secs_f32() / 60.0;
        if minutes == 0.0 {
            return 0.0;
        }
        (self.total_correct_chars as f32 / 5.0) / minutes
    }

    pub(crate) fn get_fastest_words(&self) -> &[(String, f32)] {
        self.fastest_slowest_words.get_fastest_words()
    }

    pub(crate) fn get_slowest_words(&self) -> &[(String, f32)] {
        self.fastest_slowest_words.get_slowest_words()
    }

    pub(crate) fn get_problem_words(&self) -> &[ProblemWordEntry] {
        self.problem_words.get_words()
    }

    pub(crate) fn get_struggle_combinations(&self) -> &[(String, f32)] {
        self.struggle_combinations.get_combinations()
    }

    pub(crate) fn generate_final_scores(&self) -> String {
        let json = serde_json::json!({
            "average_speed": self.average_wpm(),
            "problem_words": self.get_problem_words().iter().map(|e| {
                serde_json::json!({
                    "word": e.word,
                    "speed": e.avg_speed,
                    "backspaces": e.backspaces,
                    "correct_attempts": e.correct_attempts
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
        let tracker = PerformanceTracker::default();
        assert!(!tracker.backspace_used());
        assert!(tracker.mistyped_chars().is_empty());
    }

    #[test]
    fn test_backspace_used() {
        let mut tracker = PerformanceTracker::default();
        assert!(!tracker.backspace_used());

        tracker.record_backspace();
        assert!(tracker.backspace_used());
    }

    #[test]
    fn test_update_recent_word_speeds() {
        let mut tracker = PerformanceTracker::default();
        tracker.update_recent_word_speeds(60.0);
        tracker.update_recent_word_speeds(70.0);

        let avg = tracker.average_speed_last_10_words();
        assert!(avg > 0.0);
    }

    #[test]
    fn test_average_wpm() {
        let tracker = PerformanceTracker {
            total_time: Duration::from_secs(60),
            total_correct_chars: 250,
            ..Default::default()
        };

        let avg_wpm = tracker.average_wpm();
        assert_eq!(avg_wpm, 50.0);
    }

    #[test]
    fn test_average_wpm_zero_time() {
        let tracker = PerformanceTracker::default();
        assert_eq!(tracker.average_wpm(), 0.0);
    }

    #[test]
    fn test_generate_final_scores() {
        let tracker = PerformanceTracker::default();
        let result = tracker.generate_final_scores();

        assert!(result.contains("average_speed"));
        assert!(result.contains("problem_words"));
        assert!(result.contains("fastest_words"));
        assert!(result.contains("slowest_words"));
        assert!(result.contains("struggle_combinations"));
    }
}
