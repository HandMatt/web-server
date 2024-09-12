use shared::{CollectorCommand, DATA_COLLECTOR_ADDRESS};
use std::io::Write;

/// Send an encoded command to the data collector via TCP
pub fn send_command(command: CollectorCommand) {
    let bytes = shared::encode(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS).unwrap();
    stream.write_all(&bytes).unwrap();
}
