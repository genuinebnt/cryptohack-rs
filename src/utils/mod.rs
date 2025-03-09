use std::num::ParseIntError;

pub fn ascii_chars_to_string(input: &[u8]) -> String {
    input.iter().map(|v| *v as char).collect()
}

pub fn decode_hex(input: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect()
}
