use rand::rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

pub(crate) struct WordQueue {
    problem_word_queue: VecDeque<(String, u8)>,
    original_words: Vec<String>,
    all_words: Vec<String>,
    current_word: String,
    next_words: Vec<String>,
    is_repeating_problem_word: bool,
    problem_word_repetitions: u8,
}

impl WordQueue {
    pub(crate) fn is_current_word_problem(&self) -> bool {
        self.is_repeating_problem_word
    }

    pub(crate) fn get_current_problem_word_repetitions(&self) -> u8 {
        self.problem_word_repetitions
    }

    pub(crate) fn new(initial_words: Vec<String>) -> Self {
        let original_words = initial_words.clone();
        let mut all_words = initial_words;
        all_words.shuffle(&mut rng());
        let current_word = all_words.pop().unwrap_or_default();
        let next_words = vec![
            all_words.pop().unwrap_or_default(),
            all_words.pop().unwrap_or_default(),
        ];

        WordQueue {
            problem_word_queue: VecDeque::new(),
            original_words,
            all_words,
            current_word,
            next_words,
            is_repeating_problem_word: false,
            problem_word_repetitions: 0,
        }
    }

    pub(crate) fn next_word(&mut self) {
        if self.is_repeating_problem_word {
            if self.problem_word_repetitions >= 3 {
                self.is_repeating_problem_word = false;
                self.problem_word_repetitions = 0;
                self.problem_word_queue.pop_front();
            } else {
                return;
            }
        }

        if self.next_words.is_empty() {
            self.next_words = self
                .all_words
                .split_off(self.all_words.len().saturating_sub(2));
        }

        if let Some((problem_word, _)) = self.problem_word_queue.front() {
            self.current_word = problem_word.clone();
            self.is_repeating_problem_word = true;
            self.problem_word_repetitions = 0;
        } else {
            self.current_word = self.next_words.remove(0);
        }

        while self.next_words.len() < 2 {
            if self.all_words.is_empty() {
                self.all_words = self.original_words.clone();
                self.all_words.shuffle(&mut rng());
            }
            self.next_words
                .push(self.all_words.pop().unwrap_or_default());
        }
    }

    pub(crate) fn add_problem_word(&mut self, word: String) {
        if let Some(index) = self.problem_word_queue.iter().position(|(w, _)| w == &word) {
            self.problem_word_queue[index].1 = 0;
        } else {
            self.problem_word_queue.push_back((word, 0));
        }
        self.is_repeating_problem_word = true;
        self.problem_word_repetitions = 0;
    }

    pub(crate) fn update_problem_word_correct_attempt(&mut self) {
        if self.is_repeating_problem_word {
            self.problem_word_repetitions += 1;
        }
    }

    pub(crate) fn current_word(&self) -> &str {
        &self.current_word
    }

    pub(crate) fn next_words(&self) -> &[String] {
        &self.next_words
    }

    pub(crate) fn change_word_list(&mut self, new_words: Vec<String>) {
        self.original_words = new_words.clone();
        self.all_words = new_words;
        self.all_words.shuffle(&mut rng());

        self.next_words.clear();
        self.next_words = self
            .all_words
            .split_off(self.all_words.len().saturating_sub(2));
        self.current_word = self.all_words.pop().unwrap_or_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_word_queue() {
        let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let queue = WordQueue::new(words);
        assert!(!queue.current_word().is_empty());
        assert_eq!(queue.next_words().len(), 2);
    }

    #[test]
    fn test_next_word() {
        let words = vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
            "fourth".to_string(),
        ];
        let mut queue = WordQueue::new(words);
        let first_word = queue.current_word().to_string();

        queue.next_word();
        let second_word = queue.current_word().to_string();

        assert_ne!(first_word, second_word);
        assert!(!second_word.is_empty());
    }

    #[test]
    fn test_add_problem_word() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut queue = WordQueue::new(words);

        queue.add_problem_word("problem".to_string());
        assert!(queue.is_current_word_problem());
        assert_eq!(queue.get_current_problem_word_repetitions(), 0);
    }

    #[test]
    fn test_update_problem_word_correct_attempt() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut queue = WordQueue::new(words);

        queue.add_problem_word("problem".to_string());
        queue.update_problem_word_correct_attempt();
        assert_eq!(queue.get_current_problem_word_repetitions(), 1);
    }

    #[test]
    fn test_change_word_list() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let mut queue = WordQueue::new(words);

        let new_words = vec!["rust".to_string(), "test".to_string(), "code".to_string()];
        queue.change_word_list(new_words);

        assert_ne!(queue.current_word(), "hello");
        assert_ne!(queue.current_word(), "world");
    }
}
