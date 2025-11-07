use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FrequencyAnalysis {
    char_freq: HashMap<char, usize>,
    total_chars: usize,
    // Add #[allow(dead_code)] or use it
}

impl FrequencyAnalysis {
    pub fn new(text: &str) -> Self {
        let mut char_freq = HashMap::new();
        let total_chars = text.len();

        for c in text.chars() {
            *char_freq.entry(c).or_insert(0) += 1;
        }

        Self {
            char_freq,
            total_chars,
        }
    }

    pub fn char_frequency(&self, c: char) -> f64 {
        self.char_freq.get(&c).copied().unwrap_or(0) as f64 / self.total_chars as f64
    }

    pub fn most_common_chars(&self, n: usize) -> Vec<(char, usize)> {
        let mut freq_vec: Vec<_> = self.char_freq.iter().map(|(&k, &v)| (k, v)).collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        freq_vec.into_iter().take(n).collect()
    }

    pub fn entropy(&self) -> f64 {
        self.char_freq
            .values()
            .map(|&count| {
                let p = count as f64 / self.total_chars as f64;
                if p > 0.0 {
                    -p * p.log2()
                } else {
                    0.0
                }
            })
            .sum()
    }

    pub fn is_hex_only(&self) -> bool {
        self.char_freq.keys().all(|c| c.is_ascii_hexdigit())
    }

    pub fn is_base64_charset(&self) -> bool {
        self.char_freq
            .keys()
            .all(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/' || *c == '=')
    }
}
