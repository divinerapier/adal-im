use std::net::TcpStream as StdTcpStream;
use std::net::SocketAddr as StdSocketAddr;

pub struct Connection {
    stream: StdTcpStream,
    remote_addr: StdSocketAddr,
}

impl Connection {
    pub fn new(c: StdTcpStream, remote_addr: StdSocketAddr) -> Connection {
        Connection {
            stream:c,
            remote_addr,
        }
    }
}

impl From<(StdTcpStream, StdSocketAddr)> for Connection {
    fn from(conn: (StdTcpStream, StdSocketAddr) ) -> Connection {
        Connection{
            stream: conn.0,
            remote_addr: conn.1,
        }
    }
}