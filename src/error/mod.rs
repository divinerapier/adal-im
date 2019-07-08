use etcd::Error as EtcdError;
use std::error::Error as StdError;
use std::io::Error as StdIOError;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn is_eof(&self) -> bool {
        match &self.kind {
            ErrorKind::StdIOError(e) => match e.kind() {
                std::io::ErrorKind::UnexpectedEof => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "kind: {}, description: {}",
            self.kind,
            self.description()
        )
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::BincodeError(e) => write!(f, "bincode: {}", e),
            ErrorKind::StdIOError(e) => write!(f, "stdio: {}", e),
            ErrorKind::EtcdError(e) => write!(f, "etcd: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    BincodeError(bincode::ErrorKind),
    StdIOError(StdIOError),
    EtcdError(EtcdError),
}

impl std::convert::From<StdIOError> for Error {
    fn from(e: StdIOError) -> Error {
        Error {
            kind: ErrorKind::StdIOError(e),
        }
    }
}

impl std::convert::From<Box<bincode::ErrorKind>> for Error {
    fn from(e: Box<bincode::ErrorKind>) -> Error {
        Error {
            kind: ErrorKind::BincodeError(*e),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::BincodeError(ref e) => e.description(),
            ErrorKind::StdIOError(ref e) => e.description(),
            ErrorKind::EtcdError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self.kind {
            ErrorKind::BincodeError(ref e) => e.source(),
            ErrorKind::StdIOError(ref e) => Some(e),
            ErrorKind::EtcdError(ref e) => Some(e),
        }
    }
}

impl std::convert::From<etcd::Error> for Error {
    fn from(e: etcd::Error) -> Error {
        Error {
            kind: ErrorKind::EtcdError(e),
        }
    }
}
