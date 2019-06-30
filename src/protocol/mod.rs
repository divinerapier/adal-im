use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Packet {
    pub total_length: u32,
    pub request_user: u64,
    pub message_type: MessageType,
    pub message: String,
}

/// 消息类型
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessageType {
    /// 私聊文本消息
    PrivateTextMessage = 0isize,
    /// 私聊图片消息
    PrivateImageMessage = 1isize,
    /// 私聊音频消息
    PrivateAudioMessage = 2isize,
    /// 私聊视频消息
    PrivateVideoMessage = 3isize,
    /// 私聊文件消息
    PrivateFileMessage = 4isize,

    GroupTextMessage = 1000isize,
    GroupImageMessage = 1001isize,
    GroupAudioMessage = 1002isize,
    GroupVideoMessage = 1003isize,
    GroupFileMessage = 1004isize,
}

impl Packet {
    fn encode(&self) -> Result<Vec<u8>, Error> {
        Ok(bincode::serialize(&self)?)
    }
    fn decode(data: &Vec<u8>) -> Result<Packet, Error> {
        Ok(bincode::deserialize(&data)?)
    }
}
