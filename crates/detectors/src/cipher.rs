use ciphex_analysis::CoincidenceIndex;
use ciphex_core::{CipherType, ClassicalCipher, DetectionResult};

pub struct CipherDetector;

impl CipherDetector {
    pub fn detect(input: &str) -> Option<DetectionResult> {
        let trimmed = input.trim();

        // Only alphabetic with possible spaces
        if !trimmed
            .chars()
            .all(|c| c.is_alphabetic() || c.is_whitespace())
        {
            return None;
        }

        let ic = CoincidenceIndex::calculate(trimmed);

        // Check for ROT13 or Caesar (IC similar to English)
        if CoincidenceIndex::is_likely_english(trimmed) {
            return Some(
                DetectionResult::new(CipherType::Cipher(ClassicalCipher::Caesar), 0.70)
                    .with_evidence(format!("Index of Coincidence: {:.4}", ic))
                    .with_evidence("IC similar to natural language".to_string()),
            );
        }

        // Check for substitution cipher
        if ic > 0.055 && ic < 0.075 {
            return Some(
                DetectionResult::new(CipherType::Cipher(ClassicalCipher::Substitution), 0.65)
                    .with_evidence(format!("Index of Coincidence: {:.4}", ic))
                    .with_evidence("Possible substitution cipher".to_string()),
            );
        }

        None
    }
}
