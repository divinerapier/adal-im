use serde::{Deserialize, Serialize};

/// 消息类型
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum MessageType {
    LoginMessage = 0,

    /// 私聊文本消息
    PrivateTextMessage = 1000,
    /// 私聊图片消息
    PrivateImageMessage = 1001,
    /// 私聊音频消息
    PrivateAudioMessage = 1002,
    /// 私聊视频消息
    PrivateVideoMessage = 1003,
    /// 私聊文件消息
    PrivateFileMessage = 1004,

    GroupTextMessage = 2000,
    GroupImageMessage = 2001,
    GroupAudioMessage = 2002,
    GroupVideoMessage = 2003,
    GroupFileMessage = 2004,
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
            0 => Ok(MessageType::LoginMessage),
            1000 => Ok(MessageType::PrivateTextMessage),
            1001 => Ok(MessageType::PrivateImageMessage),
            1002 => Ok(MessageType::PrivateAudioMessage),
            1003 => Ok(MessageType::PrivateVideoMessage),
            1004 => Ok(MessageType::PrivateFileMessage),

            2000 => Ok(MessageType::GroupTextMessage),
            2001 => Ok(MessageType::GroupImageMessage),
            2002 => Ok(MessageType::GroupAudioMessage),
            2003 => Ok(MessageType::GroupVideoMessage),
            2004 => Ok(MessageType::GroupFileMessage),
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
            MessageType::LoginMessage => write!(f, "login message"),
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
