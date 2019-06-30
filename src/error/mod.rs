use std::io::Error as StdIOError;

pub struct Error {}

impl std::convert::From<StdIOError> for Error {
    fn from(e: StdIOError) -> Error {
        Error {}
    }
}

impl std::convert::From<Box<bincode::ErrorKind>> for Error {
    fn from(e : Box<bincode::ErrorKind>) -> Error {
        Error{}
    }
}