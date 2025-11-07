pub mod cipher;
pub mod encoding;
pub mod hash;

pub use cipher::CipherDetector;
pub use encoding::EncodingDetector;
pub use hash::HashDetector;

use ciphex_core::{CipherType, DetectionResult};

pub struct UniversalDetector;

impl UniversalDetector {
    pub fn detect(input: &str) -> DetectionResult {
        // Try hash detection first (most specific)
        if let Some(result) = HashDetector::detect(input) {
            return result;
        }

        // Try encoding detection
        if let Some(result) = EncodingDetector::detect(input) {
            return result;
        }

        // Try cipher detection
        if let Some(result) = CipherDetector::detect(input) {
            return result;
        }

        // Unknown
        DetectionResult::new(CipherType::Unknown, 0.0)
            .with_evidence("No pattern matched".to_string())
    }

    pub fn detect_all(input: &str) -> Vec<DetectionResult> {
        let mut results = Vec::new();

        // Get all hash possibilities
        let hash_results = HashDetector::detect_all(input);
        results.extend(hash_results);

        // Add encoding detection
        if let Some(result) = EncodingDetector::detect(input) {
            results.push(result);
        }

        // Add cipher detection
        if let Some(result) = CipherDetector::detect(input) {
            results.push(result);
        }

        if results.is_empty() {
            results.push(
                DetectionResult::new(CipherType::Unknown, 0.0)
                    .with_evidence("No pattern matched".to_string()),
            );
        }

        // Sort by confidence
        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        results
    }
}
