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

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
b58 = "0.1"
```

### Basic Usage

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

### Using Different Alphabets

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

### Error Handling

```rust
use b58::{decode, DecodeError};

match decode("invalid0characters") {
    Ok(data) => println!("Decoded: {:?}", data),
    Err(DecodeError::InvalidCharacter(c)) => println!("Invalid character: {}", c),
    Err(e) => println!("Error: {}", e),
}
```

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
