use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";
const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;
const CONFIG_STANDARD: bincode::config::Configuration = bincode::config::standard();

// helper function by rust documentation, to convert the current time into unix time
fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Encode, Decode)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128, // To be converted from a UUID
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Encode, Decode)]
pub enum CollectorResponseV1 {
    Ack(u32),
}

pub fn encode_response_v1(command: CollectorResponseV1) -> Vec<u8> {
    bincode::encode_to_vec(command, CONFIG_STANDARD).unwrap()
}

pub fn decode_response_v1(bytes: &[u8]) -> CollectorResponseV1 {
    bincode::decode_from_slice(bytes, CONFIG_STANDARD).unwrap().0
}

pub fn encode_v1(command: &CollectorCommandV1) -> Vec<u8> {
    // let json = serde_json::to_string(&command).unwrap();
    // let json_bytes = json.as_bytes();
    // let crc = crc32fast::hash(&json_bytes);
    // let payload_size = json_bytes.len() as u32;
    let payload_bytes = bincode::encode_to_vec(command, CONFIG_STANDARD).unwrap();
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;
    let timestamp = unix_now();

    // Encode into bytes
    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    // result.extend_from_slice(json_bytes);
    result.extend_from_slice(&payload_bytes);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}

pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let payload = &bytes[12..12 + payload_size as usize];
    let crc = u32::from_be_bytes([
        bytes[12 + payload_size as usize],
        bytes[13 + payload_size as usize],
        bytes[14 + payload_size as usize],
        bytes[15 + payload_size as usize],
    ]);

    // Verify the magic number
    assert_eq!(magic_number, MAGIC_NUMBER);

    // Verify the version number
    assert_eq!(version_number, VERSION_NUMBER);

    // Verify the CRC
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc);

    // Decode the payload
    (timestamp, bincode::decode_from_slice(payload, CONFIG_STANDARD).unwrap().0)
    // (timestamp, serde_json::from_slice(payload).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {        
        let command = CollectorCommandV1::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };

        let encoded = encode_v1(&command);
        println!("{:?}", encoded);
        let (timestamp, decoded) = decode_v1(&encoded);
        
        assert_eq!(decoded, command);
        assert!(timestamp > 0);
    }

    #[test]
    fn test_encode_decode_response() {
        let response = CollectorResponseV1::Ack(123);
        let encoded = encode_response_v1(response.clone());
        let decoded = decode_response_v1(&encoded);
        assert_eq!(decoded, response);
    }
}

// https://github.com/thebracket/ArdanUltimateRust-5Days/blob/main/05-Server/Planning.md
// https://github.com/thebracket/ArdanUltimateRust-5Days/blob/main/05-Server/SharedDataStructures.md

// cargo add crc32fast
// cargo test -- --nocapture
// cargo add bincode -F i128
