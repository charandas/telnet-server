use std::net::TcpStream;

use std::thread::{
    // self,
    JoinHandle
};

#[derive(Debug)]
pub struct TelnetClient {
    pub join_handle: Option<JoinHandle<()>>,
    pub stream: TcpStream
}

impl TelnetClient {
    // fn new(join_handle: Option<JoinHandle<()>>) -> TelnetClient {
    //     TelnetClient {
    //         join_handle
    //     }
    // }
}
