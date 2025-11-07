# Cipher Identifier

A powerful, modular Rust CLI tool for identifying cryptographic hashes, encodings, and classical ciphers using advanced cryptanalysis techniques.

## Features

- 🔍 **Hash Detection**: MD5, SHA-1, SHA-224, SHA-256, SHA-384, SHA-512, SHA-3 variants, BLAKE2, RIPEMD-160, Whirlpool
- 📝 **Encoding Detection**: Base64, Hex, Binary, Morse code, URL encoding, HTML entities
- 🔐 **Classical Cipher Detection**: Caesar, Vigenère, Atbash, ROT13, Substitution
- 📊 **Advanced Analysis**:
  - Frequency Analysis
  - Index of Coincidence
  - Signature Search
  - Bigram/Trigram Analysis
- 🏗️ **Modular Architecture**: Clean workspace structure with separate crates for core logic, detection, and analysis
- 🎨 **Beautiful CLI**: Colored output with confidence scores and detailed evidence

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/FanelliMarco/ciphex.git
cd ciphex

# Build and install
cargo install --path crates/cli

# The binary will be installed to ~/.cargo/bin/cipher (Unix) or %USERPROFILE%\.cargo\bin\cipher.exe (Windows)
```
