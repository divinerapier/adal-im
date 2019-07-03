mod error;
mod protocol;
mod server;
mod service;
mod transport;

fn handler1(mut ctx: protocol::Context) -> protocol::Context {
    println!(
        "remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    match ctx.prot.write_packet(
        ctx.packet.message_type,
        &format!("handler1. nihao. {}", ctx.prot.transport.remote_addr),
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("failed to reply. error: {}", e);
        }
    }
    ctx
}

fn handler2(mut ctx: protocol::Context) -> protocol::Context {
    println!(
        "remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    match ctx.prot.write_packet(
        ctx.packet.message_type,
        &format!("handler2. nihao. {}", ctx.prot.transport.remote_addr),
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("failed to reply. error: {}", e);
        }
    }
    ctx
}

fn main() {
    server::Server::new()
        .add(
            protocol::MessageType::PrivateAudioMessage,
            service::Handler::H1(handler1),
        )
        .add(
            protocol::MessageType::GroupAudioMessage,
            service::Handler::H1(handler2),
        )
        .add(
            protocol::MessageType::LoginMessage,
            service::Handler::H2(service::login),
        )
        .add(
            protocol::MessageType::PrivateTextMessage,
            service::Handler::H2(service::private_text_message),
        )
        .run("0.0.0.0:6810")
        .unwrap();

    // let packet: protocol::Packet = protocol::Packet::new(
    //     1,
    //     protocol::MessageType::PrivateTextMessage,
    //     "{\"from\":2,\"to\":1,\"message\":\"1562068944\"}",
    // );
    // let data = packet.encode().unwrap();
    // println!("{:?}", data);
}
