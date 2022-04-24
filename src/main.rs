pub mod telnet_messaging {
    pub mod telnet_server;
    pub mod telnet_client;
}

use std::io::{
    prelude::Write
};

use std::sync::{
    Arc,
    Mutex,
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
    let server = Arc::new(Mutex::new(TelnetServer::new()));

    let (tx, rx): (Sender<MessageWrapper>, Receiver<MessageWrapper>) = mpsc::channel();

    // TODO: collect this join Handle
    {
        let server = Arc::clone(&server);
        thread::spawn(move || {
            for received in rx {
                println!("Got: {:?} from sender: {}", received.message, received.sender_id);

                for (id, client) in server.lock().unwrap().connected_clients.iter_mut() {
                    if *id != received.sender_id {
                        client.stream.write(&received.message).unwrap();
                        client.stream.flush().unwrap();
                    }
                }

            }
        });
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Registering new client...");
        server.lock().unwrap().register_client(stream, tx.clone())
    }
}
