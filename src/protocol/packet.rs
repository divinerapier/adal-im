use serde::{Deserialize, Serialize};

use super::Error;
use super::MessageType;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Packet {
    pub total_length: u32,
    pub user: u64,
    pub message_type: MessageType,
    pub message: String,
}

impl Packet {
    pub fn new(user: u64, message_type: MessageType, message: &str) -> Packet {
        Packet {
            total_length: 24 + message.len() as u32,
            user,
            message_type,
            message: message.to_owned(),
        }
    }
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        dbg!(Ok(bincode::serialize(&self)?))
    }
    pub fn decode(data: &Vec<u8>) -> Result<Packet, Error> {
        dbg!(Ok(bincode::deserialize(&data)?))
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "length: {}, user: {}, type: {}, message: {}",
            self.total_length, self.user, self.message_type, self.message
        )
    }
}
