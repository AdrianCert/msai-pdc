use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::time::sleep;

async fn handle_udp_client(socket: UdpSocket, file_path: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; BUFFER_SIZE];
    let mut total_messages = 0;
    let mut total_bytes = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        socket.send(&buffer[..bytes_read]).await?;
        total_messages += 1;
        total_bytes += bytes_read;
    }

    println!("UDP: Number of messages read: {}", total_messages);
    println!("UDP: Number of bytes read: {}", total_bytes);

    Ok(())
}

async fn handle_tcp_client(mut stream: TcpStream, file_path: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0; BUFFER_SIZE];
    let mut total_messages = 0;
    let mut total_bytes = 0;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        stream.write_all(&buffer[..bytes_read]).await?;
        total_messages += 1;
        total_bytes += bytes_read;
    }

    println!("TCP: Number of messages read: {}", total_messages);
    println!("TCP: Number of bytes read: {}", total_bytes);

    Ok(())
}

async fn handle_udp_server(socket: UdpSocket) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; BUFFER_SIZE];
    let mut total_messages = 0;
    let mut total_bytes = 0;

    loop {
        let (bytes_read, _) = socket.recv_from(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        total_messages += 1;
        total_bytes += bytes_read;
    }

    println!("UDP: Number of messages read: {}", total_messages);
    println!("UDP: Number of bytes read: {}", total_bytes);

    Ok(())
}

async fn handle_tcp_server(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; BUFFER_SIZE];
    let mut total_messages = 0;
    let mut total_bytes = 0;

    loop {
        let bytes_read = stream.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }

        total_messages += 1;
        total_bytes += bytes_read;
    }

    println!("TCP: Number of messages read: {}", total_messages);
    println!("TCP: Number of bytes read: {}", total_bytes);

    Ok(())
}

async fn run_client(protocol: &str, file_path: &str, address: &str) -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    match protocol {
        "UDP" => {
            let socket = UdpSocket::bind("0.0.0.0:0").await?;
            socket.connect(address).await?;
            handle_udp_client(socket, file_path.to_string()).await?;
        }
        "TCP" => {
            let stream = TcpStream::connect(address).await?;
            handle_tcp_client(stream, file_path.to_string()).await?;
        }
        _ => {
            println!("Unsupported protocol: {}", protocol);
            return Ok(());
        }
    }

    let transmission_time = start_time.elapsed().as_secs_f64();
    println!("Transmission time: {:.2} seconds", transmission_time);

    Ok(())
}

async fn run_server(protocol: &str, address: &str) -> Result<(), Box<dyn Error>> {
    match protocol {
        "UDP" => {
            let socket = UdpSocket::bind(address).await?;
            handle_udp_server(socket).await?;
        }
        "TCP" => {
            let listener = TcpListener::bind(address).await?;
            let (stream, _) = listener.accept().await?;
            handle_tcp_server(stream).await?;
        }
        _ => {
            println!("Unsupported protocol: {}", protocol);
            return Ok(());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let protocol = "UDP"; // Specify the protocol here (UDP or TCP)
    let file_path = "/path/to/file"; // Specify the file path here
    let address = "127.0.0.1:8080"; // Specify the server address here

    let server_handle = tokio::spawn(async move {
        run_server(protocol, address).await.unwrap();
    });

    sleep(Duration::from_secs(1)).await; // Wait for the server to start

    run_client(protocol, file_path, address).await.unwrap();

    server_handle.await.unwrap();

    Ok(())
}
