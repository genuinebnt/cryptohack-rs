pub fn bytes_to_string(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

pub fn rot13_decode(encoded: &str) -> Vec<u8> {
    encoded
        .as_bytes()
        .into_iter()
        .map(|v| match v {
            b'A'..=b'Z' => (v - b'A' + 13) % 26 + b'A',
            b'a'..=b'z' => (v - b'a' + 13) % 26 + b'a',
            _ => *v,
        })
        .collect()
}

pub fn xor(bytes: &[u8], value: u8) -> Vec<u8> {
    bytes.into_iter().map(|byte| byte ^ value).collect()
}
