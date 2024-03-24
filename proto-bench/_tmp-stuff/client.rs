use std::io::{Read, Write};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::time::{Duration, Instant};
use std::time::{Duration, Instant};

fn main() {
    // Define the data sizes to transfer
    let data_sizes = vec![
        500 * 1024 * 1024,  // 500MB
        1024 * 1024 * 1024, // 1GB
        1500000,            // 1.5 million WhatsApp messages
        4000 * 1024,        // 4000 photos
        10000,              // 10,000 emails
        1024 * 1024 * 1024, // Reuse a large buffer
    ];

    // Define the conditions to test
    let conditions = vec![
        "Local network",
        "High latency network",
        "Low bandwidth network",
    ];

    // Iterate over the data sizes and conditions
    for size in &data_sizes {
        for condition in &conditions {
            // Start the timer
            let start = Instant::now();

            // Simulate data transfer based on the condition
            match condition {
                "Local network" => transfer_local_network(*size),
                "High latency network" => transfer_high_latency_network(*size),
                "Low bandwidth network" => transfer_low_bandwidth_network(*size),
                _ => println!("Unknown condition: {}", condition),
            }

            // Calculate the elapsed time
            let elapsed = start.elapsed();

            // Print the result
            println!(
                "Transferred {} bytes under {} condition in {} seconds",
                size,
                condition,
                elapsed.as_secs_f64()
            );
        }
    }
}

fn transfer_local_network(size: usize) {
    // Create a TCP listener and accept a connection
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let (mut stream, _) = listener.accept().unwrap();

    // Create a buffer with the specified size
    let mut buffer = vec![0; size];

    // Read from the stream to simulate receiving data
    stream.read_exact(&mut buffer).unwrap();
}

fn transfer_high_latency_network(size: usize) {
    // Create a TCP stream and connect to a remote server
    let mut stream = TcpStream::connect("example.com:8080").unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    // Create a buffer with the specified size
    let buffer = vec![0; size];

    // Write to the stream to simulate sending data
    stream.write_all(&buffer).unwrap();
}

fn transfer_low_bandwidth_network(size: usize) {
    // Create a TCP stream and connect to a remote server
    let mut stream = TcpStream::connect("example.com:8080").unwrap();
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    // Create a buffer with the specified size
    let buffer = vec![0; size];

    // Write to the stream to simulate sending data
    stream.write_all(&buffer).unwrap();
}

fn transfer_udp(size: usize) {
    // Create a UDP socket and bind it to a local address
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    // Create a buffer with the specified size
    let buffer = vec![0; size];

    // Send the data in chunks
    let chunk_size = 1024;
    let num_chunks = size / chunk_size;
    for _ in 0..num_chunks {
        socket
            .send_to(&buffer[..chunk_size], "example.com:8080")
            .unwrap();
    }

    // Calculate the total number of bytes sent
    let total_bytes_sent = num_chunks * chunk_size;

    // Print the result
    println!(
        "Transmitted {} bytes in {} messages",
        total_bytes_sent, num_chunks
    );
}
