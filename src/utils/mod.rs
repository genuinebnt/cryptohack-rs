pub fn ascii_chars_to_string(input: &[u8]) -> String {
    input.iter().map(|v| *v as char).collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_chars_to_string() {
        let input = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        let got = ascii_chars_to_string(&input);
        let want = "crypto{ASCII_pr1nt4bl3}";
        assert_eq!(got, want);
    }
}
