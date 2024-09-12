use crate::errors::CollectorError;
use shared::{CollectorCommand, DATA_COLLECTOR_ADDRESS};
use std::io::Write;

/// Send a command to the data collector thread and wait for a response from the server
pub fn send_command(command: CollectorCommand) -> Result<(), CollectorError> {
    let bytes = shared::encode(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(&bytes)
        .map_err(|_| CollectorError::UnableToSendData)?;

    Ok(())
}
