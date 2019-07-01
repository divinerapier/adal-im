use crate::error::Error;
use crate::protocol::BinaryProtocol;
pub struct Server {}

pub struct ServerBuilder {}

impl Server {
    pub fn run(&self) -> Result<(), Error> {
        let ln = std::net::TcpListener::bind("0.0.0.0:6810")?;
        loop {
            let conn = ln.accept()?;
            println!("accept a new connection. {}", conn.1);
            let mut protocol: BinaryProtocol = BinaryProtocol::new(From::from(conn));
            std::thread::spawn(move || loop {
                let packet = protocol.read_packet();
                println!(
                    "remote: {}, packet: {}",
                    protocol.transport.remote_addr, packet
                );
            });
        }
    }
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        ServerBuilder {}
    }

    pub fn build(self) -> Server {
        Server {}
    }
}
