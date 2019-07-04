mod data;
mod error;
mod network;
mod protocol;
mod server;
mod service;
mod transport;

fn main() {
    server::Server::new()
        .add(
            protocol::MessageType::LoginMessageRequest,
            service::Handler::H2(service::login),
        )
        .add(
            protocol::MessageType::PrivateTextMessageRequest,
            service::Handler::H2(service::private_text_message),
        )
        .add(
            protocol::MessageType::ExchangeRouteMessageRequest,
            service::Handler::H2(service::exchange_route),
        )
        .add(
            protocol::MessageType::KeepaliveMessageRequest,
            service::Handler::H2(service::keepalive),
        )
        .run(&std::env::args().collect::<Vec<_>>()[1])
        .unwrap();

    // let packet: protocol::Packet = protocol::Packet::new(
    //     1,
    //     protocol::MessageType::PrivateTextMessage,
    //     "{\"from\":2,\"to\":1,\"message\":\"1562068944\"}",
    // );
    // let data = packet.encode().unwrap();
    // println!("{:?}", data);
}
