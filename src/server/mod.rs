use crate::error::Error;
use crate::protocol::BinaryProtocol;
use crate::protocol::MessageType;
use crate::service::Service;
use std::collections::HashMap;

pub struct Server {
    router: HashMap<MessageType, Box<Service>>,
}

impl Server {
    fn register_service(&mut self, svc: &dyn Service) {
        svc.register(self);
    }
}

pub struct ServerBuilder {}

impl Server {
    pub fn run(&self) -> Result<(), Error> {
        let ln = std::net::TcpListener::bind("0.0.0.0:6810")?;
        loop {
            let conn = ln.accept()?;
            println!("accept a new connection. {}", conn.1);
            let mut protocol: BinaryProtocol = BinaryProtocol::new(From::from(conn));
            let router = self.router.
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
        Server {
            router: HashMap::new(),
        }
    }
}
