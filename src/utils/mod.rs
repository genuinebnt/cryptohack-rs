use std::num::ParseIntError;

use hex::FromHexError;
use num_bigint::BigUint;

pub fn bytes_to_string(input: &[u8]) -> String {
    input.iter().map(|v| *v as char).collect()
}

pub fn hex_to_bytes(input: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect()
}

pub fn bytes_to_long(input: &[u8]) -> Result<u64, ParseIntError> {
    let hex_str = hex::encode(input);
    u64::from_str_radix(&hex_str, 16)
}

pub fn long_to_bytes(input: BigUint) -> Result<Vec<u8>, ParseIntError> {
    let hex_str = input.to_str_radix(16);
    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_long() {
        let bytes = [72, 69, 76, 76, 79];
        assert_eq!(bytes_to_long(&bytes), Ok(310400273487))
    }
}
