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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_problem_words() {
        let problem_words = ProblemWords::new();
        assert!(problem_words.get_words().is_empty());
    }

    #[test]
    fn test_add_new_word() {
        let mut problem_words = ProblemWords::new();
        problem_words.add("test".to_string(), 25.0, 2);
        assert_eq!(problem_words.get_words().len(), 1);
    }

    #[test]
    fn test_add_existing_word() {
        let mut problem_words = ProblemWords::new();
        problem_words.add("test".to_string(), 25.0, 2);
        problem_words.add("test".to_string(), 30.0, 3);

        let words = problem_words.get_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0].1, 27.5); // Average of 25.0 and 30.0
    }

    #[test]
    fn test_update_correct_attempts() {
        let mut problem_words = ProblemWords::new();
        problem_words.add("test".to_string(), 25.0, 2);
        problem_words.update_correct_attempts("test");

        let words = problem_words.get_words();
        assert_eq!(words[0].3, 1);
    }

    #[test]
    fn test_remove_learned_words() {
        let mut problem_words = ProblemWords::new();
        problem_words.add("fast".to_string(), 40.0, 0); // Should be removed (speed >= 30.0 AND correct_attempts >= 2)
        problem_words.add("slow".to_string(), 20.0, 0); // Should be kept (speed < 30.0)
        problem_words.update_correct_attempts("fast");
        problem_words.update_correct_attempts("fast"); // 2 correct attempts

        problem_words.remove_learned_words();
        let words = problem_words.get_words();
        assert_eq!(words.len(), 1); // Only "slow" remains
        assert_eq!(words[0].0, "slow");
    }
}
