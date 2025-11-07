use ciphex_analysis::{FrequencyAnalysis, SignatureSearch};
use ciphex_core::{CipherType, DetectionResult, EncodingType};

pub struct EncodingDetector;

impl EncodingDetector {
    pub fn detect(input: &str) -> Option<DetectionResult> {
        let trimmed = input.trim();
        let freq = FrequencyAnalysis::new(trimmed);

        // Check Base64
        if SignatureSearch::check_base64_signature(trimmed) {
            return Some(
                DetectionResult::new(CipherType::Encoding(EncodingType::Base64), 0.90)
                    .with_evidence("Valid Base64 character set".to_string())
                    .with_evidence("Correct padding".to_string()),
            );
        }

        // Check Hex
        if freq.is_hex_only() && trimmed.len() % 2 == 0 {
            return Some(
                DetectionResult::new(CipherType::Encoding(EncodingType::Hex), 0.85)
                    .with_evidence("Hexadecimal characters only".to_string())
                    .with_evidence("Even length".to_string()),
            );
        }

        // Check Binary
        if SignatureSearch::check_binary_signature(trimmed) {
            return Some(
                DetectionResult::new(CipherType::Encoding(EncodingType::Binary), 0.90)
                    .with_evidence("Binary characters only".to_string()),
            );
        }

        // Check Morse
        if SignatureSearch::check_morse_signature(trimmed) {
            return Some(
                DetectionResult::new(CipherType::Encoding(EncodingType::Morse), 0.85)
                    .with_evidence("Morse code characters detected".to_string()),
            );
        }

        None
    }
}
