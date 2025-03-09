#[cfg(test)]
mod tests {
    use crate::utils::{ascii_chars_to_string, decode_hex};

    #[test]
    fn solve_intro_ascii() {
        let input = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        let got = ascii_chars_to_string(&input);
        let want = "crypto{ASCII_pr1nt4bl3}";
        assert_eq!(got, want);
    }

    #[test]
    fn solve_intro_hex() {
        let input ="63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
        let output = ascii_chars_to_string(&decode_hex(input).unwrap());
        assert_eq!(output, "crypto{You_will_be_working_with_hex_strings_a_lot}");
    }
}
