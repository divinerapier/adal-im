use super::BinaryProtocol;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExchangeRouteMessage {
    pub user: u64,
    pub address: String,
}

impl ExchangeRouteMessage {
    fn new(user: u64, address: String) -> ExchangeRouteMessage {
        ExchangeRouteMessage { user, address }
    }
}

impl BinaryProtocol {
    pub fn exchange_route(&mut self, user: u64, server_addr: &str) {
        let message = ExchangeRouteMessage::new(user, server_addr.to_owned());
        let data = serde_json::to_string(&message).unwrap();
        self.write_packet(0, super::MessageType::ExchangeRouteMessageRequest, &data)
            .unwrap();
    }
}
