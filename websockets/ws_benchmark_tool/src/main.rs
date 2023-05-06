use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::mpsc;
use tokio::task;
use futures_util::StreamExt;
use futures_util::sink::SinkExt; // Import SinkExt

#[tokio::main]
async fn main() {
    let url = "ws://localhost:12345"; // Change this to the WebSocket server URL
    let num_clients = 1000; // Change this to the number of clients
    let num_messages = 100; // Change this to the number of messages

    let start_time = Instant::now();
    let stats = Arc::new(Mutex::new(Vec::new()));

    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut tasks = Vec::new();
    for _ in 0..num_clients {
        let stats = Arc::clone(&stats);
        let tx = tx.clone();
        let task = task::spawn(async move {
            let (mut ws_stream, _) = connect_async(url)
                .await
                .expect("Failed to connect to WebSocket server");
            let mut timings = Vec::with_capacity(num_messages);

            for _ in 0..num_messages {
                let start_time = Instant::now();
                ws_stream.send(Message::Text("test message".to_string())).await.unwrap();
                let _msg = ws_stream.next().await.unwrap().unwrap();
                let elapsed = start_time.elapsed();
                timings.push(elapsed);
            }

            let _ = tx.send(timings);
        });
        tasks.push(task);
    }

    for _ in 0..num_clients {
        if let Some(timings) = rx.recv().await {
            let mut stats = stats.lock().unwrap();
            stats.push(timings);
        }
    }

    let elapsed = start_time.elapsed();
    print_stats(&*stats.lock().unwrap(), elapsed); // Dereference the MutexGuard<Vec<_>>
}

fn print_stats(stats: &Vec<Vec<Duration>>, elapsed: Duration) {
    let mut total_messages = 0;
    let mut total_time = Duration::from_secs(0);

    for client_stats in stats {
        total_messages += client_stats.len();
        for &time in client_stats {
            total_time += time;
        }
    }

    let avg_time = total_time / (total_messages as u32);
    let throughput = total_messages as f64 / elapsed.as_secs_f64();

    println!("Total clients: {}", stats.len());
    println!("Total messages: {}", total_messages);
    println!("Total elapsed time: {:?}", elapsed);
    println!("Average time per message: {:?}", avg_time);
    println!("Throughput (messages/s): {:.2}", throughput);
}
