use std::io::{
    prelude::Read,
    ErrorKind
};
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Duration;
use std::thread::{
    self,
    JoinHandle
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut server = TelnetServer::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server.register_client(stream)
    }
}
#[derive(Debug)]
pub struct TelnetServer {
}

impl TelnetServer {
    pub fn new() -> TelnetServer {
        TelnetServer {
        }
    }
    pub fn register_client(&mut self, client: TcpStream) {
        client.set_nonblocking(true).unwrap();
        let join_handle = thread::spawn(|| {
            Self::handle_connection(client);
        });
        println!("{:?}", self);
        join_handle.join().unwrap();
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
        loop {
            match stream.read(&mut buffer) {
                Ok(num_bytes) => {
                    if num_bytes > 0 {
                        println!("{:?}", buffer.get(0..num_bytes));
                    } else if num_bytes == 0 {
                        // Stream is closed
                        println!("Client left. Closing connection...");
                        break;
                    }
                }
                Err(error) => match error.kind() {
                    ErrorKind::WouldBlock => {
                        // retry
                    },
                    other_error => {
                        panic!("Problem opening the file: {:?}", other_error)
                    }
                }
            };
            /* match stream.peek(&mut buffer) {
                Err(_) => break,
                Ok(_) => {}
            } */
        }
    }

}
