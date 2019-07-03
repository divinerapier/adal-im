use crate::data::SyncData;

pub fn private_text_message(mut ctx: super::Context, data: SyncData) -> super::Context {
    println!(
        "remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    let request: super::TextMessage = match serde_json::from_str(&ctx.packet.message) {
        Ok(request) => request,
        Err(e) => {
            match ctx.prot.write_packet(
                ctx.packet.user,
                ctx.packet.message_type + 1,
                &format!("invalid private text message. {}", e),
            ) {
                Ok(_) => {}
                Err(e) => {
                    println!("failed to reply. error: {}", e);
                }
            };
            return ctx;
        }
    };
    let peer_conn = data.get_user_connection(request.to);
    match peer_conn {
        None => {
            match ctx.prot.write_packet(
                ctx.packet.user,
                ctx.packet.message_type + 1,
                &format!("user {} not exists.", request.to),
            ) {
                Ok(_) => {}
                Err(e) => {
                    println!("failed to reply. error: {}", e);
                }
            };
        }
        Some(mut conn) => {
            if data.is_local_user(request.to) {
                match conn.write_packet(
                    ctx.packet.user,
                    ctx.packet.message_type + 1,
                    &format!("{}", ctx.packet.message),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "failed to send message to peer. from: {}, to: {}. error: {}",
                            request.from, request.to, e
                        );
                    }
                }
            } else {
                match conn.write_packet(
                    ctx.packet.user,
                    ctx.packet.message_type,
                    &format!("{}", ctx.packet.message),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        println!(
                            "failed to send message to peer. from: {}, to: {}. error: {}",
                            request.from, request.to, e
                        );
                    }
                }
            }
        }
    }
    ctx
}
