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
