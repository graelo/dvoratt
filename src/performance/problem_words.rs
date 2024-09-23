pub struct ProblemWords {
    pub words: Vec<(String, f32, u32, u8)>,
}

impl ProblemWords {
    pub fn new() -> Self {
        ProblemWords { words: Vec::new() }
    }

    pub fn add(&mut self, word: String, speed: f32, backspace_count: u32) {
        if let Some(index) = self.words.iter().position(|(w, _, _, _)| w == &word) {
            let (_, avg_speed, backspaces, correct_attempts) = &mut self.words[index];
            *avg_speed = (*avg_speed + speed) / 2.0;
            *backspaces = backspace_count;
            *correct_attempts = 0;
        } else {
            self.words.push((word, speed, backspace_count, 0));
        }
    }

    pub fn update_correct_attempts(&mut self, word: &str) {
        if let Some(index) = self.words.iter().position(|(w, _, _, _)| w == word) {
            self.words[index].3 += 1;
        }
    }

    pub fn remove_learned_words(&mut self) {
        self.words
            .retain(|(_, speed, _, correct_attempts)| *speed < 30.0 || *correct_attempts < 2);
    }

    pub fn get_words(&self) -> &[(String, f32, u32, u8)] {
        &self.words
    }
}
