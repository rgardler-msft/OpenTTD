/// Gamma encoding/decoding for variable-length integers
/// Used in OpenTTD savegames for length fields and array indices
use openttd_core::error::CoreError;

/// Read a gamma-encoded value from a buffer
/// Returns the value and the number of bytes consumed
pub fn decode_gamma(buf: &[u8]) -> Result<(u64, usize), CoreError> {
    if buf.is_empty() {
        return Err(CoreError::BufferTooSmall);
    }

    let first_byte = buf[0];

    // Count leading ones to determine byte count
    let byte_count = if first_byte & 0x80 == 0 {
        // 0xxxxxxx - 1 byte
        return Ok((first_byte as u64, 1));
    } else if first_byte & 0x40 == 0 {
        // 10xxxxxx - 2 bytes
        2
    } else if first_byte & 0x20 == 0 {
        // 110xxxxx - 3 bytes
        3
    } else if first_byte & 0x10 == 0 {
        // 1110xxxx - 4 bytes
        4
    } else {
        // 11110xxx - 5 bytes (max)
        5
    };

    if buf.len() < byte_count {
        return Err(CoreError::BufferTooSmall);
    }

    let value = match byte_count {
        2 => {
            let value = ((first_byte & 0x3F) as u64) << 8;
            value | buf[1] as u64
        }
        3 => {
            let mut value = ((first_byte & 0x1F) as u64) << 16;
            value |= (buf[1] as u64) << 8;
            value | buf[2] as u64
        }
        4 => {
            let mut value = ((first_byte & 0x0F) as u64) << 24;
            value |= (buf[1] as u64) << 16;
            value |= (buf[2] as u64) << 8;
            value | buf[3] as u64
        }
        5 => {
            let mut value = ((first_byte & 0x07) as u64) << 32;
            value |= (buf[1] as u64) << 24;
            value |= (buf[2] as u64) << 16;
            value |= (buf[3] as u64) << 8;
            value | buf[4] as u64
        }
        _ => unreachable!(),
    };

    Ok((value, byte_count))
}

/// Encode a value using gamma encoding
/// Returns the encoded bytes
pub fn encode_gamma(value: u64) -> Vec<u8> {
    if value <= 0x7F {
        // Fits in 1 byte: 0xxxxxxx
        vec![value as u8]
    } else if value <= 0x3FFF {
        // Fits in 2 bytes: 10xxxxxx xxxxxxxx
        vec![0x80 | ((value >> 8) as u8), (value & 0xFF) as u8]
    } else if value <= 0x1FFFFF {
        // Fits in 3 bytes: 110xxxxx xxxxxxxx xxxxxxxx
        vec![
            0xC0 | ((value >> 16) as u8),
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ]
    } else if value <= 0x0FFFFFFF {
        // Fits in 4 bytes: 1110xxxx xxxxxxxx xxxxxxxx xxxxxxxx
        vec![
            0xE0 | ((value >> 24) as u8),
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ]
    } else {
        // Fits in 5 bytes: 11110--- xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
        vec![
            0xF0,
            ((value >> 24) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_single_byte() {
        // Test values that fit in single byte
        for i in 0..=127u64 {
            let encoded = encode_gamma(i);
            assert_eq!(encoded.len(), 1);
            assert_eq!(encoded[0], i as u8);

            let (decoded, bytes_read) = decode_gamma(&encoded).unwrap();
            assert_eq!(decoded, i);
            assert_eq!(bytes_read, 1);
        }
    }

    #[test]
    fn test_gamma_two_bytes() {
        // Test boundary cases for 2-byte encoding
        let test_cases = vec![
            128u64, // Minimum 2-byte value
            255u64, 16383u64, // Maximum 2-byte value
        ];

        for value in test_cases {
            let encoded = encode_gamma(value);
            assert_eq!(encoded.len(), 2);

            let (decoded, bytes_read) = decode_gamma(&encoded).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(bytes_read, 2);
        }
    }

    #[test]
    fn test_gamma_three_bytes() {
        // Test boundary cases for 3-byte encoding
        let test_cases = vec![
            16384u64, // Minimum 3-byte value
            65536u64, 2097151u64, // Maximum 3-byte value
        ];

        for value in test_cases {
            let encoded = encode_gamma(value);
            assert_eq!(encoded.len(), 3);

            let (decoded, bytes_read) = decode_gamma(&encoded).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(bytes_read, 3);
        }
    }

    #[test]
    fn test_gamma_round_trip() {
        // Test various values for round-trip encoding/decoding
        let test_values = vec![
            0, 1, 127, 128, 255, 256, 16383, 16384, 65535, 65536, 1000000, 2097151, 2097152,
            268435455, 268435456,
        ];

        for value in test_values {
            let encoded = encode_gamma(value);
            let (decoded, _) = decode_gamma(&encoded).unwrap();
            assert_eq!(decoded, value, "Failed round-trip for value {}", value);
        }
    }
}
