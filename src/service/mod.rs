use crate::data::SyncData;
use crate::protocol::Context;
use serde::{Deserialize, Serialize};

mod exchange_route;
mod keepalive;
mod login;
mod private_text;

pub use exchange_route::exchange_route;
pub use keepalive::keepalive;
pub use login::login;
pub use private_text::private_text_message;

pub use crate::protocol::ExchangeRouteMessage;

pub enum Handler {
    H1(Handler1),
    H2(Handler2),
}

pub type Handler1 = fn(Context) -> Context;
pub type Handler2 = fn(Context, SyncData) -> Context;

#[derive(Serialize, Deserialize)]
pub struct TextMessage {
    pub from: u64,
    pub to: u64,
    pub message: String,
}
