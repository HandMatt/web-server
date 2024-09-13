use shared::CollectorCommand;
use std::collections::VecDeque;
mod data_collector;
mod errors;
mod sender;

/// Get the UUID from the file. If it doesn't exist, generate a new one.
fn get_uuid() -> u128 {
    let path = std::path::Path::new("uuid");
    if path.exists() {
        let contents = std::fs::read_to_string(path).unwrap();
        contents.parse::<u128>().unwrap()
    } else {
        let uuid = uuid::Uuid::new_v4().as_u128();
        std::fs::write(path, uuid.to_string()).unwrap();
        uuid
        // provide collector UUID as part of setup in order to reduce bin size
        // 1234567890
    }
}

fn main() {
    let uuid = get_uuid();
    let (tx, rx) = std::sync::mpsc::sync_channel::<CollectorCommand>(1);

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx, uuid);
    });

    // Listen for commands to send
    let mut send_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = shared::encode(&command);
        println!("Encoded: {} bytes", encoded.len());
        if send_queue.len() > 120 {
            // Drop the first entry
            send_queue.pop_front();
        }
        send_queue.push_back(encoded);
        let result = sender::send_queue(&mut send_queue, uuid);
        if result.is_err() {
            println!("{result:?}");
        }
    }
}
