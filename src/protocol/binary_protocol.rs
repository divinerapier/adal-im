use super::{MessageType, Packet, TCPConnection};
use std::io::Write;
pub struct BinaryProtocol {
    pub transport: TCPConnection,
}

impl BinaryProtocol {
    pub fn new(transport: TCPConnection) -> BinaryProtocol {
        BinaryProtocol { transport }
    }

    pub fn read_packet(&mut self) -> Packet {
        let length = dbg!(self.transport.read_u32().unwrap());
        let mut buffer: Vec<u8> = dbg!(Vec::with_capacity(length as usize));
        buffer.resize(length as usize, 0);
        dbg!(self.transport.read_extract(&mut buffer).unwrap());
        dbg!(&buffer);
        dbg!(Packet::decode(&buffer).unwrap())
    }

    pub fn write_packet(&mut self, message_type: MessageType, message: &str) {
        let packet = Packet::new(0, message_type, message);
        let data = packet.encode().unwrap();
        self.transport.stream.write(&data);
    }

    pub fn try_clone(&self) -> BinaryProtocol {
        let stream = self.transport.stream.try_clone().unwrap();
        let remote_addr = self.transport.remote_addr.clone();
        BinaryProtocol {
            transport: TCPConnection::new(stream, remote_addr),
        }
    }
}
