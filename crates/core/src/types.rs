use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CipherType {
    Hash(HashType),
    Encoding(EncodingType),
    Cipher(ClassicalCipher),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashType {
    MD5,
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    SHA3_224,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    BLAKE2b,
    BLAKE2s,
    RIPEMD160,
    Whirlpool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingType {
    Base64,
    Base32,
    Hex,
    Binary,
    Morse,
    URL,
    HTML,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassicalCipher {
    Caesar,
    Vigenere,
    Atbash,
    ROT13,
    Substitution,
}

// THIS IS THE KEY FIX - Add Serialize and Deserialize
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub cipher_type: CipherType,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

impl fmt::Display for CipherType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherType::Hash(h) => write!(f, "{:?}", h),
            CipherType::Encoding(e) => write!(f, "{:?}", e),
            CipherType::Cipher(c) => write!(f, "{:?}", c),
            CipherType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl DetectionResult {
    pub fn new(cipher_type: CipherType, confidence: f64) -> Self {
        Self {
            cipher_type,
            confidence,
            evidence: Vec::new(),
        }
    }

    pub fn with_evidence(mut self, evidence: String) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn add_evidence(&mut self, evidence: String) {
        self.evidence.push(evidence);
    }
}
