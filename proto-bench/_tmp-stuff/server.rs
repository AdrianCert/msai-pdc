use std::fs::File;
use std::io::{self, Write};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    async fn new(addr: SocketAddr) -> io::Result<Self> {
        let socket = UdpSocket::bind(addr).await?;
        Ok(Self { socket })
    }

    async fn receive_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;

        let mut buffer = [0; 1024]; // Buffer size for receiving data

        loop {
            let (size, _) = self.socket.recv_from(&mut buffer).await?;
            if size == 0 {
                break; // End of file
            }
            file.write_all(&buffer[..size])?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = "127.0.0.1:8080".parse().unwrap();
    let file_path = "/path/to/save/file.txt";

    let server = UdpServer::new(server_addr).await?;
    server.receive_file(file_path).await?;

    Ok(())
}
