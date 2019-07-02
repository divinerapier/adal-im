use crate::protocol::BinaryProtocol;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub fn private_text_message(
    mut ctx: super::Context,
    data: Arc<RwLock<HashMap<u64, BinaryProtocol>>>,
) -> super::Context {
    println!(
        "remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );

    let mut data = data.write().unwrap();
    let request: super::TextMessage = serde_json::from_str(&ctx.packet.message).unwrap();

    if !data.contains_key(&request.to) {
        match ctx.prot.write_packet(
            ctx.packet.message_type,
            &format!("user {} not exists.", request.to),
        ) {
            Ok(_) => {}
            Err(e) => {
                println!("failed to reply. error: {}", e);
            }
        }
        return ctx;
    }

    let peer_conn: &mut BinaryProtocol = data.get_mut(&request.to).unwrap();
    match peer_conn.write_packet(ctx.packet.message_type, &format!("{}", request.message)) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "failed to send message to peer. from: {}, to: {}. error: {}",
                request.from, request.to, e
            );
        }
    }

    ctx
}
