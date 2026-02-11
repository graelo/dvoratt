pub(crate) struct ProblemWordEntry {
    pub(crate) word: String,
    pub(crate) avg_speed: f32,
    pub(crate) backspaces: u32,
    pub(crate) correct_attempts: u8,
}

#[derive(Default)]
pub(crate) struct ProblemWords {
    words: Vec<ProblemWordEntry>,
}

impl ProblemWords {
    pub(crate) fn add(&mut self, word: String, speed: f32, backspace_count: u32) {
        if let Some(entry) = self.words.iter_mut().find(|e| e.word == word) {
            entry.avg_speed = (entry.avg_speed + speed) / 2.0;
            entry.backspaces = backspace_count;
            entry.correct_attempts = 0;
        } else {
            self.words.push(ProblemWordEntry {
                word,
                avg_speed: speed,
                backspaces: backspace_count,
                correct_attempts: 0,
            });
        }
    }

    pub(crate) fn update_correct_attempts(&mut self, word: &str) {
        if let Some(entry) = self.words.iter_mut().find(|e| e.word == word) {
            entry.correct_attempts += 1;
        }
    }

    pub(crate) fn remove_learned_words(&mut self) {
        self.words
            .retain(|e| e.avg_speed < 30.0 || e.correct_attempts < 2);
    }

    pub(crate) fn get_words(&self) -> &[ProblemWordEntry] {
        &self.words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_problem_words() {
        let problem_words = ProblemWords::default();
        assert!(problem_words.get_words().is_empty());
    }

    #[test]
    fn test_add_new_word() {
        let mut problem_words = ProblemWords::default();
        problem_words.add("test".to_string(), 25.0, 2);
        assert_eq!(problem_words.get_words().len(), 1);
    }

    #[test]
    fn test_add_existing_word() {
        let mut problem_words = ProblemWords::default();
        problem_words.add("test".to_string(), 25.0, 2);
        problem_words.add("test".to_string(), 30.0, 3);

        let words = problem_words.get_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0].avg_speed, 27.5);
    }

    #[test]
    fn test_update_correct_attempts() {
        let mut problem_words = ProblemWords::default();
        problem_words.add("test".to_string(), 25.0, 2);
        problem_words.update_correct_attempts("test");

        let words = problem_words.get_words();
        assert_eq!(words[0].correct_attempts, 1);
    }

    #[test]
    fn test_remove_learned_words() {
        let mut problem_words = ProblemWords::default();
        problem_words.add("fast".to_string(), 40.0, 0);
        problem_words.add("slow".to_string(), 20.0, 0);
        problem_words.update_correct_attempts("fast");
        problem_words.update_correct_attempts("fast");

        problem_words.remove_learned_words();
        let words = problem_words.get_words();
        assert_eq!(words.len(), 1);
        assert_eq!(words[0].word, "slow");
    }
}
