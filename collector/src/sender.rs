use crate::errors::CollectorError;
use shared::{decode_response, CollectorResponse, DATA_COLLECTOR_ADDRESS};
use std::{
    collections::VecDeque,
    io::{Read, Write},
};

/// Send a command to the data collector thread and wait for a response from the server
/*pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream
        .write_all(bytes)
        .map_err(|_| CollectorError::UnableToSendData)?;

    Ok(())
}*/

pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::ServerConnection)?;

    // Send every queue item
    let mut buf = vec![0u8; 512];
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::DataTransmission);
        }
        let bytes_read = stream
            .read(&mut buf)
            .map_err(|_| CollectorError::DataRetrieval)?;
        if bytes_read == 0 {
            queue.push_front(command);
            return Err(CollectorError::DataRetrieval);
        }
        let ack = decode_response(&buf[0..bytes_read]);
        if ack != CollectorResponse::Ack {
            queue.push_front(command);
            return Err(CollectorError::DataRetrieval);
        } else {
            // Comment this out for production
            println!("Ack received");
        }
    }

    Ok(())
}
