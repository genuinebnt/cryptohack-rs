use crate::utils::bytes_to_string;

pub fn ascii() -> String {
    let input: Vec<u8> = vec![
        99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98,
        108, 51, 125,
    ];

    bytes_to_string(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let result = ascii();
        assert_eq!(result, "crypto{ASCII_pr1nt4bl3}");
    }
}
