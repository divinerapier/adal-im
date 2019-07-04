use crate::data::SyncData;

pub fn login(mut ctx: super::Context, mut data: SyncData) -> super::Context {
    println!(
        "login. remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    data.user_login(ctx.packet.user, ctx.prot.try_clone());
    match ctx.prot.write_packet(
        ctx.packet.user,
        ctx.packet.message_type + 1,
        &format!("user: {}. login successfully!", ctx.packet.user),
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("failed to reply. error: {}", e);
        }
    }
    ctx
}
