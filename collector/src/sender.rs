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

/// Send a queue of commands to the data collector thread and wait for a response from the server
pub fn send_queue(queue: &mut VecDeque<Vec<u8>>, collector_id: u128) -> Result<(), CollectorError> {
    // Connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::ServerConnection)?;

    // Send every queue item
    let mut buf = vec![0u8; 512];
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::DataSend);
        }
        let bytes_read = stream
            .read(&mut buf)
            .map_err(|_| CollectorError::DataReceive)?;
        if bytes_read == 0 {
            queue.push_front(command);
            return Err(CollectorError::DataReceive);
        }
        let ack = decode_response(&buf[0..bytes_read]);
        if ack != CollectorResponse::Ack {
            queue.push_front(command);
            return Err(CollectorError::DataReceive);
        } else {
            // Comment this out for production
            println!("Ack received");
        }
    }

    // Ask for work
    let bytes = shared::encode(&shared::CollectorCommand::RequestWork(collector_id));
    if stream.write_all(&bytes).is_err() {
        return Err(CollectorError::DataSend);
    }
    let bytes_read = stream
        .read(&mut buf)
        .map_err(|_| CollectorError::DataReceive)?;
    if bytes_read == 0 {
        return Err(CollectorError::DataReceive);
    }
    let work = decode_response(&buf[0..bytes_read]);
    match work {
        CollectorResponse::NoWork => {}
        CollectorResponse::Task(task) => {
            println!("Task received: {task:?}");
        }
        _ => {}
    }

    Ok(())
}
