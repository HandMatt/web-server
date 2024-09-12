use shared::CollectorCommand;
use std::collections::VecDeque;
mod data_collector;
mod errors;
mod sender;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommand>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    // Listen for commands to send
    let mut send_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = shared::encode(&command);
        println!("Encoded: {} bytes", encoded.len());
        send_queue.push_back(encoded);
        let _ = sender::send_queue(&mut send_queue);
    }
}
