use crate::error::Error;
use crate::protocol::{BinaryProtocol, Context, MessageType, Packet};
use crate::service::Handler;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[allow(dead_code)]
pub struct Server {
    services: Arc<RwLock<HashMap<MessageType, Handler>>>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub fn add(self, message_type: MessageType, handler: Handler) -> Server {
        let map = self.services.clone();
        let mut map = map.write().unwrap();
        map.insert(message_type, handler);
        self
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
            Self::dispatch(services.clone(), conn, packet);
        });
    }

    fn dispatch(
        router: Arc<RwLock<HashMap<MessageType, Handler>>>,
        conn: BinaryProtocol,
        packet: Packet,
    ) {
        let router = router.read().unwrap();
        if !router.contains_key(&packet.message_type) {
            println!("not found message type. {}", packet);
            return;
        }
        router[&packet.message_type](Context::new(conn, packet));
    }
}

// impl<S, I> Server<S>
// where
//     S: ServiceFactory<I> + Send + Sync + 'static,
// {
//     pub fn run(&self, addr: &str) -> Result<(), Error> {
//         let ln = std::net::TcpListener::bind(addr)?;
//         loop {
//             let conn = ln.accept()?;
//             println!("accept a new connection. {}", conn.1);
//             let protocol = BinaryProtocol::new(From::from(conn));
//             self.handler(protocol);
//         }
//     }

//     fn handler(&self, mut prot: BinaryProtocol) {
//         let services = self.services.clone();
//         std::thread::spawn(move || loop {
//             let packet = prot.read_packet();
//             let conn = prot.try_clone();
//             Self::dispatch(services.clone(), conn, packet);
//         });
//     }

//     fn dispatch(
//         router: Arc<Mutex<HashMap<MessageType, Box<S>>>>,
//         conn: BinaryProtocol,
//         packet: Packet,
//     ) where
//         S: ServiceFactory<I>,
//     {
//         let router = router.lock().unwrap();
//         if !router.contains_key(&packet.message_type) {
//             return;
//         }
//         router[&packet.message_type].serve(&mut Context::new(conn, packet));
//     }
// }

// impl<S, I> ServerBuilder<S> {
//     pub fn new() -> ServerBuilder<S> {
//         ServerBuilder {
//             services: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     pub fn service(self, svc: S) -> Self
//     where
//         S: ServiceFactory<I>,
//     {
//         let router = self.services.clone();
//         let mut router = router.lock().unwrap();
//         router.insert(svc.service_type(), Box::new(svc));
//         self
//     }

//     pub fn build(self) -> Server<S> {
//         Server {
//             services: self.services,
//         }
//     }
// }
