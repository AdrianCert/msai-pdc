use std::fs::File;
use std::io::Read;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

fn main() {
    // Server address
    let server_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    // Read data from file
    let chunk_size: u64 = 1024;
    let mut file = File::open(r"D:\media\movies\The.Hunger.Games.The.Ballad.of.Songbirds.and.Snakes.2023.2160p.WEB-DL.DDP5.1.Atmos.DV.HDR.H.265-FLUX.mkv").expect("Failed to open file");
    let mut data = vec![0; chunk_size as usize];

    // Chunk size

    // Get the file size
    let file_size = file
        .metadata()
        .expect("Failed to retrieve file metadata")
        .len();

    // Number of chunks
    let num_chunks = (file_size + chunk_size - 1) / chunk_size;

    // Start the timer
    let start_time = Instant::now();

    // Send the data in chunks
    for _ in 0..num_chunks {
        file.read_exact(&mut data).expect("Failed to read file");

        // Send the chunk to the server
        socket
            .send_to(&data, server_address)
            .expect("Failed to send data");

        // Uncomment the line below if you want to introduce a delay between chunks
        // std::thread::sleep(Duration::from_millis(10));
    }

    // Calculate the transmit time
    let transmit_time = start_time.elapsed();

    // Print the transmit time, number of messages, and number of bytes sent
    println!("Transmit Time: {:?}", transmit_time);
    println!("Number of Messages: {}", num_chunks);
    println!("Number of Bytes Sent: {}", data.len());
}
