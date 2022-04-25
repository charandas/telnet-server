pub mod telnet_messaging {
    pub mod telnet_server;
    pub mod telnet_client;
}

use std::io::{
    prelude::Write
};

use std::sync::{
    Arc,
    mpsc::{
        self,
        Sender,
        Receiver
    }
};

use std::thread;
use std::net::TcpListener;
use crate::telnet_messaging::telnet_server::{
    MessageWrapper,
    TelnetServer
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut server = TelnetServer::new();

    let (tx, rx): (Sender<MessageWrapper>, Receiver<MessageWrapper>) = mpsc::channel();

    // Register receive handler that will fan-out the responses
    let cloned_server = Arc::clone(&server.inner);
    thread::spawn(move || {
        for received in rx {
            println!("Got: {:?} from sender: {}", received.message, received.sender_id);

            for (id, client) in cloned_server.lock().unwrap().connected_clients.iter_mut() {
                if *id != received.sender_id {
                    client.stream.write(&received.message).unwrap();
                    client.stream.flush().unwrap();
                }
            }

        }
    });

    // Register each client's stream with a unique tx that will pass the data back from their thread into the server receive handler
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Registering new client...");
        server.register_client(stream, tx.clone());
    }
}
