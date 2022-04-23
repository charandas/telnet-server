use std::io::{
    prelude::Read,
    ErrorKind
};
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc::{
    self,
    Sender,
    Receiver
};
use std::time::Duration;
use std::thread::{
    self,
    JoinHandle
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut server = TelnetServer::new();

    let (tx, rx): (Sender<MessageWrapper>, Receiver<MessageWrapper>) = mpsc::channel();

    // TODO: collect this join Handle
    thread::spawn(|| {
        for received in rx {
            println!("Got: {:?}", received.message);
        }
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Registering new client...");
        server.register_client(stream, tx.clone())
    }
}
#[derive(Debug)]
pub struct TelnetServer {
    connected_clients: Vec<TelnetClient>,
    connected_id_counter: u32
}

#[derive(Debug)]
pub struct TelnetClient {
    sender_id: u32,
    join_handle: Option<JoinHandle<()>>,
}

pub struct MessageWrapper {
    sender_id: u32,
    message: Vec<u8>,
}

impl TelnetClient {
    // fn new(join_handle: Option<JoinHandle<()>>) -> TelnetClient {
    //     TelnetClient {
    //         join_handle
    //     }
    // }
}

impl TelnetServer {
    pub fn new() -> TelnetServer {
        TelnetServer {
            connected_clients: Vec::new(),
            connected_id_counter: 1
        }
    }
    pub fn register_client(&mut self, client: TcpStream, sender: Sender<MessageWrapper>) {
        let sender_id = self.connected_id_counter;
        client.set_nonblocking(true).unwrap();
        let join_handle = thread::spawn(move || {
            Self::handle_connection(client, sender_id, sender);
        });
        println!("{:?}", self);
        self.connected_clients.push(
            TelnetClient {
                join_handle: Some(join_handle),
                sender_id
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
