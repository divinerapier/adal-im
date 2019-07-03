use crate::protocol::{BinaryProtocol, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

mod login;
mod private_text;

pub use login::login;
pub use private_text::private_text_message;

pub enum Handler {
    H1(Handler1),
    H2(Handler2),
}

pub type Handler1 = fn(Context) -> Context;
pub type Handler2 = fn(Context, Arc<RwLock<HashMap<u64, BinaryProtocol>>>) -> Context;

#[derive(Serialize, Deserialize)]
pub struct TextMessage {
    pub from: u64,
    pub to: u64,
    pub message: String,
}
