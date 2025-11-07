use std::collections::HashMap;

pub struct CoincidenceIndex;

impl CoincidenceIndex {
    /// Calculate the Index of Coincidence for the given text
    /// IC ≈ 0.067 for English text
    /// IC ≈ 0.038 for random text
    pub fn calculate(text: &str) -> f64 {
        let text = text.to_uppercase();
        let mut freq: HashMap<char, usize> = HashMap::new();
        let mut n = 0;

        for c in text.chars() {
            if c.is_ascii_alphabetic() {
                *freq.entry(c).or_insert(0) += 1;
                n += 1;
            }
        }

        if n <= 1 {
            return 0.0;
        }

        let sum: usize = freq.values().map(|&f| f * (f - 1)).sum();
        sum as f64 / (n * (n - 1)) as f64
    }

    pub fn is_likely_english(text: &str) -> bool {
        let ic = Self::calculate(text);
        ic > 0.06 && ic < 0.08
    }

    pub fn is_likely_random(text: &str) -> bool {
        let ic = Self::calculate(text);
        ic < 0.045
    }
}
