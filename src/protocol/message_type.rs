use serde::{Deserialize, Serialize};

/// 消息类型
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[allow(dead_code)]
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
            1 => Ok(MessageType::PrivateImageMessage),
            2 => Ok(MessageType::PrivateAudioMessage),
            3 => Ok(MessageType::PrivateVideoMessage),
            4 => Ok(MessageType::PrivateFileMessage),

            1000 => Ok(MessageType::GroupTextMessage),
            1001 => Ok(MessageType::GroupImageMessage),
            1002 => Ok(MessageType::GroupAudioMessage),
            1003 => Ok(MessageType::GroupVideoMessage),
            1004 => Ok(MessageType::GroupFileMessage),
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
