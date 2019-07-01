use crate::protocol::{Context, MessageType};

pub trait Service {
    fn service_type(&self) -> MessageType;
    fn serve(&self, ctx: &mut Context);
}
