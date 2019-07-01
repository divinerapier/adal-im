mod error;
mod protocol;
mod server;
mod service;
mod transport;

fn main() {
    let message = "1".to_owned();
    format!("{:?}", dbg!(bincode::serialize(&message)));

    server::ServerBuilder::new().build().run().unwrap();
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
