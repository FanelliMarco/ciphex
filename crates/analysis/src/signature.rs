pub struct SignatureSearch;

impl SignatureSearch {
    pub fn check_hash_length(text: &str) -> Option<Vec<&'static str>> {
        let len = text.len();
        let is_hex = text.chars().all(|c| c.is_ascii_hexdigit());

        if !is_hex {
            return None;
        }

        let mut possible = Vec::new();

        match len {
            32 => possible.push("MD5"),
            40 => {
                // SHA1 and RIPEMD160 both have 40 hex chars (160 bits)
                possible.push("SHA1");
                possible.push("RIPEMD160");
            }
            56 => {
                possible.push("SHA224");
                possible.push("SHA3-224");
            }
            64 => {
                possible.push("SHA256");
                possible.push("SHA3-256");
                possible.push("BLAKE2s");
            }
            96 => {
                possible.push("SHA384");
                possible.push("SHA3-384");
            }
            128 => {
                possible.push("SHA512");
                possible.push("SHA3-512");
                possible.push("BLAKE2b");
                possible.push("Whirlpool");
            }
            _ => return None,
        }

        if possible.is_empty() {
            None
        } else {
            Some(possible)
        }
    }

    pub fn check_base64_signature(text: &str) -> bool {
        if text.len() % 4 != 0 {
            return false;
        }

        let valid_chars = text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');

        if !valid_chars {
            return false;
        }

        let equals_count = text.chars().rev().take_while(|&c| c == '=').count();
        equals_count <= 2
    }

    pub fn check_binary_signature(text: &str) -> bool {
        text.chars().all(|c| c == '0' || c == '1' || c == ' ')
    }

    pub fn check_morse_signature(text: &str) -> bool {
        text.chars()
            .all(|c| c == '.' || c == '-' || c == ' ' || c == '/')
    }
}
