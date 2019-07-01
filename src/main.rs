mod error;
mod protocol;
mod server;
mod service;
mod transport;

struct TestService1 {}

impl service::Service for TestService1 {
    fn service_type(&self) -> protocol::MessageType {
        protocol::MessageType::PrivateTextMessage
    }
    fn serve(&self, ctx: &mut protocol::Context) {
        println!(
            "remote: {}, type: {}, message: {}",
            ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
        );
        ctx.prot.write_packet(
            ctx.packet.message_type,
            &format!("TestService1. nihao. {}", ctx.prot.transport.remote_addr),
        );
    }
}

impl TestService1 {
    fn new() -> TestService1 {
        TestService1 {}
    }
}

struct TestService2 {}

impl service::Service for TestService2 {
    fn service_type(&self) -> protocol::MessageType {
        protocol::MessageType::PrivateTextMessage
    }
    fn serve(&self, ctx: &mut protocol::Context) {
        println!(
            "remote: {}, type: {}, message: {}",
            ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
        );
        ctx.prot.write_packet(
            ctx.packet.message_type,
            &format!("TestService2. nihao. {}", ctx.prot.transport.remote_addr),
        );
    }
}

impl TestService2 {
    fn new() -> TestService2 {
        TestService2 {}
    }
}

fn main() {
    server::ServerBuilder::new()
        .service(TestService1::new())
        .build()
        .run()
        .unwrap();
    let packet: protocol::Packet =
        protocol::Packet::new(1, protocol::MessageType::PrivateImageMessage, "lanaya");
    let data = dbg!(packet.encode().unwrap());
    println!("{:?}", data);
}
