use crate::utils::xor as xor_lib;

pub fn xor() -> String {
    let input = "label";
    String::from_utf8(xor_lib(input.as_bytes(), 13)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!("aloha", xor())
    }
}
