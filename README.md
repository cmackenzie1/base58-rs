# b58

A fast, zero-dependency Base58 encoding and decoding library for Rust.

[![CI](https://github.com/cmackenzie1/base58-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/cmackenzie1/base58-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/b58.svg)](https://crates.io/crates/b58)
[![Documentation](https://docs.rs/b58/badge.svg)](https://docs.rs/b58)

## Features

- **Zero dependencies**: No external crates required at runtime
- **Multiple alphabets**: Supports Bitcoin (default), Ripple, and Flickr Base58 alphabets
- **Arbitrary precision**: Handles inputs of any size using big integer arithmetic
- **Comprehensive error handling**: Clear error messages for invalid input
- **Well tested**: Extensive test suite with edge cases and roundtrip testing

## Installation

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
b58 = "0.1"
```

### As a Command Line Tool

Install the binary using cargo:

```bash
cargo install b58
```

## Usage

### Library Usage

#### Basic Usage

```rust
use b58::{encode, decode};

// Encode bytes to Base58 (uses Bitcoin alphabet by default)
let data = b"Hello, World!";
let encoded = encode(data);
println!("Encoded: {}", encoded); // "72k1xXWG59fYdzSNoA"

// Decode Base58 string back to bytes
let decoded = decode(&encoded).unwrap();
assert_eq!(data, decoded.as_slice());
```

#### Using Different Alphabets

```rust
use b58::{encode_with_alphabet, decode_with_alphabet, Alphabet};

// Encode using Ripple alphabet
let data = b"Hello, World!";
let encoded = encode_with_alphabet(data, Alphabet::Ripple);
println!("Ripple encoded: {}", encoded);

// Decode using Ripple alphabet
let decoded = decode_with_alphabet(&encoded, Alphabet::Ripple).unwrap();
assert_eq!(data, decoded.as_slice());

// Encode using Flickr alphabet
let encoded_flickr = encode_with_alphabet(data, Alphabet::Flickr);
println!("Flickr encoded: {}", encoded_flickr);
```

#### Error Handling

```rust
use b58::{decode, DecodeError};

match decode("invalid0characters") {
    Ok(data) => println!("Decoded: {:?}", data),
    Err(DecodeError::InvalidCharacter(c)) => println!("Invalid character: {}", c),
    Err(e) => println!("Error: {}", e),
}
```

### Command Line Usage

The `base58` binary provides a convenient command-line interface for encoding and decoding Base58 data, similar to the `base64` command:

```bash
# Encode text to Base58 (default behavior)
printf "Hello, World!" | base58
# Output: 72k1xXWG59fYdzSNoA

# Decode Base58 back to original data
printf "72k1xXWG59fYdzSNoA" | base58 -d
# Output: Hello, World!

# Use different alphabets
printf "Hello, World!" | base58 --alphabet ripple
# Output: fpkrxXWGn9CYdzS4ow

printf "Hello, World!" | base58 --alphabet flickr
# Output: 72K1Xwvg59ExCZrnNa

# Decode with specific alphabet
printf "fpkrxXWGn9CYdzS4ow" | base58 -d --alphabet ripple
# Output: Hello, World!

# Encode/decode files
base58 < input.txt > encoded.txt
base58 -d < encoded.txt > output.txt

# Show help
base58 --help
```

#### Available Options

- `-d, --decode` - Decode Base58 input (default: encode)
- `-a, --alphabet <ALPHABET>` - Specify alphabet (bitcoin, ripple, flickr) [default: bitcoin]  
- `-h, --help` - Show help information

#### Design Philosophy

Like the standard `base64` command, `base58` defaults to encoding mode when no flags are specified. This provides a clean, intuitive interface that follows Unix conventions.

## API Reference

### Functions

- `encode(input: &[u8]) -> String` - Encodes a byte slice to a Base58 string using Bitcoin alphabet
- `decode(input: &str) -> Result<Vec<u8>, DecodeError>` - Decodes a Base58 string to bytes using Bitcoin alphabet
- `encode_with_alphabet(input: &[u8], alphabet: Alphabet) -> String` - Encodes using specified alphabet
- `decode_with_alphabet(input: &str, alphabet: Alphabet) -> Result<Vec<u8>, DecodeError>` - Decodes using specified alphabet

### Alphabets

- `Alphabet::Bitcoin` (default) - `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`
- `Alphabet::Ripple` - `rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz`
- `Alphabet::Flickr` - `123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ`

### Error Types

- `DecodeError::InvalidCharacter(char)` - Invalid character in Base58 string
- `DecodeError::EmptyInput` - Empty input string (currently unused)
- `DecodeError::Overflow` - Numeric overflow during decoding

## Implementation Details

This library uses big integer arithmetic to handle arbitrarily large inputs without overflow. The implementation:

1. **Encoding**: Converts input bytes to a big integer, then repeatedly divides by 58 to get Base58 digits
2. **Decoding**: Multiplies accumulated value by 58 and adds each digit value
3. **Leading zeros**: Properly handles leading zero bytes as the first character of the chosen alphabet
4. **Alphabet flexibility**: Each alphabet variant maintains its own character set and decode table for efficient lookups

## Performance

The library is optimized for correctness and clarity rather than raw speed. For most use cases, performance is more than adequate. The big integer arithmetic ensures no data loss for large inputs.

## Testing

Run the test suite:

```bash
cargo test
```

The tests include:
- Basic encoding/decoding
- Edge cases (empty input, all zeros, large numbers)
- Invalid character handling
- Roundtrip testing with various data sizes
- All alphabet character validation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Compatibility

- Rust 2024 edition
- No external dependencies
- Works with `no_std` environments (std features used only for error trait implementations)

## FAQ

### Why Base58?

Base58 was designed specifically for use in Bitcoin addresses to provide human-friendly encoding with several key benefits:
- **No ambiguous characters**: Excludes 0 (zero), O (capital o), I (capital i) and l (lowercase L) to prevent confusion
- **Alphanumeric only**: Contains only letters and numbers, making it easy to select with double-click
- **Compact representation**: More efficient than hexadecimal while remaining readable

### Is Base58 URL-safe?

Yes! Base58 is inherently URL-safe because it only uses alphanumeric characters (no special characters like +, /, or =). This makes it ideal for:
- URL parameters
- File names
- Database keys
- Any context where special characters might cause issues

### Encoding Size Comparison

Here's how Base58 compares to other common encodings:

| Encoding | Characters Used | Size Increase | URL-Safe |
|----------|----------------|---------------|----------|
| Hex      | 16             | 100%          | Yes      |
| Base32   | 32             | 60%           | Yes*     |
| Base58   | 58             | ~38%          | Yes      |
| Base64   | 64             | 33%           | No**     |

*Base32 typically uses padding (=) which requires URL encoding
**Base64 uses +, /, and = which require URL encoding

For example, encoding 32 random bytes:
- Hex: 64 characters
- Base32: 52 characters (plus padding)
- Base58: 44 characters
- Base64: 44 characters (plus padding)

While Base58 produces slightly larger output than Base64, it's completely URL-safe without any encoding, making it perfect for web applications, APIs, and anywhere human readability matters.
