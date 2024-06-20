pub fn xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .into_iter()
        .zip(repeat(key, input.len()))
        .map(|(this, other)| this ^ other)
        .collect()
}

fn repeat(input: &[u8], len: usize) -> Vec<u8> {
    let mut repeated = Vec::new();

    if input.len() >= len {
        return input.to_vec();
    }

    while repeated.len() < len {
        for b in input {
            repeated.push(*b);
        }
    }

    repeated
}
