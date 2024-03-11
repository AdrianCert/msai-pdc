pub mod cli;

use clap::Parser;
use cli::CliArguments;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn process_request(request: &str) {
    // Process the request and return a response
    // based on the provided data and protocol
    // implementation logic goes here
    // unimplemented!()
    println!("{}", request);
}

fn handler_tcp_server_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let request = String::from_utf8_lossy(&buffer[..n]);
                process_request(&request);
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}

fn main_tcp_server_stream() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        let stream = stream.expect("Failed to establish connection");
        thread::spawn(move || {
            handler_tcp_server_stream(stream);
        });
    }
    Ok(())
}

fn handler_tcp_client_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let request = String::from_utf8_lossy(&buffer[..n]);
                process_request(&request);
                stream.write_all(b"ACK").unwrap(); // Send ACK back to sender
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}

fn main_tcp_client_stream() -> io::Result<()> {
    Ok(())
}

fn main_tcp_client_sync() -> io::Result<()> {
    Ok(())
}

fn main_udp_server_stream() -> io::Result<()> {
    Ok(())
}

fn main_udp_server_sync() -> io::Result<()> {
    Ok(())
}

fn main_udp_client_stream() -> io::Result<()> {
    Ok(())
}

fn main_udp_client_sync() -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    let args = CliArguments::parse();
    println!("{args:?}");

    match (args.role, args.protocol, args.mechanism) {
        (cli::values::Role::Client, cli::values::Protocol::UDP, cli::values::Mechanism::Stream) => {
            main_udp_client_stream()
        }
        (cli::values::Role::Client, cli::values::Protocol::UDP, cli::values::Mechanism::Sync) => {
            main_udp_client_sync()
        }
        (cli::values::Role::Client, cli::values::Protocol::TCP, cli::values::Mechanism::Stream) => {
            main_tcp_client_stream()
        }
        (cli::values::Role::Client, cli::values::Protocol::TCP, cli::values::Mechanism::Sync) => {
            main_tcp_client_sync()
        }
        (cli::values::Role::Server, cli::values::Protocol::UDP, cli::values::Mechanism::Stream) => {
            main_udp_server_stream()
        }
        (cli::values::Role::Server, cli::values::Protocol::UDP, cli::values::Mechanism::Sync) => {
            main_udp_server_sync()
        }
        (cli::values::Role::Server, cli::values::Protocol::TCP, cli::values::Mechanism::Stream) => {
            main_tcp_server_stream()
        }
        (cli::values::Role::Server, cli::values::Protocol::TCP, cli::values::Mechanism::Sync) => {
            main_tcp_server_sync()
        }
    }
}
