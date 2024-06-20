use std::{
    io::{Read, Write},
    net::TcpStream,
};

use base64::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::{bytes_to_string, hex_to_bytes, rot13_decode};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    r#type: String,
    encoded: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataAsBytes {
    r#type: String,
    encoded: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DecodedData {
    decoded: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Flag {
    flag: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseError {
    error: String,
}

#[derive(Debug)]
pub enum ChallengeError {
    ServerError(String),
    SerdeError(String),
    SocketConnectError(String),
    SocketReadError(String),
    SocketWriteError(String),
}

impl std::fmt::Display for ChallengeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerError(e) => write!(f, "Server cannot parse decoded data: {}", e),
            Self::SerdeError(e) => write!(f, "Serde failed to parse: {}", e),
            Self::SocketConnectError(e) => write!(f, "Failed to connect to socket: {}", e),
            Self::SocketReadError(e) => write!(f, "Failed to read from socket: {}", e),
            Self::SocketWriteError(e) => write!(f, "Failed to write to socket: {}", e),
        }
    }
}

fn handle_response(decoded: String, connection: &mut TcpStream) -> Result<(), ChallengeError> {
    let decoded_data = DecodedData { decoded };
    let encoded_response = serde_json::to_string(&decoded_data)
        .map_err(|e| ChallengeError::SerdeError(e.to_string()))?;
    connection
        .write_all(encoded_response.as_bytes())
        .map_err(|e| ChallengeError::SocketWriteError(e.to_string()))?;

    Ok(())
}

pub fn encoding_challenge() -> Result<String, ChallengeError> {
    let mut connection = TcpStream::connect("socket.cryptohack.org:13377")
        .map_err(|e| ChallengeError::SocketConnectError(e.to_string()))?;

    let mut buffer = [0; 512];
    loop {
        let bytes_read = connection
            .read(&mut buffer)
            .map_err(|e| ChallengeError::SocketReadError(e.to_string()))?;
        let response = bytes_to_string(buffer[..bytes_read].to_vec());

        if let Ok(flag) = serde_json::from_str::<Flag>(&response) {
            return Ok(flag.flag);
        }

        if let Ok(parse_error) = serde_json::from_str::<ParseError>(&response) {
            return Err(ChallengeError::ServerError(parse_error.error));
        }

        if let Ok(data) = serde_json::from_str::<DataAsBytes>(&response) {
            let decoded = bytes_to_string(data.encoded);
            println!("Encoded: {}, Decoded: {}", response, decoded);
            handle_response(decoded, &mut connection)?;
            continue;
        }

        let data: Data = serde_json::from_str(&response).map_err(|e| {
            ChallengeError::SerdeError(format!("Error: {} for value {}", e, response))
        })?;

        let decoded = match data.r#type.as_str() {
            "bigint" => bytes_to_string(hex_to_bytes(&&data.encoded[2..])),
            "base64" => bytes_to_string(BASE64_STANDARD.decode(&data.encoded).unwrap()),
            "hex" => bytes_to_string(hex_to_bytes(&data.encoded)),
            "rot13" => bytes_to_string(rot13_decode(&data.encoded)),
            "utf-8" => bytes_to_string(data.encoded.as_bytes().to_owned()),
            _ => unreachable!("No value of type {}", data.r#type),
        };

        println!("Encoded: {}, Decoded: {}", response, decoded);

        handle_response(decoded, &mut connection)?;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encoding_challenge() {
        assert_eq!(
            "crypto{3nc0d3_d3c0d3_3nc0d3}",
            encoding_challenge().unwrap()
        )
    }
}
