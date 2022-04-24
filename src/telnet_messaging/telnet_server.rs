use std::io::{
    prelude::Read,
    // prelude::Write,
    ErrorKind
};
use std::collections::HashMap;
use std::net::TcpStream;

use std::thread::{
    self,
    // JoinHandle
};

use std::time::Duration;
use std::sync::mpsc::Sender;

use crate::telnet_messaging::telnet_client::TelnetClient;


pub struct MessageWrapper {
    pub sender_id: u32,
    pub message: Vec<u8>,
}

#[derive(Debug)]
pub struct TelnetServer {
    pub connected_clients:  HashMap<u32, TelnetClient>,
    connected_id_counter: u32
}

impl TelnetServer {
    pub fn new() -> TelnetServer {
        TelnetServer {
            connected_clients: HashMap::new(),
            connected_id_counter: 1
        }
    }
    pub fn register_client(&mut self, client: TcpStream, sender: Sender<MessageWrapper>) {
        let sender_id = self.connected_id_counter;
        client.set_nonblocking(true).unwrap();

        let cloned_client = client.try_clone().unwrap();

        let join_handle = thread::spawn(move || {
            Self::handle_connection(cloned_client, sender_id, sender);
        });
        println!("{:?}", self);
        self.connected_clients.insert(
            sender_id,
            TelnetClient {
                join_handle: None,
                stream: client
            }
        );
        self.connected_id_counter += 1;
    }

    fn handle_connection(mut stream: TcpStream, sender_id: u32, sender: Sender<MessageWrapper>) {
        let mut buffer = [0; 1024];
        stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
        loop {
            match stream.read(&mut buffer) {
                Ok(num_bytes) => {
                    if num_bytes > 0 {
                        println!("{:?}", buffer.get(0..num_bytes));
                        match buffer.get(0..num_bytes) {
                            Some(returned) => sender.send(MessageWrapper {
                                sender_id,
                                message: returned.to_vec(),
                            }).unwrap(),
                            None => {}
                        };

                    } else if num_bytes == 0 {
                        // Stream is closed
                        drop(sender);
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
        }
    }
}
