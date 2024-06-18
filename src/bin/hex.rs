use cryptohack_rs::utils::{bytes_to_string, hex_to_bytes};

fn main() {
    let hex_string = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let bytes = hex_to_bytes(&hex_string);
    let result = bytes_to_string(bytes);
    println!("{:?}", result);
}
