use crate::error::Error;
use crate::transport::Connection;
pub struct Server {}

pub struct ServerBuilder {}

impl Server {
    fn run(&self) -> Result<(), Error> {
        let ln = std::net::TcpListener::bind("0.0.0.0:6810")?;
        loop {
            let conn = ln.accept()?;
            let conn: Connection = From::from(conn);
            std::thread::spawn(move || {
                let _c = conn;
            });
        }
        unreachable!();
    }
}

impl ServerBuilder {
    pub fn new() -> ServerBuilder {
        ServerBuilder {}
    }

    pub fn build(self) -> Server {
        Server {}
    }
}
