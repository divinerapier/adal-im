use super::Connection;
use super::Packet;

pub struct BinaryProtocol {
    pub transport: Connection,
}

impl BinaryProtocol {
    pub fn new(transport: Connection) -> BinaryProtocol {
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
}
