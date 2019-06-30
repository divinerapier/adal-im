use crate::error::Error;
use byteorder::ByteOrder;
use byteorder::WriteBytesExt;
use std::io::Read;
use std::net::SocketAddr as StdSocketAddr;
use std::net::TcpStream as StdTcpStream;

pub struct TCPConnection {
    pub stream: StdTcpStream,
    pub remote_addr: StdSocketAddr,
}

#[allow(dead_code)]
impl TCPConnection {
    pub fn new(c: StdTcpStream, remote_addr: StdSocketAddr) -> TCPConnection {
        TCPConnection {
            stream: c,
            remote_addr,
        }
    }

    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buffer = [0; 1];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer[0])
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buffer: [u8; 2] = [0; 2];
        self.stream.read_exact(&mut buffer)?;
        Ok(byteorder::LittleEndian::read_u16(&mut buffer))
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buffer: [u8; 4] = [0; 4];
        self.stream.read_exact(&mut buffer)?;
        Ok(byteorder::LittleEndian::read_u32(&mut buffer))
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buffer: [u8; 8] = [0; 8];
        self.stream.read_exact(&mut buffer)?;
        Ok(byteorder::LittleEndian::read_u64(&mut buffer))
    }

    pub fn read_extract(&mut self, mut buffer: &mut [u8]) -> Result<(), Error> {
        Ok(self.stream.read_exact(&mut buffer)?)
    }

    pub fn write_u8(&mut self, v: u8) -> Result<(), Error> {
        self.stream.write_u8(v)?;
        Ok(())
    }
    pub fn write_u16(&mut self, v: u16) -> Result<(), Error> {
        self.stream.write_u16::<byteorder::LittleEndian>(v)?;
        Ok(())
    }
    pub fn write_u32(&mut self, v: u32) -> Result<(), Error> {
        self.stream.write_u32::<byteorder::LittleEndian>(v)?;
        Ok(())
    }
    pub fn write_u64(&mut self, v: u64) -> Result<(), Error> {
        self.stream.write_u64::<byteorder::LittleEndian>(v)?;
        Ok(())
    }
}

impl From<(StdTcpStream, StdSocketAddr)> for TCPConnection {
    fn from(conn: (StdTcpStream, StdSocketAddr)) -> TCPConnection {
        TCPConnection {
            stream: conn.0,
            remote_addr: conn.1,
        }
    }
}
