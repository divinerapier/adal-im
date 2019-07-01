use crate::protocol::MessageType;
use crate::protocol::Packet;
use crate::server::Server;
use crate::transport::Connection;

pub trait Service {
    fn service_type(&self) -> MessageType;
    fn register(&self, svr: &mut Server);
    fn serve(&self, pkt: &Packet);
}
