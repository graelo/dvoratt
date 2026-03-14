//! Tracking for word typing speeds.
//!
//! This module maintains a rolling window of the most recent word typing speeds
//! to calculate average typing speed and provide real-time feedback to users.

use std::collections::VecDeque;

/// Tracker for recent word typing speeds.
///
/// Maintains a sliding window of the 10 most recently typed words with their
/// typing speeds in WPM. Used to calculate current average typing speed.
#[derive(Default)]
pub(crate) struct WordSpeedTracker {
    /// Queue of recent word speeds (limited to 10 entries)
    recent_word_speeds: VecDeque<f32>,
}

impl WordSpeedTracker {
    pub(crate) fn update_recent_word_speeds(&mut self, speed: f32) {
        self.recent_word_speeds.push_back(speed);
        if self.recent_word_speeds.len() > 10 {
            self.recent_word_speeds.pop_front();
        }
    }

    pub(crate) fn average_speed_last_10_words(&self) -> f32 {
        if self.recent_word_speeds.is_empty() {
            0.0
        } else {
            self.recent_word_speeds.iter().sum::<f32>() / self.recent_word_speeds.len() as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_word_speed_tracker() {
        let tracker = WordSpeedTracker::default();
        assert_eq!(tracker.average_speed_last_10_words(), 0.0);
    }

    #[test]
    fn test_update_recent_word_speeds() {
        let mut tracker = WordSpeedTracker::default();
        tracker.update_recent_word_speeds(60.0);
        tracker.update_recent_word_speeds(70.0);
        assert_eq!(tracker.average_speed_last_10_words(), 65.0);
    }

    #[test]
    fn test_average_speed_last_10_words() {
        let mut tracker = WordSpeedTracker::default();
        assert_eq!(tracker.average_speed_last_10_words(), 0.0);

        tracker.update_recent_word_speeds(60.0);
        tracker.update_recent_word_speeds(70.0);
        assert_eq!(tracker.average_speed_last_10_words(), 65.0);
    }

    #[test]
    fn test_average_speed_overflow() {
        let mut tracker = WordSpeedTracker::default();
        for i in 0..15 {
            tracker.update_recent_word_speeds(i as f32);
        }
        // Average of last 10 values (5..=14)
        let expected_avg = (5.0 + 6.0 + 7.0 + 8.0 + 9.0 + 10.0 + 11.0 + 12.0 + 13.0 + 14.0) / 10.0;
        assert_eq!(tracker.average_speed_last_10_words(), expected_avg);
    }
}
