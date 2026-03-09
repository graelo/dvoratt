use std::time::Duration;

#[derive(Default)]
pub(crate) struct StruggleCombinations {
    combinations: Vec<(String, f32)>,
}

impl StruggleCombinations {
    pub(crate) fn update(&mut self, duration: Duration, user_input: &str) {
        let combos = self.get_letter_combinations(user_input);
        for combo in combos {
            let speed = self.calculate_combo_speed(&combo, duration);
            if let Some(index) = self.combinations.iter().position(|(c, _)| c == &combo) {
                let (_, avg_speed) = &mut self.combinations[index];
                *avg_speed = (*avg_speed + speed) / 2.0;
            } else {
                self.combinations.push((combo, speed));
            }
        }
        self.combinations
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        self.combinations.truncate(50);
    }

    fn get_letter_combinations(&self, user_input: &str) -> Vec<String> {
        let chars: Vec<char> = user_input.chars().collect();
        let mut combos = Vec::new();
        for i in 0..chars.len() {
            if i + 1 < chars.len() {
                combos.push(chars[i..=i + 1].iter().collect());
            }
            if i + 2 < chars.len() {
                combos.push(chars[i..=i + 2].iter().collect());
            }
        }
        combos
    }

    fn calculate_combo_speed(&self, combo: &str, duration: Duration) -> f32 {
        let minutes = duration.as_secs_f32() / 60.0;
        (combo.len() as f32 / 5.0) / minutes
    }

    pub(crate) fn get_combinations(&self) -> &[(String, f32)] {
        &self.combinations
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_struggle_combinations() {
        let tracker = StruggleCombinations::default();
        assert!(tracker.get_combinations().is_empty());
    }

    #[test]
    fn test_get_letter_combinations() {
        let tracker = StruggleCombinations::default();
        let combos = tracker.get_letter_combinations("hello");
        // For "hello": he, el, ll, lo (2-char) + hel, ell, llo (3-char) = 7 combinations
        assert_eq!(combos.len(), 7);
    }

    #[test]
    fn test_update_combinations() {
        let mut tracker = StruggleCombinations::default();
        let duration = Duration::from_secs(1);

        tracker.update(duration, "test");
        let combos = tracker.get_combinations();
        assert!(!combos.is_empty());
    }

    #[test]
    fn test_calculate_combo_speed() {
        let tracker = StruggleCombinations::default();
        let duration = Duration::from_secs(60); // 1 minute
        let speed = tracker.calculate_combo_speed("ab", duration);
        // (2 chars / 5) / 1 minute = 0.4 WPM
        assert_eq!(speed, 0.4);
    }

    #[test]
    fn test_combinations_limit() {
        let mut tracker = StruggleCombinations::default();
        let duration = Duration::from_secs(1);

        // Add many combinations
        for i in 0..60 {
            tracker.update(duration, &format!("word{}", i));
        }

        let combos = tracker.get_combinations();
        assert_eq!(combos.len(), 50); // Should be limited to 50
    }
}
