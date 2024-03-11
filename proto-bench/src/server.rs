pub mod cli;
pub mod models;

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::str;
use std::{error::Error, io::Write};

use clap::Parser;
use cli::CliArguments;
use models::{Mechanism, Protocol};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use std::net::{TcpListener, TcpStream, UdpSocket};

#[derive(Debug)]
enum ServerState {
    WaitForFile,
    Transferring,
    Done,
}

#[derive(Debug)]
struct Server {
    protocol: Protocol,
    mechanism: Mechanism,
    message_length: u16,
    addr: SocketAddr,
    // socket: ServerSocket,
}

#[derive(Debug)]
struct SessionStats {
    count: u8,
    bytes: u64,
}

impl SessionStats {
    pub fn new() -> SessionStats {
        SessionStats { count: 0, bytes: 0 }
    }
}

const SERVER_MSG_SEPARATOR: u8 = 0x1f; // ASCII Unit Separator

impl Server {
    pub fn new(protocol: Protocol, mechanism: Mechanism, size: u16, addr: SocketAddr) -> Server {
        Server {
            protocol,
            mechanism,
            message_length: size,
            addr,
            // socket: ServerSocket::None,
        }
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        match self.protocol {
            Protocol::TCP => self.start_tcp(),
            Protocol::UDP => self.start_udp(),
        }
    }

    pub fn start_tcp(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn start_udp(&mut self) -> Result<(), Box<dyn Error>> {
        let socket = UdpSocket::bind(self.addr)?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;
        let x = socket.recv().unwrap();
        let mut buffer = [0; 0xffff]; // Buffer size for receiving data
        let mut file: Option<File> = None;
        let mut state = ServerState::WaitForFile;
        let mut stats = SessionStats::new();
        let mut timer: Option<Instant> = None;

        loop {
            match socket.recv_from(&mut buffer) {
                Ok((size, addr)) => {}
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        println!("No data available");
                    } else {
                        println!("Error: {:?}", e);
                    }
                }
            }
            let (size, addr) = socket
                .recv_from(&mut buffer)
                .expect("Failed to receive data");

            if size == 0 {
                state = ServerState::Done;
            }

            if self.mechanism == Mechanism::Sync {
                socket
                    .send_to("ACK".as_bytes(), addr)
                    .expect("Failed to send ACK to client");
            }

            stats.bytes += size as u64;
            stats.count += 1;

            match state {
                ServerState::WaitForFile => {
                    timer = Some(Instant::now());

                    let mut splitter = buffer.splitn(2, |chr| SERVER_MSG_SEPARATOR == *chr);
                    let filepath = str::from_utf8(splitter.next().unwrap())?;
                    file = Some(File::create(filepath)?);

                    let buf = splitter.next().unwrap()[..{ size - filepath.len() - 1 }].to_vec();

                    file.as_ref().unwrap().borrow_mut().write_all(&buf)?;

                    state = ServerState::Transferring;
                }
                ServerState::Transferring => {
                    file.as_ref()
                        .unwrap()
                        .borrow_mut()
                        .write_all(&buffer[..size])?;
                }
                ServerState::Done => {
                    let transmit_time: Duration = timer.as_ref().unwrap().elapsed();

                    println!("Transmit time:: {:?}", transmit_time);
                    println!("Address:: {:?}", addr);
                    println!("Messages received:: {:?}", stats.count);
                    println!("Bytes received:: {:?}", stats.bytes);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let cli_arguments = CliArguments::parse();
    let server_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    // let mut server = Server::new(
    //     cli_arguments.protocol,
    //     cli_arguments.mechanism,
    //     cli_arguments.size,
    //     server_address,
    // );
    let mut server = Server::new(Protocol::UDP, Mechanism::Sync, 0xffff, server_address);

    server.start()
}
