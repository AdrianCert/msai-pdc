use std::fs::File;
use std::io::{self, Read};
use std::net::{TcpStream, UdpSocket};

struct FileTransferClient {
    file_path: String,
    chunk_size: usize,
    use_udp: bool,
}

impl FileTransferClient {
    fn new(file_path: String, chunk_size: usize, use_udp: bool) -> Self {
        FileTransferClient {
            file_path,
            chunk_size,
            use_udp,
        }
    }

    fn transfer_file(&self) -> io::Result<()> {
        let mut file = File::open(&self.file_path)?;
        let mut buffer = vec![0; self.chunk_size];

        if self.use_udp {
            let socket = UdpSocket::bind("0.0.0.0:0")?;
            socket.connect("127.0.0.1:8080")?;

            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }

                socket.send(&buffer[..bytes_read])?;

                let mut ack_buffer = [0; 1];
                socket.recv(&mut ack_buffer)?;
                // Process acknowledgement message

                // Wait for the message before sending the next chunk
                let mut wait_buffer = [0; 1];
                socket.recv(&mut wait_buffer)?;
            }
        } else {
            let mut stream = TcpStream::connect("127.0.0.1:8080")?;

            loop {
                let bytes_read = file.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }

                stream.write_all(&buffer[..bytes_read])?;

                let mut ack_buffer = [0; 1];
                stream.read_exact(&mut ack_buffer)?;
                // Process acknowledgement message

                // Wait for the message before sending the next chunk
                let mut wait_buffer = [0; 1];
                stream.read_exact(&mut wait_buffer)?;
            }
        }

        Ok(())
    }
}

fn main() {
    let client = FileTransferClient::new("/path/to/file".to_string(), 1024, true);
    client.transfer_file().unwrap();
}
