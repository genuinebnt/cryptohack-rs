use cryptohack_rs::utils::bytes_to_string;

fn main() {
    let input: Vec<u8> = vec![
        99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98,
        108, 51, 125,
    ];

    println!("{}", bytes_to_string(input));
}
