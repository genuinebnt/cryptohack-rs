use crate::utils::bytes::bytes_to_string;
use crate::utils::hex::hex_to_bytes;
use crate::utils::xor::xor;

pub fn you_either_know_xor() -> String {
    let hidden =
        "0e0b213f26041e480b26217f27342e175d0e070a3c5b103e2526217f27342e175d0e077e263451150104";
    let hidden = hex_to_bytes(hidden);

    bytes_to_string(xor(&hidden, "myXORkey".as_bytes()))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_favorite_byte() {
        assert_eq!("crypto{1f_y0u_Kn0w_En0uGH_y0u_Kn0w_1t_4ll}", you_either_know_xor())
    }
}
