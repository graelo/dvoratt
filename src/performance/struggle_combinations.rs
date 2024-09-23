use std::time::Duration;

pub struct StruggleCombinations {
    pub combinations: Vec<(String, f32)>,
}

impl StruggleCombinations {
    pub fn new() -> Self {
        StruggleCombinations {
            combinations: Vec::new(),
        }
    }

    pub fn update(&mut self, duration: Duration, user_input: &str) {
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
            .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        self.combinations.truncate(50);
    }

    fn get_letter_combinations(&self, user_input: &str) -> Vec<String> {
        let mut combos = Vec::new();
        for i in 0..user_input.len() {
            if i < user_input.len() - 1 {
                combos.push(user_input[i..=i + 1].to_string());
            }
            if user_input.len() >= 2 && i < user_input.len() - 2 {
                combos.push(user_input[i..=i + 2].to_string());
            }
        }
        combos
    }

    fn calculate_combo_speed(&self, combo: &str, duration: Duration) -> f32 {
        let minutes = duration.as_secs_f32() / 60.0;
        (combo.len() as f32 / 5.0) / minutes
    }

    pub fn get_combinations(&self) -> &[(String, f32)] {
        &self.combinations
    }
}
