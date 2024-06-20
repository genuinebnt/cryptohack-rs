use crate::utils::bytes::bytes_to_string;
use crate::utils::hex::hex_to_bytes;
use crate::utils::xor::xor;

pub fn xor_properties() -> String {
    let key1 = "a6c8b6733c9b22de7bc0253266a3867df55acde8635e19c73313";
    let key1_xor_key2 = "37dcb292030faa90d07eec17e3b1c6d8daf94c35d4c9191a5e1e";

    let key2 = xor(&hex_to_bytes(&key1), &hex_to_bytes(&key1_xor_key2));
    let key2_xor_key3 = "c1545756687e7573db23aa1c3452a098b71a7fbf0fddddde5fc1";

    let key3 = xor(&key2, &hex_to_bytes(&key2_xor_key3));
    let flag_xor_key1_key2_key3 = "04ee9855208a2cd59091d04767ae47963170d1660df7f56f5faf";

    let flag = xor(
        &xor(
            &xor(&hex_to_bytes(&flag_xor_key1_key2_key3), &hex_to_bytes(key1)),
            &key2,
        ),
        &key3,
    );

    bytes_to_string(flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_properties() {
        assert_eq!("crypto{x0r_i5_ass0c1at1v3}", xor_properties());
    }
}
