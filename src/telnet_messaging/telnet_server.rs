use std::io::{
    // prelude::Read,
    BufRead,
    BufReader,
    ErrorKind
};
use std::collections::HashMap;
use std::net::TcpStream;

use std::thread;

use std::sync::{
    Arc,
    Mutex,
};

use std::time::Duration;
use std::sync::mpsc::Sender;

use crate::telnet_messaging::telnet_client::TelnetClient;


pub struct MessageWrapper {
    pub sender_id: u32,
    pub message: Vec<u8>,
}

#[derive(Debug)]
pub struct TelnetServerInner {
    pub connected_clients:  HashMap<u32, TelnetClient>,
    pub connected_id_counter: u32
}

impl TelnetServer {
    pub fn new() -> TelnetServer {
        TelnetServer {
            inner: Arc::new(Mutex::new(TelnetServerInner {
                connected_clients: HashMap::new(),
                connected_id_counter: 1
            }))
        }
    }
    pub fn register_client(&mut self, client: TcpStream, sender: Sender<MessageWrapper>) -> u32 {
        let sender_id = self.inner.lock().unwrap().connected_id_counter;

        let cloned_client = client.try_clone().unwrap();
        let cloned_server = self.inner.clone();

        thread::spawn(move || {
            Self::handle_connection(cloned_client, sender_id, sender);
            cloned_server.lock().unwrap().connected_clients.remove(&sender_id);
        });
        println!("{:?}", self.inner);
        self.inner.lock().unwrap().connected_clients.insert(
            sender_id,
            TelnetClient {
                stream: client
            }
        );
        self.inner.lock().unwrap().connected_id_counter += 1;

        sender_id
    }

    fn handle_connection(stream: TcpStream, sender_id: u32, sender: Sender<MessageWrapper>) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());

        // stream.set_read_timeout(Some(Duration::from_millis(1000))).unwrap();
        loop {
            match reader.fill_buf() {
                Ok(buffer) => {
                    let num_bytes = buffer.len();
                    if num_bytes > 0 {
                        sender.send(MessageWrapper {
                            sender_id,
                            message:  buffer.get(0..num_bytes).unwrap().to_vec()
                        }).unwrap();
                        reader.consume(num_bytes);
                    }
                }
                Err(error) => match error.kind() {
                    other_error => {
                        panic!("Unexpected error reading client stream: {:?}", other_error)
                    }
                }
            };
        }
    }
}


pub struct TelnetServer {
    pub inner: Arc<Mutex<TelnetServerInner>>
}
