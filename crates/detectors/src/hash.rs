use ciphex_analysis::SignatureSearch;
use ciphex_core::{CipherType, DetectionResult, HashType};

pub struct HashDetector;

impl HashDetector {
    pub fn detect(input: &str) -> Option<DetectionResult> {
        let trimmed = input.trim();

        if !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        let (hash_type, confidence) = match trimmed.len() {
            32 => (HashType::MD5, 0.95),
            40 => (HashType::SHA1, 0.90), // Lower confidence due to RIPEMD160
            56 => (HashType::SHA224, 0.85),
            64 => (HashType::SHA256, 0.90),
            96 => (HashType::SHA384, 0.85),
            128 => (HashType::SHA512, 0.85),
            _ => return None,
        };

        let mut result = DetectionResult::new(CipherType::Hash(hash_type.clone()), confidence);
        result.add_evidence(format!("Matches {:?} length", hash_type));
        result.add_evidence("Hexadecimal characters only".to_string());

        Some(result)
    }

    pub fn detect_all(input: &str) -> Vec<DetectionResult> {
        let trimmed = input.trim();

        if !trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
            return Vec::new();
        }

        let Some(possible_hashes) = SignatureSearch::check_hash_length(trimmed) else {
            return Vec::new();
        };

        let mut results = Vec::new();

        for hash_name in possible_hashes {
            let (hash_type, confidence) = match hash_name {
                "MD5" => (HashType::MD5, 0.95),
                "SHA1" => (HashType::SHA1, 0.90),
                "SHA224" => (HashType::SHA224, 0.85),
                "SHA256" => (HashType::SHA256, 0.90),
                "SHA384" => (HashType::SHA384, 0.85),
                "SHA512" => (HashType::SHA512, 0.85),
                "SHA3-224" => (HashType::SHA3_224, 0.80),
                "SHA3-256" => (HashType::SHA3_256, 0.80),
                "SHA3-384" => (HashType::SHA3_384, 0.80),
                "SHA3-512" => (HashType::SHA3_512, 0.80),
                "BLAKE2b" => (HashType::BLAKE2b, 0.75),
                "BLAKE2s" => (HashType::BLAKE2s, 0.75),
                "RIPEMD160" => (HashType::RIPEMD160, 0.85),
                "Whirlpool" => (HashType::Whirlpool, 0.80),
                _ => continue,
            };

            let mut result = DetectionResult::new(CipherType::Hash(hash_type), confidence);
            result.add_evidence(format!("Matches {} length ({})", hash_name, trimmed.len()));
            result.add_evidence("Hexadecimal characters only".to_string());

            results.push(result);
        }

        results.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        results
    }
}
