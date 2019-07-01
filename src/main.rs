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
    // let packet: protocol::Packet =
    //     protocol::Packet::new(1, protocol::MessageType::PrivateImageMessage, "1");
    // let data = dbg!(packet.encode().unwrap());
    // println!("{:?}", data);

    // dbg!(protocol::MessageType::PrivateTextMessage as isize);
    // dbg!(protocol::MessageType::PrivateImageMessage as isize);
    // dbg!(protocol::MessageType::PrivateAudioMessage as isize);
    // dbg!(protocol::MessageType::PrivateVideoMessage as isize);
    // dbg!(protocol::MessageType::PrivateFileMessage as isize);
    // dbg!(protocol::MessageType::GroupTextMessage as isize);
    // dbg!(protocol::MessageType::GroupImageMessage as isize);
    // dbg!(protocol::MessageType::GroupAudioMessage as isize);
    // dbg!(protocol::MessageType::GroupVideoMessage as isize);
    // dbg!(protocol::MessageType::GroupFileMessage as isize);

    // dbg!(protocol::MessageType::PrivateTextMessage as i32);
    // dbg!(protocol::MessageType::PrivateImageMessage as i32);
    // dbg!(protocol::MessageType::PrivateAudioMessage as i32);
    // dbg!(protocol::MessageType::PrivateVideoMessage as i32);

    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::PrivateTextMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::PrivateImageMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::PrivateAudioMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::PrivateVideoMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::PrivateFileMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::GroupTextMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::GroupImageMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::GroupAudioMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::GroupVideoMessage
    // ));
    // dbg!(std::mem::size_of_val(
    //     &protocol::MessageType::GroupFileMessage
    // ));
}
