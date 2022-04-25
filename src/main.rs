pub mod telnet_messaging {
    pub mod telnet_server;
    pub mod telnet_client;
}

pub mod tests;

use std::sync::{
    mpsc::{
        self,
        Sender,
        Receiver
    }
};
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
    server.register_rx_handler(rx);

    // Register each client's stream with a unique tx that will pass the data back from their thread into the server receive handler
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Registering new client...");
        server.register_client(&mut stream, tx.clone());
    }
}
