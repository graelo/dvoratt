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
