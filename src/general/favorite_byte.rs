use crate::utils::{bytes::bytes_to_string, hex::hex_to_bytes};
use crate::utils::xor::xor_with_byte;

pub fn favorite_byte() -> String {
    let hidden = "73626960647f6b206821204f21254f7d694f7624662065622127234f726927756d";
    let hidden = hex_to_bytes(hidden);
    println!("{}", bytes_to_string(hidden.clone()));

    let printable_ascii: Vec<u8> = (0..=255).collect();
    println!("{}", bytes_to_string(printable_ascii.clone()));
    for char in printable_ascii {
        let result = bytes_to_string(xor_with_byte(&hidden, char));
        println!("{}", result);
        if result.starts_with("crypto") {
            return result;
        }
    }

    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorite_byte() {
        assert_eq!("crypto{0x10_15_my_f4v0ur173_by7e}", favorite_byte())
    }
}
