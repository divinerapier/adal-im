use crate::data::SyncData;
use crate::error::Error;
use crate::protocol::{BinaryProtocol, Context, MessageType, Packet};
use crate::service::Handler;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub mod manager;

#[allow(dead_code)]
pub struct Server {
    services: Arc<RwLock<HashMap<MessageType, Handler>>>,
    data: SyncData,
}

impl Server {
    pub fn new() -> Server {
        Server {
            services: Arc::new(RwLock::new(HashMap::new())),
            data: SyncData::new(),
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

    pub fn run(&mut self, addr: &str) -> Result<(), Error> {
        let parts: Vec<_> = addr.split(':').collect();
        let mut local_addr = crate::network::local_ip().unwrap();
        local_addr.push(':');
        local_addr.push_str(parts[1]);
        self.data.set_local_addr(&local_addr);
        self.data.sync();
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
        data: SyncData,
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

mod test {
    #[test]
    fn foo() {
        let parts: Vec<_> = "0.0.0.0:6810".split(':').collect();
        assert!(parts.len() == 2, format!("wrong result! {:?}", parts));
        assert!(parts[0] == "0.0.0.0", format!("wrong result! {:?}", parts));
    }
}
