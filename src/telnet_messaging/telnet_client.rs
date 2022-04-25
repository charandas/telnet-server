use std::net::TcpStream;

#[derive(Debug)]
pub struct TelnetClient {
    pub stream: TcpStream
}
