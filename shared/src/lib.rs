use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// The address of the data collector
pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";
/// The magic number for the data collector
const MAGIC_NUMBER: u16 = 1234;
/// The version number for the data collector
const VERSION_NUMBER: u16 = 1;

/// Get the current timestamp in seconds.
fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

/// Commands for the data collector.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommand {
    SubmitData {
        collector_id: u128, // To be converted from a UUID
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
    RequestWork(u128), // Contains the collector id
}

/// Responses from the data collector.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorResponse {
    Ack,
    NoWork,
    Task(TaskType),
}

/// Types of tasks that the data collector can perform
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskType {
    Shutdown,
}

/// Encode a collector command.
pub fn encode(command: &CollectorCommand) -> Vec<u8> {
    let payload_bytes = bincode::serialize(command).unwrap();
    // let json = serde_json::to_string(&command).unwrap();
    // let json_bytes = json.as_bytes();
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;
    let timestamp = unix_now();

    // Encode into bytes
    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(&payload_bytes);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}

/// Decode a collector command.
pub fn decode(bytes: &[u8]) -> (u32, CollectorCommand) {
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
    (timestamp, bincode::deserialize(payload).unwrap())
}

/// Encode a collector response.
pub fn encode_response(command: CollectorResponse) -> Vec<u8> {
    bincode::serialize(&command).unwrap()
}

/// Decode a collector response.
pub fn decode_response(bytes: &[u8]) -> CollectorResponse {
    bincode::deserialize(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the encoding and decoding works.
    #[test]
    fn test_encode_decode() {
        let command = CollectorCommand::SubmitData {
            collector_id: 1234,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };
        let encoded = encode(&command);
        let (timestamp, decoded) = decode(&encoded);
        assert_eq!(decoded, command);
        assert!(timestamp > 0);
    }

    /// Test that the encoding and decoding of responses works.
    #[test]
    fn test_encode_decode_response() {
        let response = CollectorResponse::Ack;
        let encoded = encode_response(response.clone());
        let decoded = decode_response(&encoded);
        assert_eq!(decoded, response);
    }
}
