use serde::{Deserialize, Serialize};

/// 消息类型
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
#[allow(dead_code)]
pub enum MessageType {
    LoginMessageRequest = 0,
    LoginMessageResponse = 1,
    LogoutMessageRequest = 2,
    LogoutMessageResponse = 3,
    KeepaliveMessageRequest = 4,
    KeepaliveMessageResponse = 5,
    ExchangeRouteMessageRequest = 6,
    ExchangeRouteMessageResponse = 7,

    /// 私聊文本消息
    PrivateTextMessageRequest = 1000,
    PrivateTextMessageResponse = 1001,
    /// 私聊图片消息
    PrivateImageMessageRequest = 1002,
    PrivateImageMessageResponse = 1003,
    /// 私聊音频消息
    PrivateAudioMessageRequest = 1004,
    PrivateAudioMessageResponse = 1005,
    /// 私聊视频消息
    PrivateVideoMessageRequest = 1006,
    PrivateVideoMessageResponse = 1007,
    /// 私聊文件消息
    PrivateFileMessageRequest = 1008,
    PrivateFileMessageResponse = 1009,

    GroupTextMessageRequest = 2000,
    GroupTextMessageResponse = 2001,
    GroupImageMessageRequest = 2002,
    GroupImageMessageResponse = 2003,
    GroupAudioMessageRequest = 2004,
    GroupAudioMessageResponse = 2005,
    GroupVideoMessageRequest = 2006,
    GroupVideoMessageResponse = 2007,
    GroupFileMessageRequest = 2008,
    GroupFileMessageResponse = 2009,
}

impl std::ops::Add<i32> for MessageType {
    type Output = MessageType;

    fn add(self, rhs: i32) -> Self::Output {
        (self as i32 + rhs).into()
    }
}

impl std::convert::Into<i32> for MessageType {
    fn into(self) -> i32 {
        self as i32
    }
}

impl std::convert::Into<MessageType> for i32 {
    fn into(self) -> MessageType {
        match self {
            0 => MessageType::LoginMessageRequest,
            1 => MessageType::LoginMessageResponse,
            2 => MessageType::LogoutMessageRequest,
            3 => MessageType::LogoutMessageResponse,
            4 => MessageType::KeepaliveMessageRequest,
            5 => MessageType::KeepaliveMessageResponse,
            6 => MessageType::ExchangeRouteMessageRequest,
            7 => MessageType::ExchangeRouteMessageResponse,
            1000 => MessageType::PrivateTextMessageRequest,
            1001 => MessageType::PrivateTextMessageResponse,
            1002 => MessageType::PrivateImageMessageRequest,
            1003 => MessageType::PrivateImageMessageResponse,
            1004 => MessageType::PrivateAudioMessageRequest,
            1005 => MessageType::PrivateAudioMessageResponse,
            1006 => MessageType::PrivateVideoMessageRequest,
            1007 => MessageType::PrivateVideoMessageResponse,
            1008 => MessageType::PrivateFileMessageRequest,
            1009 => MessageType::PrivateFileMessageResponse,
            2000 => MessageType::GroupTextMessageRequest,
            2001 => MessageType::GroupTextMessageResponse,
            2002 => MessageType::GroupImageMessageRequest,
            2003 => MessageType::GroupImageMessageResponse,
            2004 => MessageType::GroupAudioMessageRequest,
            2005 => MessageType::GroupAudioMessageResponse,
            2006 => MessageType::GroupVideoMessageRequest,
            2007 => MessageType::GroupVideoMessageResponse,
            2008 => MessageType::GroupFileMessageRequest,
            2009 => MessageType::GroupFileMessageResponse,
            _ => panic!("unknown"),
        }
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
            0 => Ok(MessageType::LoginMessageRequest),
            1 => Ok(MessageType::LoginMessageResponse),
            2 => Ok(MessageType::LogoutMessageRequest),
            3 => Ok(MessageType::LogoutMessageResponse),
            4 => Ok(MessageType::KeepaliveMessageRequest),
            5 => Ok(MessageType::KeepaliveMessageResponse),
            6 => Ok(MessageType::ExchangeRouteMessageRequest),
            7 => Ok(MessageType::ExchangeRouteMessageResponse),
            1000 => Ok(MessageType::PrivateTextMessageRequest),
            1001 => Ok(MessageType::PrivateTextMessageResponse),
            1002 => Ok(MessageType::PrivateImageMessageRequest),
            1003 => Ok(MessageType::PrivateImageMessageResponse),
            1004 => Ok(MessageType::PrivateAudioMessageRequest),
            1005 => Ok(MessageType::PrivateAudioMessageResponse),
            1006 => Ok(MessageType::PrivateVideoMessageRequest),
            1007 => Ok(MessageType::PrivateVideoMessageResponse),
            1008 => Ok(MessageType::PrivateFileMessageRequest),
            1009 => Ok(MessageType::PrivateFileMessageResponse),
            2000 => Ok(MessageType::GroupTextMessageRequest),
            2001 => Ok(MessageType::GroupTextMessageResponse),
            2002 => Ok(MessageType::GroupImageMessageRequest),
            2003 => Ok(MessageType::GroupImageMessageResponse),
            2004 => Ok(MessageType::GroupAudioMessageRequest),
            2005 => Ok(MessageType::GroupAudioMessageResponse),
            2006 => Ok(MessageType::GroupVideoMessageRequest),
            2007 => Ok(MessageType::GroupVideoMessageResponse),
            2008 => Ok(MessageType::GroupFileMessageRequest),
            2009 => Ok(MessageType::GroupFileMessageResponse),

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
            MessageType::LoginMessageRequest => {
                write!(f, "MessageType::LoginMessageRequest              ")
            }
            MessageType::LoginMessageResponse => {
                write!(f, "MessageType::LoginMessageResponse             ")
            }
            MessageType::LogoutMessageRequest => {
                write!(f, "MessageType::LogoutMessageRequest             ")
            }
            MessageType::LogoutMessageResponse => {
                write!(f, "MessageType::LogoutMessageResponse            ")
            }
            MessageType::KeepaliveMessageRequest => {
                write!(f, "MessageType::KeepaliveMessageRequest          ")
            }
            MessageType::KeepaliveMessageResponse => {
                write!(f, "MessageType::KeepaliveMessageResponse         ")
            }
            MessageType::ExchangeRouteMessageRequest => {
                write!(f, "MessageType::ExchangeRouteMessageRequest      ")
            }
            MessageType::ExchangeRouteMessageResponse => {
                write!(f, "MessageType::ExchangeRouteMessageResponse     ")
            }
            MessageType::PrivateTextMessageRequest => {
                write!(f, "MessageType::PrivateTextMessageRequest        ")
            }
            MessageType::PrivateTextMessageResponse => {
                write!(f, "MessageType::PrivateTextMessageResponse       ")
            }
            MessageType::PrivateImageMessageRequest => {
                write!(f, "MessageType::PrivateImageMessageRequest       ")
            }
            MessageType::PrivateImageMessageResponse => {
                write!(f, "MessageType::PrivateImageMessageResponse      ")
            }
            MessageType::PrivateAudioMessageRequest => {
                write!(f, "MessageType::PrivateAudioMessageRequest       ")
            }
            MessageType::PrivateAudioMessageResponse => {
                write!(f, "MessageType::PrivateAudioMessageResponse      ")
            }
            MessageType::PrivateVideoMessageRequest => {
                write!(f, "MessageType::PrivateVideoMessageRequest       ")
            }
            MessageType::PrivateVideoMessageResponse => {
                write!(f, "MessageType::PrivateVideoMessageResponse      ")
            }
            MessageType::PrivateFileMessageRequest => {
                write!(f, "MessageType::PrivateFileMessageRequest        ")
            }
            MessageType::PrivateFileMessageResponse => {
                write!(f, "MessageType::PrivateFileMessageResponse       ")
            }
            MessageType::GroupTextMessageRequest => {
                write!(f, "MessageType::GroupTextMessageRequest          ")
            }
            MessageType::GroupTextMessageResponse => {
                write!(f, "MessageType::GroupTextMessageResponse         ")
            }
            MessageType::GroupImageMessageRequest => {
                write!(f, "MessageType::GroupImageMessageRequest         ")
            }
            MessageType::GroupImageMessageResponse => {
                write!(f, "MessageType::GroupImageMessageResponse        ")
            }
            MessageType::GroupAudioMessageRequest => {
                write!(f, "MessageType::GroupAudioMessageRequest         ")
            }
            MessageType::GroupAudioMessageResponse => {
                write!(f, "MessageType::GroupAudioMessageResponse        ")
            }
            MessageType::GroupVideoMessageRequest => {
                write!(f, "MessageType::GroupVideoMessageRequest         ")
            }
            MessageType::GroupVideoMessageResponse => {
                write!(f, "MessageType::GroupVideoMessageResponse        ")
            }
            MessageType::GroupFileMessageRequest => {
                write!(f, "MessageType::GroupFileMessageRequest          ")
            }
            MessageType::GroupFileMessageResponse => {
                write!(f, "MessageType::GroupFileMessageResponse         ")
            }
        }
    }
}
