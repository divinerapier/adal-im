use crate::data::SyncData;

pub fn exchange_route(ctx: super::Context, mut data: SyncData) -> super::Context {
    println!(
        "remote: {}, type: {}, message: {}",
        ctx.prot.transport.remote_addr, ctx.packet.message_type, ctx.packet.message
    );
    let request: super::ExchangeRouteMessage = serde_json::from_str(&ctx.packet.message).unwrap();

    match data.get_connnection(&request.address) {
        None => {
            println!("unknown server: {}", request.address);
        }
        Some(c) => {
            println!(
                "add user route. user: {}, address: {}",
                request.user, request.address
            );
            data.add_user_route(request.user, c, &request.address);
        }
    }
    ctx
}
