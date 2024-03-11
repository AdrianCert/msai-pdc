use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::net::{SocketAddr, UdpSocket};

const SERVER_MSG_SEPARATOR: u8 = 0x1f; // ASCII Unit Separator

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    let args: Vec<String> = vec![
        "client".to_string(),
        "127.0.0.1:8080".to_string(),
        "d:/dev/msai-pdc/proto-bench/readme-2.md".to_string(),
    ];
    // if args.len() != 3 {
    //     println!("Usage: {} <server> <file>", args[0]);
    //     return Ok(());
    // }

    let server_addr: SocketAddr = args[1].parse()?;
    let file_path = &args[2];
    let message_size: u16 = 0xffff;

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(server_addr)?;

    let file = File::open(file_path)?;
    let mut file_reader = BufReader::with_capacity(message_size.into(), file);

    let mut message = file_path.as_bytes().to_vec();
    message.extend(".recv".as_bytes());
    message.push(SERVER_MSG_SEPARATOR);
    socket.send(&message)?;

    file_reader.read_exact(&mut message)?;

    // message.extend(buffer);
    socket.send(&message)?;

    let mut buffer = [0; 1024];
    let (size, _) = socket.recv_from(&mut buffer)?;
    let ack = std::str::from_utf8(&buffer[..size])?;
    println!("Received ACK: {}", ack);

    Ok(())
}
