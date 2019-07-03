use super::{MessageType, Packet, TCPConnection};
use crate::error::Error;

use std::io::Write;

pub struct BinaryProtocol {
    pub transport: TCPConnection,
}

impl std::fmt::Debug for BinaryProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "tcp. local {}, peer {}",
            self.transport.stream.local_addr().unwrap(),
            self.transport.stream.peer_addr().unwrap()
        )
    }
}

impl BinaryProtocol {
    pub fn new(transport: TCPConnection) -> BinaryProtocol {
        BinaryProtocol { transport }
    }

    pub fn with_address(address: &str) -> Result<BinaryProtocol, Error> {
        let stream = std::net::TcpStream::connect(address)?;
        let peer_addr = stream.peer_addr()?;
        Ok(BinaryProtocol {
            transport: TCPConnection::new(stream, peer_addr),
        })
    }

    pub fn read_packet(&mut self) -> Result<Packet, Error> {
        let length = self.transport.read_u32()?;
        let mut buffer: Vec<u8> = Vec::with_capacity(length as usize);
        buffer.resize(length as usize, 0);
        self.transport.read_extract(&mut buffer)?;
        Ok(Packet::decode(&buffer)?)
    }

    pub fn write_packet(
        &mut self,
        user: u64,
        message_type: MessageType,
        message: &str,
    ) -> Result<(), Error> {
        let packet = Packet::new(user, message_type, message);
        let data = packet.encode()?;
        self.transport
            .stream
            .write(&bincode::serialize::<u32>(&packet.total_length)?)?;
        self.transport.stream.write(&data)?;
        Ok(())
    }

    pub fn try_clone(&self) -> BinaryProtocol {
        let stream = self.transport.stream.try_clone().unwrap();
        let remote_addr = self.transport.remote_addr.clone();
        BinaryProtocol {
            transport: TCPConnection::new(stream, remote_addr),
        }
    }
}
