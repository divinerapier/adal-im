use crate::error::Error;
use crate::protocol::{BinaryProtocol, Context, MessageType, Packet};
use crate::service::Service;
use crate::transport::TCPConnection;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

#[allow(dead_code)]
pub struct Server<S> {
    services: Arc<Mutex<HashMap<MessageType, Box<S>>>>,
}

pub struct ServerBuilder<S> {
    services: Arc<Mutex<HashMap<MessageType, Box<S>>>>,
}

impl<S> Server<S>
where
    S: Service + Send + Sync + 'static,
{
    pub fn run(&self) -> Result<(), Error> {
        let ln = std::net::TcpListener::bind("0.0.0.0:6810")?;
        loop {
            let conn = ln.accept()?;
            println!("accept a new connection. {}", conn.1);
            let protocol = BinaryProtocol::new(From::from(conn));
            self.handler(protocol);
        }
    }

    fn handler(&self, mut prot: BinaryProtocol) {
        let services = self.services.clone();
        std::thread::spawn(move || loop {
            let packet = prot.read_packet();
            let conn = prot.try_clone();
            Self::dispatch(services.clone(), conn, packet);
        });
    }

    fn dispatch(
        router: Arc<Mutex<HashMap<MessageType, Box<S>>>>,
        conn: BinaryProtocol,
        packet: Packet,
    ) where
        S: Service,
    {
        let router = router.lock().unwrap();
        if !router.contains_key(&packet.message_type) {
            return;
        }
        router[&packet.message_type].serve(&mut Context::new(conn, packet));
    }
}

impl<S> ServerBuilder<S> {
    pub fn new() -> ServerBuilder<S> {
        ServerBuilder {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn service(self, svc: S) -> Self
    where
        S: Service,
    {
        let router = self.services.clone();
        let mut router = router.lock().unwrap();
        router.insert(svc.service_type(), Box::new(svc));
        self
    }

    pub fn build(self) -> Server<S> {
        Server {
            services: self.services,
        }
    }
}
