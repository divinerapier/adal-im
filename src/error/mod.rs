use std::error::Error as StdError;
use std::io::Error as StdIOError;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum EtcdError {
    Unexpected(String),
}

impl std::fmt::Display for EtcdError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for EtcdError {}

impl From<String> for EtcdError {
    fn from(s: String) -> EtcdError {
        if s.eq("") {
            return EtcdError::Unexpected(s);
        }
        return EtcdError::Unexpected(s);
    }
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

impl From<StdIOError> for Error {
    fn from(e: StdIOError) -> Error {
        Error {
            kind: ErrorKind::StdIOError(e),
        }
    }
}

impl From<Box<bincode::ErrorKind>> for Error {
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

impl From<etcd::Error> for Error {
    fn from(e: etcd::Error) -> Error {
        match e {
            etcd::Error::Api(e) => Error {
                kind: ErrorKind::EtcdError(EtcdError::from(e.message)),
            },
            default_e @ _ => Error {
                kind: ErrorKind::EtcdError(EtcdError::Unexpected(format!("{:?}", default_e))),
            },
        }
    }
}

impl From<Vec<etcd::Error>> for Error {
    fn from(err: Vec<etcd::Error>) -> Error {
        for e in err {
            return Error::from(e);
        }
        return Error {
            kind: ErrorKind::EtcdError(EtcdError::Unexpected(format!("empty errors vector"))),
        };
    }
}
