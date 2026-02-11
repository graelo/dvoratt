use std::cmp::Ordering;

pub struct FastestSlowestWords {
    pub fastest_words: Vec<(String, f32)>,
    pub slowest_words: Vec<(String, f32)>,
}

impl FastestSlowestWords {
    pub fn new() -> Self {
        FastestSlowestWords {
            fastest_words: Vec::new(),
            slowest_words: Vec::new(),
        }
    }

    pub fn update(&mut self, word: &str, speed: f32) {
        self.update_fastest_words(word, speed);
        self.update_slowest_words(word, speed);
    }

    fn update_fastest_words(&mut self, word: &str, speed: f32) {
        if self.fastest_words.len() < 10 || speed > self.fastest_words.last().unwrap().1 {
            if let Some(pos) = self.fastest_words.iter().position(|(w, _)| w == word) {
                self.fastest_words.remove(pos);
            }
            self.fastest_words.push((word.to_string(), speed));
            self.fastest_words
                .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            self.fastest_words.truncate(10);
        }
    }

    fn update_slowest_words(&mut self, word: &str, speed: f32) {
        if self.slowest_words.len() < 10 || speed < self.slowest_words.last().unwrap().1 {
            if let Some(pos) = self.slowest_words.iter().position(|(w, _)| w == word) {
                self.slowest_words.remove(pos);
            }
            self.slowest_words.push((word.to_string(), speed));
            self.slowest_words
                .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
            self.slowest_words.truncate(10);
        }
    }

    pub fn get_fastest_words(&self) -> &[(String, f32)] {
        &self.fastest_words
    }

    pub fn get_slowest_words(&self) -> &[(String, f32)] {
        &self.slowest_words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fastest_slowest_words() {
        let tracker = FastestSlowestWords::new();
        assert!(tracker.get_fastest_words().is_empty());
        assert!(tracker.get_slowest_words().is_empty());
    }

    #[test]
    fn test_update_fastest_words() {
        let mut tracker = FastestSlowestWords::new();
        tracker.update("fast", 100.0);
        tracker.update("faster", 120.0);
        tracker.update("slow", 50.0);

        let fastest = tracker.get_fastest_words();
        assert_eq!(fastest.len(), 3); // All 3 words are in the top 10
        assert_eq!(fastest[0].0, "faster");
        assert_eq!(fastest[1].0, "fast");
        assert_eq!(fastest[2].0, "slow");
    }

    #[test]
    fn test_update_slowest_words() {
        let mut tracker = FastestSlowestWords::new();
        tracker.update("slow", 30.0);
        tracker.update("slower", 20.0);
        tracker.update("fast", 100.0);

        let slowest = tracker.get_slowest_words();
        assert_eq!(slowest.len(), 3); // All 3 words are in the bottom 10
        assert_eq!(slowest[0].0, "slower");
        assert_eq!(slowest[1].0, "slow");
        assert_eq!(slowest[2].0, "fast");
    }

    #[test]
    fn test_fastest_words_limit() {
        let mut tracker = FastestSlowestWords::new();
        for i in 0..15 {
            tracker.update(&format!("word{}", i), i as f32);
        }

        let fastest = tracker.get_fastest_words();
        assert_eq!(fastest.len(), 10);
        assert_eq!(fastest[0].0, "word14");
    }

    #[test]
    fn test_slowest_words_limit() {
        let mut tracker = FastestSlowestWords::new();
        for i in 0..15 {
            tracker.update(&format!("word{}", i), i as f32);
        }

        let slowest = tracker.get_slowest_words();
        assert_eq!(slowest.len(), 10);
        assert_eq!(slowest[0].0, "word0");
    }
}
