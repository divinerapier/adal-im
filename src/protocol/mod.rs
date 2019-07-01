use crate::error::Error;
use crate::transport::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Packet {
    pub total_length: u32,
    pub request_user: u64,
    pub message_type: MessageType,
    pub message: String,
}

impl Packet {
    pub fn new(user: u64, message_type: MessageType, message: &str) -> Packet {
        let total_length = 4 + 8 + 4 + 8 + message.len();
        Packet {
            total_length: total_length as u32,
            request_user: user,
            message_type,
            message: message.to_owned(),
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "total_length: {}, request_user: {}, message_type: {}, message: {}",
            self.total_length, self.request_user, self.message_type, self.message
        )
    }
}

/// 消息类型
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MessageType {
    /// 私聊文本消息
    PrivateTextMessage = 0,
    /// 私聊图片消息
    PrivateImageMessage = 1,
    /// 私聊音频消息
    PrivateAudioMessage = 2,
    /// 私聊视频消息
    PrivateVideoMessage = 3,
    /// 私聊文件消息
    PrivateFileMessage = 4,

    GroupTextMessage = 1000,
    GroupImageMessage = 1001,
    GroupAudioMessage = 1002,
    GroupVideoMessage = 1003,
    GroupFileMessage = 1004,
}

impl std::convert::Into<i32> for MessageType {
    fn into(self) -> i32 {
        self as i32
    }
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(MessageType::into(*self))
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            0 => Ok(MessageType::PrivateTextMessage),
            1 => Ok(MessageType::PrivateTextMessage),
            2 => Ok(MessageType::PrivateTextMessage),
            3 => Ok(MessageType::PrivateTextMessage),
            4 => Ok(MessageType::PrivateTextMessage),

            1000 => Ok(MessageType::PrivateTextMessage),
            1001 => Ok(MessageType::PrivateTextMessage),
            1002 => Ok(MessageType::PrivateTextMessage),
            1003 => Ok(MessageType::PrivateTextMessage),
            1004 => Ok(MessageType::PrivateTextMessage),
            _ => Err(serde::de::Error::custom(format!(
                "unknown message type: {}",
                value as i32
            ))),
        }
    }
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MessageType::PrivateTextMessage => write!(f, "private text message"),
            MessageType::PrivateImageMessage => write!(f, "private image message"),
            MessageType::PrivateAudioMessage => write!(f, "private audio message"),
            MessageType::PrivateVideoMessage => write!(f, "private video message"),
            MessageType::PrivateFileMessage => write!(f, "private file message"),
            MessageType::GroupTextMessage => write!(f, "group text message"),
            MessageType::GroupImageMessage => write!(f, "group image message"),
            MessageType::GroupAudioMessage => write!(f, "group audio message"),
            MessageType::GroupVideoMessage => write!(f, "group video message"),
            MessageType::GroupFileMessage => write!(f, "group file message"),
        }
    }
}

impl Packet {
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        dbg!(Ok(bincode::serialize(&self)?))
    }
    pub fn decode(data: &Vec<u8>) -> Result<Packet, Error> {
        dbg!(Ok(bincode::deserialize(&data)?))
    }
}

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
