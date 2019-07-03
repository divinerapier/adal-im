use crate::error::Error;
use crate::protocol::{BinaryProtocol, Context, MessageType, Packet};
use crate::service::Handler;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[allow(dead_code)]
pub struct Server {
    services: Arc<RwLock<HashMap<MessageType, Handler>>>,
    data: Arc<RwLock<HashMap<u64, BinaryProtocol>>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            services: Arc::new(RwLock::new(HashMap::new())),
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add(self, message_type: MessageType, handler: Handler) -> Server {
        let map = self.services.clone();
        let mut map = map.write().unwrap();
        match map.insert(message_type, handler) {
            None => self,
            Some(_) => panic!("duplicated message type: {}", message_type),
        }
    }

    pub fn run(&self, addr: &str) -> Result<(), Error> {
        let ln = std::net::TcpListener::bind(addr)?;
        loop {
            let conn = ln.accept()?;
            println!("accept a new connection. {}", conn.1);
            let protocol = BinaryProtocol::new(From::from(conn));
            self.handle(protocol);
        }
    }

    fn handle(&self, mut prot: BinaryProtocol) {
        let services = self.services.clone();
        let data = self.data.clone();
        std::thread::spawn(move || loop {
            let packet = match prot.read_packet() {
                Ok(p) => p,
                Err(e) => {
                    println!("failed to read packet. error: {} ", e);
                    if e.is_eof() {
                        println!("eof");
                        return;
                    }
                    continue;
                }
            };
            let conn = prot.try_clone();
            Self::dispatch(services.clone(), data.clone(), conn, packet);
        });
    }

    fn dispatch(
        services: Arc<RwLock<HashMap<MessageType, Handler>>>,
        data: Arc<RwLock<HashMap<u64, BinaryProtocol>>>,
        conn: BinaryProtocol,
        packet: Packet,
    ) {
        let router = services.read().unwrap();
        if !router.contains_key(&packet.message_type) {
            println!("not found message type. {}", packet);
            return;
        }
        let handler = &router[&packet.message_type];
        match handler {
            crate::service::Handler::H1(h1) => h1(Context::new(conn, packet)),
            crate::service::Handler::H2(h2) => h2(Context::new(conn, packet), data),
        };
    }
}
