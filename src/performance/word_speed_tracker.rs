use std::collections::VecDeque;

pub struct WordSpeedTracker {
    pub recent_word_speeds: VecDeque<f32>,
}

impl WordSpeedTracker {
    pub fn new() -> Self {
        WordSpeedTracker {
            recent_word_speeds: VecDeque::new(),
        }
    }

    pub fn update_recent_word_speeds(&mut self, speed: f32) {
        self.recent_word_speeds.push_back(speed);
        if self.recent_word_speeds.len() > 10 {
            self.recent_word_speeds.pop_front();
        }
    }

    pub fn average_speed_last_10_words(&self) -> f32 {
        if self.recent_word_speeds.is_empty() {
            0.0
        } else {
            self.recent_word_speeds.iter().sum::<f32>() / self.recent_word_speeds.len() as f32
        }
    }
}
