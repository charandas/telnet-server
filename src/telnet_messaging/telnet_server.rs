use std::io::{
    Read,
    Write,
    BufRead,
    BufReader
};
use std::collections::HashMap;
use std::thread;

use std::sync::{
    Arc,
    Mutex,
    mpsc::Receiver
};

use std::sync::mpsc::Sender;

use crate::telnet_messaging::telnet_client::TelnetClient;


pub struct MessageWrapper {
    pub sender_id: u32,
    pub message: Vec<u8>,
}


#[derive(Debug)]
pub struct TelnetServerInner<'a, T>
where T :  Read + Write + BufRead + Clone + Send
{
    pub connected_clients:  HashMap<u32, TelnetClient<'a, T>>,
    pub connected_id_counter: u32
}

impl<'a, T> TelnetServer<'a, T>
where T :  Read + Write + BufRead + Clone + Send
{
    pub fn new() -> TelnetServer<'a, T> {
        TelnetServer {
            inner: Arc::new(Mutex::new(TelnetServerInner {
                connected_clients: HashMap::new(),
                connected_id_counter: 1
            }))
        }
    }
    pub fn register_client(&mut self, client: &T, sender: Sender<MessageWrapper>) -> u32 {
        let mut inner = self.inner.lock().unwrap();

        let sender_id = inner.connected_id_counter;

        // let cloned_client = client.try_clone().unwrap();
        let cloned_server = self.inner.clone();

        thread::spawn(move || {
            Self::handle_connection(client, sender_id, sender);
            cloned_server.lock().unwrap().connected_clients.remove(&sender_id);
        });
       //  println!("{:?}", self.inner);

        inner.connected_clients.insert(
            sender_id,
            TelnetClient {
                stream: client
            }
        );
        inner.connected_id_counter += 1;

        sender_id
    }

    pub fn register_rx_handler(&mut self, rx: Receiver<MessageWrapper>) {
        let cloned_server = Arc::clone(&self.inner);

        thread::spawn(move || {
            for received in rx {
                println!("Got: {:?} from sender: {}", received.message, received.sender_id);

                for (id, client) in cloned_server.lock().unwrap().connected_clients.iter_mut() {
                    if *id != received.sender_id {
                        client.stream.write(format!("{}: ", received.sender_id).as_bytes()).unwrap();
                        client.stream.write(&received.message).unwrap();
                        client.stream.flush().unwrap();
                    }
                }

            }
        });
    }

    fn handle_connection(stream: &T, sender_id: u32, sender: Sender<MessageWrapper>) {
        let mut reader = BufReader::new(stream);

        loop {
            let buffer = reader.fill_buf().unwrap();
            let num_bytes = buffer.len();
            if num_bytes > 0 {
                sender.send(MessageWrapper {
                    sender_id,
                    message:  buffer.get(0..num_bytes).unwrap().to_vec()
                }).unwrap();
                // using buffer.consume still returns the same data again. Do it on the reader itself.
                reader.consume(num_bytes);
            } else {
                // Stream is closed
                drop(sender);
                println!("Client left. Closing connection...");
                break;
            }
        }
    }
}

pub struct TelnetServer<'a, T>
where T :  Read + Write + BufRead + Clone + Send
{
    inner: Arc<Mutex<TelnetServerInner<'a, T>>>
}
