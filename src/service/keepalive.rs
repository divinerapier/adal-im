use serde::{Deserialize, Serialize};

use crate::data::SyncData;

#[derive(Serialize, Deserialize)]
pub struct KeepaliveMessage {
    pub user: u64,
}

pub fn keepalive(ctx: super::Context, mut data: SyncData) -> super::Context {
    println!(
        "remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    let request: KeepaliveMessage = match serde_json::from_str(&ctx.packet.message) {
        Ok(r) => r,
        Err(_) => {
            println!("invalid keepalive message. {}", ctx.packet.message);
            return ctx;
        }
    };

    data.user_keepalive(request.user);

    ctx
}
