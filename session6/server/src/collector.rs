use std::net::SocketAddr;

use shared_data::{decode_v1, encode_response_v1, CollectorCommandV1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};
use sqlx::{Pool, Sqlite};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

// where the data is coming in
pub async fn data_collector(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, cnn.clone()));
    }
}

async fn new_connection(
    mut socket: TcpStream,
    address: SocketAddr,
    cnn: Pool<Sqlite>,
    // every connection needs to talk to database;
    // if run out of connection there will be some delay involved waiting for the next one to be available becoming self pacing
    // have to make sure that the database is up for the job
) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024]; // 1 KB
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        // println!("Received {n} bytes");
        let received_data = decode_v1(&buf[0..n]);
        // println!("Received data: {received_data:?}");

        match received_data {
            (
                timestamp,
                CollectorCommandV1::SubmitData {
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
                    let ack = CollectorResponseV1::Ack(0);
                    let response_bytes = encode_response_v1(ack);
                    socket.write_all(&response_bytes).await.unwrap();
                }
            }
        }
    }
}
