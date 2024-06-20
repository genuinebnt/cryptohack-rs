use crate::utils::bytes::bytes_to_string;
use crate::utils::hex::hex_to_bytes;

pub fn hex() -> String {
    let hex_string = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let bytes = hex_to_bytes(&hex_string);
    bytes_to_string(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let result = hex();
        assert_eq!(result, "crypto{You_will_be_working_with_hex_strings_a_lot}");
    }
}
