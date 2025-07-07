//! A Base58 encoding/decoding library with no external dependencies.
//!
//! This library provides encoding and decoding functionality for Base58 format
//! with support for multiple alphabets including Bitcoin (default), Ripple, and Flickr.
//!
//! # Examples
//!
//! ```
//! use b58::{encode, decode, encode_with_alphabet, decode_with_alphabet, Alphabet};
//!
//! // Using default Bitcoin alphabet
//! let data = b"Hello, World!";
//! let encoded = encode(data);
//! let decoded = decode(&encoded).unwrap();
//! assert_eq!(data, decoded.as_slice());
//!
//! // Using Ripple alphabet
//! let encoded_ripple = encode_with_alphabet(data, Alphabet::Ripple);
//! let decoded_ripple = decode_with_alphabet(&encoded_ripple, Alphabet::Ripple).unwrap();
//! assert_eq!(data, decoded_ripple.as_slice());
//! ```

/// Enum representing different Base58 alphabets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alphabet {
    /// Bitcoin alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
    #[default]
    Bitcoin,
    /// Ripple alphabet: rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz
    Ripple,
    /// Flickr alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ
    Flickr,
}

impl Alphabet {
    /// Returns the alphabet string for the given alphabet variant.
    pub fn as_bytes(&self) -> &'static [u8] {
        match self {
            Alphabet::Bitcoin => b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz",
            Alphabet::Ripple => b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz",
            Alphabet::Flickr => b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ",
        }
    }

    /// Returns the decode table for the given alphabet variant.
    pub fn decode_table(&self) -> [u8; 256] {
        let mut table = [255u8; 256];
        let alphabet = self.as_bytes();
        let mut i = 0;
        while i < alphabet.len() {
            table[alphabet[i] as usize] = i as u8;
            i += 1;
        }
        table
    }
}

/// Encodes a byte slice into a Base58 string using the default Bitcoin alphabet.
///
/// # Arguments
///
/// * `input` - The byte slice to encode
///
/// # Returns
///
/// A Base58 encoded string
///
/// # Examples
///
/// ```
/// use b58::encode;
///
/// let data = b"Hello";
/// let encoded = encode(data);
/// assert_eq!(encoded, "9Ajdvzr");
/// ```
pub fn encode(input: &[u8]) -> String {
    encode_with_alphabet(input, Alphabet::Bitcoin)
}

/// Encodes a byte slice into a Base58 string using the specified alphabet.
///
/// # Arguments
///
/// * `input` - The byte slice to encode
/// * `alphabet` - The alphabet to use for encoding
///
/// # Returns
///
/// A Base58 encoded string
///
/// # Examples
///
/// ```
/// use b58::{encode_with_alphabet, Alphabet};
///
/// let data = b"Hello";
/// let encoded = encode_with_alphabet(data, Alphabet::Ripple);
/// ```
pub fn encode_with_alphabet(input: &[u8], alphabet: Alphabet) -> String {
    if input.is_empty() {
        return String::new();
    }

    // Count leading zeros
    let leading_zeros = input.iter().take_while(|&&b| b == 0).count();

    // Skip leading zeros for calculation
    let significant_bytes = &input[leading_zeros..];

    if significant_bytes.is_empty() {
        // All zeros
        let zero_char = alphabet.as_bytes()[0] as char;
        return zero_char.to_string().repeat(leading_zeros);
    }

    // For larger numbers, use a different approach
    let mut result = encode_big_int(significant_bytes, alphabet);

    // Add leading zero characters for leading zeros
    let zero_char = alphabet.as_bytes()[0] as char;
    for _ in 0..leading_zeros {
        result.insert(0, zero_char);
    }

    result
}

/// Encodes using big integer arithmetic with Vec<u8> for arbitrary precision
fn encode_big_int(input: &[u8], alphabet: Alphabet) -> String {
    let mut num = input.to_vec();
    let mut encoded = Vec::new();
    let alphabet_bytes = alphabet.as_bytes();

    // Convert to base58 using long division
    while !is_zero(&num) {
        let remainder = divide_by_58(&mut num);
        encoded.push(alphabet_bytes[remainder]);
    }

    if encoded.is_empty() {
        encoded.push(alphabet_bytes[0]);
    }

    encoded.reverse();
    String::from_utf8(encoded).unwrap()
}

/// Check if a big integer (as Vec<u8>) is zero
fn is_zero(num: &[u8]) -> bool {
    num.iter().all(|&b| b == 0)
}

/// Divide a big integer by 58 and return the remainder
fn divide_by_58(num: &mut [u8]) -> usize {
    let mut remainder = 0u16;

    for byte in num.iter_mut() {
        let temp = remainder * 256 + *byte as u16;
        *byte = (temp / 58) as u8;
        remainder = temp % 58;
    }

    remainder as usize
}

/// Error type for Base58 decoding failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// Invalid character encountered during decoding.
    InvalidCharacter(char),
    /// Input string is empty.
    EmptyInput,
    /// Numeric overflow during decoding.
    Overflow,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::InvalidCharacter(c) => write!(f, "Invalid character: '{c}'"),
            DecodeError::EmptyInput => write!(f, "Input string is empty"),
            DecodeError::Overflow => write!(f, "Numeric overflow during decoding"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// Decodes a Base58 string into a byte vector using the default Bitcoin alphabet.
///
/// # Arguments
///
/// * `input` - The Base58 string to decode
///
/// # Returns
///
/// A `Result` containing the decoded bytes on success, or a `DecodeError` on failure
///
/// # Examples
///
/// ```
/// use b58::decode;
///
/// let encoded = "9Ajdvzr";
/// let decoded = decode(encoded).unwrap();
/// assert_eq!(decoded, b"Hello");
/// ```
pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
    decode_with_alphabet(input, Alphabet::Bitcoin)
}

/// Decodes a Base58 string into a byte vector using the specified alphabet.
///
/// # Arguments
///
/// * `input` - The Base58 string to decode
/// * `alphabet` - The alphabet to use for decoding
///
/// # Returns
///
/// A `Result` containing the decoded bytes on success, or a `DecodeError` on failure
///
/// # Examples
///
/// ```
/// use b58::{decode_with_alphabet, Alphabet};
///
/// let encoded = "9Ajdvzr";
/// let decoded = decode_with_alphabet(encoded, Alphabet::Bitcoin).unwrap();
/// assert_eq!(decoded, b"Hello");
/// ```
pub fn decode_with_alphabet(input: &str, alphabet: Alphabet) -> Result<Vec<u8>, DecodeError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let zero_char = alphabet.as_bytes()[0] as char;

    // Count leading zero characters
    let leading_zeros = input.chars().take_while(|&c| c == zero_char).count();

    // Skip leading zero characters for calculation
    let significant_chars: String = input.chars().skip(leading_zeros).collect();

    if significant_chars.is_empty() {
        // All zero characters
        return Ok(vec![0; leading_zeros]);
    }

    // Decode using big integer arithmetic
    let mut result = decode_big_int(&significant_chars, alphabet)?;

    // Add leading zeros for leading zero characters
    for _ in 0..leading_zeros {
        result.insert(0, 0);
    }

    Ok(result)
}

/// Decodes using big integer arithmetic with Vec<u8> for arbitrary precision
fn decode_big_int(input: &str, alphabet: Alphabet) -> Result<Vec<u8>, DecodeError> {
    let mut num = vec![0u8];
    let decode_table = alphabet.decode_table();

    for c in input.chars() {
        let c_val = c as u32;
        if c_val >= 256 {
            return Err(DecodeError::InvalidCharacter(c));
        }

        let digit = decode_table[c_val as usize];
        if digit == 255 {
            return Err(DecodeError::InvalidCharacter(c));
        }

        // Multiply by 58 and add digit
        multiply_by_58(&mut num);
        add_digit(&mut num, digit);
    }

    // Remove leading zeros
    while num.len() > 1 && num[0] == 0 {
        num.remove(0);
    }

    Ok(num)
}

/// Multiply a big integer by 58
fn multiply_by_58(num: &mut Vec<u8>) {
    let mut carry = 0u16;

    for byte in num.iter_mut().rev() {
        let temp = *byte as u16 * 58 + carry;
        *byte = (temp % 256) as u8;
        carry = temp / 256;
    }

    while carry > 0 {
        num.insert(0, (carry % 256) as u8);
        carry /= 256;
    }
}

/// Add a single digit to a big integer
fn add_digit(num: &mut Vec<u8>, digit: u8) {
    let mut carry = digit as u16;

    for byte in num.iter_mut().rev() {
        let temp = *byte as u16 + carry;
        *byte = (temp % 256) as u8;
        carry = temp / 256;
        if carry == 0 {
            break;
        }
    }

    while carry > 0 {
        num.insert(0, (carry % 256) as u8);
        carry /= 256;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_empty() {
        assert_eq!(encode(&[]), "");
    }

    #[test]
    fn test_encode_single_zero() {
        assert_eq!(encode(&[0]), "1");
    }

    #[test]
    fn test_encode_multiple_zeros() {
        assert_eq!(encode(&[0, 0, 0]), "111");
    }

    #[test]
    fn test_encode_hello() {
        assert_eq!(encode(b"Hello"), "9Ajdvzr");
    }

    #[test]
    fn test_encode_hello_world() {
        assert_eq!(encode(b"Hello, World!"), "72k1xXWG59fYdzSNoA");
    }

    #[test]
    fn test_encode_with_leading_zeros() {
        assert_eq!(encode(&[0, 0, 1, 2, 3]), "11Ldp");
    }

    #[test]
    fn test_decode_empty() {
        assert_eq!(decode("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_decode_single_one() {
        assert_eq!(decode("1").unwrap(), vec![0]);
    }

    #[test]
    fn test_decode_multiple_ones() {
        assert_eq!(decode("111").unwrap(), vec![0, 0, 0]);
    }

    #[test]
    fn test_decode_hello() {
        assert_eq!(decode("9Ajdvzr").unwrap(), b"Hello");
    }

    #[test]
    fn test_decode_hello_world() {
        assert_eq!(decode("72k1xXWG59fYdzSNoA").unwrap(), b"Hello, World!");
    }

    #[test]
    fn test_decode_with_leading_ones() {
        assert_eq!(decode("11Ldp").unwrap(), vec![0, 0, 1, 2, 3]);
    }

    #[test]
    fn test_decode_invalid_character() {
        match decode("9Ajdvzr0") {
            Err(DecodeError::InvalidCharacter('0')) => {}
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn test_decode_invalid_character_unicode() {
        match decode("9Ajdvzr€") {
            Err(DecodeError::InvalidCharacter('€')) => {}
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn test_roundtrip_random_data() {
        let test_cases = vec![
            vec![],
            vec![0],
            vec![0, 0, 0],
            vec![1, 2, 3, 4, 5],
            vec![255, 254, 253],
            b"The quick brown fox jumps over the lazy dog".to_vec(),
            (0..=255).collect::<Vec<u8>>(),
        ];

        for original in test_cases {
            let encoded = encode(&original);
            let decoded = decode(&encoded).unwrap();
            assert_eq!(original, decoded, "Roundtrip failed for {original:?}");
        }
    }

    #[test]
    fn test_encode_large_number() {
        let large_input = vec![255; 16]; // 16 bytes of 0xFF
        let encoded = encode(&large_input);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(large_input, decoded);
    }

    #[test]
    fn test_all_alphabet_characters() {
        for &c in Alphabet::Bitcoin.as_bytes().iter() {
            let decoded = decode(&(c as char).to_string()).unwrap();
            assert!(
                !decoded.is_empty(),
                "Decoding alphabet character {} failed",
                c as char
            );
        }
    }

    #[test]
    fn test_encode_with_ripple_alphabet() {
        let data = b"Hello";
        let encoded = encode_with_alphabet(data, Alphabet::Ripple);
        let decoded = decode_with_alphabet(&encoded, Alphabet::Ripple).unwrap();
        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_encode_with_flickr_alphabet() {
        let data = b"Hello";
        let encoded = encode_with_alphabet(data, Alphabet::Flickr);
        let decoded = decode_with_alphabet(&encoded, Alphabet::Flickr).unwrap();
        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_different_alphabets_produce_different_results() {
        let data = b"Hello, World!";
        let bitcoin_encoded = encode_with_alphabet(data, Alphabet::Bitcoin);
        let ripple_encoded = encode_with_alphabet(data, Alphabet::Ripple);
        let flickr_encoded = encode_with_alphabet(data, Alphabet::Flickr);

        // They should all be different
        assert_ne!(bitcoin_encoded, ripple_encoded);
        assert_ne!(bitcoin_encoded, flickr_encoded);
        assert_ne!(ripple_encoded, flickr_encoded);

        // But they should all decode back to the same data
        assert_eq!(
            decode_with_alphabet(&bitcoin_encoded, Alphabet::Bitcoin).unwrap(),
            data
        );
        assert_eq!(
            decode_with_alphabet(&ripple_encoded, Alphabet::Ripple).unwrap(),
            data
        );
        assert_eq!(
            decode_with_alphabet(&flickr_encoded, Alphabet::Flickr).unwrap(),
            data
        );
    }

    #[test]
    fn test_cross_alphabet_decoding_fails() {
        let data = b"Hello";
        let bitcoin_encoded = encode_with_alphabet(data, Alphabet::Bitcoin);

        // Trying to decode with wrong alphabet should fail (in most cases)
        // Note: This might not always fail due to overlapping characters, but it's worth testing
        let result = decode_with_alphabet(&bitcoin_encoded, Alphabet::Ripple);
        if result.is_ok() {
            // If it doesn't fail, the result should be different from original
            assert_ne!(result.unwrap(), data);
        }
    }

    #[test]
    fn test_ripple_alphabet_roundtrip() {
        let test_cases = vec![
            vec![],
            vec![0],
            vec![0, 0, 0],
            vec![1, 2, 3, 4, 5],
            vec![255, 254, 253],
            b"The quick brown fox jumps over the lazy dog".to_vec(),
        ];

        for original in test_cases {
            let encoded = encode_with_alphabet(&original, Alphabet::Ripple);
            let decoded = decode_with_alphabet(&encoded, Alphabet::Ripple).unwrap();
            assert_eq!(
                original, decoded,
                "Ripple roundtrip failed for {original:?}"
            );
        }
    }

    #[test]
    fn test_flickr_alphabet_roundtrip() {
        let test_cases = vec![
            vec![],
            vec![0],
            vec![0, 0, 0],
            vec![1, 2, 3, 4, 5],
            vec![255, 254, 253],
            b"The quick brown fox jumps over the lazy dog".to_vec(),
        ];

        for original in test_cases {
            let encoded = encode_with_alphabet(&original, Alphabet::Flickr);
            let decoded = decode_with_alphabet(&encoded, Alphabet::Flickr).unwrap();
            assert_eq!(
                original, decoded,
                "Flickr roundtrip failed for {original:?}"
            );
        }
    }

    #[test]
    fn test_alphabet_default() {
        assert_eq!(Alphabet::default(), Alphabet::Bitcoin);
    }

    #[test]
    fn test_alphabet_as_bytes() {
        assert_eq!(
            Alphabet::Bitcoin.as_bytes(),
            b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
        );
        assert_eq!(
            Alphabet::Ripple.as_bytes(),
            b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz"
        );
        assert_eq!(
            Alphabet::Flickr.as_bytes(),
            b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ"
        );
    }
}
