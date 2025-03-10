#[cfg(test)]
mod tests {
    use base64::prelude::*;
    use num_bigint::BigUint;
    use num_traits::Num;

    use crate::utils::{self, bytes_to_string, hex_to_bytes, long_to_bytes};

    #[test]
    fn solve_ascii() {
        let input = [
            99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52,
            98, 108, 51, 125,
        ];
        let got = bytes_to_string(&input);
        let want = "crypto{ASCII_pr1nt4bl3}";
        assert_eq!(got, want);
    }

    #[test]
    fn solve_hex() {
        let input ="63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
        let output = bytes_to_string(&hex_to_bytes(input).unwrap());
        assert_eq!(output, "crypto{You_will_be_working_with_hex_strings_a_lot}");
    }

    #[test]
    fn solve_base64() {
        let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
        let output = hex_to_bytes(input).unwrap();
        assert_eq!(
            BASE64_STANDARD.encode(output),
            "crypto/Base+64+Encoding+is+Web+Safe/"
        );
    }

    #[test]
    fn solve_bytes_and_big_integers() {
        let long =
            "11515195063862318899931685488813747395775516287289682636499965282714637259206269";
        let long = BigUint::from_str_radix(long, 10).unwrap();
        let bytes = long_to_bytes(long).unwrap();
        assert_eq!(bytes_to_string(&bytes), "crypto{3nc0d1n6_4ll_7h3_w4y_d0wn}");
    }

    #[test]
    fn solve_xor_starter() {
        let value = "label";
        let output = utils::xor(value.to_string(), 13);
        assert_eq!("crypto{aloha}", format!("crypto{{{}}}", output));
    }
}
