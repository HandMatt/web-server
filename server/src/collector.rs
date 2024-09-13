use crate::commands::get_commands;
use shared::{
    decode, encode_response, CollectorCommand, CollectorResponse, DATA_COLLECTOR_ADDRESS,
};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// Start the data collector thread and listen for connections
pub async fn data_collector(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let cnn = cnn.clone();
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, cnn));
    }
}

/// Handle a new connection to the data collector
async fn new_connection(mut socket: TcpStream, address: SocketAddr, cnn: Pool<Sqlite>) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        println!("Received {n} bytes");
        let received_data = decode(&buf[0..n]);
        println!("Received data: {received_data:?}");

        match received_data {
            (_timestamp, CollectorCommand::RequestWork(collector_id)) => {
                if let Some(commands) = get_commands(collector_id) {
                    let work = CollectorResponse::Task(commands);
                    let bytes = encode_response(work);
                    socket.write_all(&bytes).await.unwrap();
                } else {
                    let no_work = CollectorResponse::NoWork;
                    let bytes = encode_response(no_work);
                    socket.write_all(&bytes).await.unwrap();
                }
            }
            (
                timestamp,
                CollectorCommand::SubmitData {
                    collector_id,
                    total_memory,
                    used_memory,
                    average_cpu_usage,
                },
            ) => {
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();

                let result = sqlx::query("INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu) VALUES ($1, $2, $3, $4, $5)")
                    .bind(collector_id)
                    .bind(timestamp)
                    .bind(total_memory as i64)
                    .bind(used_memory as i64)
                    .bind(average_cpu_usage)
                    .execute(&cnn)
                    .await;

                if result.is_err() {
                    println!("Error inserting data into the database: {result:?}");
                } else {
                    let ack = CollectorResponse::Ack;
                    let bytes = encode_response(ack);
                    socket.write_all(&bytes).await.unwrap();
                }
            }
        }
    }
}
