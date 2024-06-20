use crate::utils::hex::hex_to_bytes;
use base64::prelude::*;

pub fn base64() -> String {
    let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let bytes = hex_to_bytes(input);
    BASE64_STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        assert_eq!("crypto/Base+64+Encoding+is+Web+Safe/", base64())
    }
}
