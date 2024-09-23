use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

pub struct WordQueue {
    problem_word_queue: VecDeque<(String, u8)>,
    all_words: Vec<String>,
    current_word: String,
    next_words: Vec<String>,
}
impl WordQueue {
    pub fn new(initial_words: Vec<String>) -> Self {
        let mut all_words = initial_words;
        all_words.shuffle(&mut thread_rng());
        let current_word = all_words.pop().unwrap_or_default();
        let next_words = vec![
            all_words.pop().unwrap_or_default(),
            all_words.pop().unwrap_or_default(),
        ];

        WordQueue {
            problem_word_queue: VecDeque::new(),
            all_words,
            current_word,
            next_words,
        }
    }

    pub fn next_word(&mut self) {
        if self.next_words.is_empty() {
            self.next_words = self
                .all_words
                .split_off(self.all_words.len().saturating_sub(2));
        }

        if let Some((problem_word, attempts)) = self.problem_word_queue.pop_front() {
            self.current_word = problem_word.clone();
            if attempts < 2 {
                self.problem_word_queue
                    .push_back((problem_word, attempts + 1));
            }
        } else {
            self.current_word = self.next_words.remove(0);
        }

        while self.next_words.len() < 2 {
            if self.all_words.is_empty() {
                self.all_words.shuffle(&mut thread_rng());
            }
            self.next_words
                .push(self.all_words.pop().unwrap_or_default());
        }
    }

    pub fn add_problem_word(&mut self, word: String) {
        if let Some(index) = self.problem_word_queue.iter().position(|(w, _)| w == &word) {
            self.problem_word_queue[index].1 = 0;
        } else {
            self.problem_word_queue.push_back((word, 0));
        }
    }

    pub fn current_word(&self) -> &str {
        &self.current_word
    }

    pub fn next_words(&self) -> &[String] {
        &self.next_words
    }

    pub fn change_word_list(&mut self, new_words: Vec<String>) {
        self.all_words = new_words;
        self.all_words.shuffle(&mut thread_rng());

        self.next_words.clear();
        self.next_words = self
            .all_words
            .split_off(self.all_words.len().saturating_sub(2));
        self.current_word = self.all_words.pop().unwrap_or_default();
    }
}
