use crate::protocol::BinaryProtocol;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub fn login(
    mut ctx: super::Context,
    data: Arc<RwLock<HashMap<u64, BinaryProtocol>>>,
) -> super::Context {
    println!(
        "login. remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    let mut data = data.write().unwrap();
    data.insert(ctx.packet.user, ctx.prot.try_clone());
    match ctx.prot.write_packet(
        ctx.packet.message_type,
        &format!("user: {}. login successfully!", ctx.packet.user),
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("failed to reply. error: {}", e);
        }
    }
    ctx
}
