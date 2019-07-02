use crate::protocol::{Context, MessageType, Packet};

pub type Handler = fn(Context) -> Context;


// pub trait ServiceFactory<I> {
//     fn service_type(&self) -> MessageType;
//     fn serve(&self, ctx:  I);
// }

// pub struct Service<F, I>
// where
//     F: Factory<I>,
// {
//     message_type: MessageType,
//     handler: Option<Box<Handler<F, I>>>,
// }

// impl<F, I> Service<F, I>
// where
//     F: Factory<I>,
// {
//     pub fn new(message_type: MessageType) -> Service<F, I> {
//         Service {
//             message_type,
//             handler: None,
//         }
//     }

//     pub fn route(mut self, handler: F) -> Service<F, I>
//     where
//         F: Factory<I>,
//     {
//         self.handler = Some(Box::new(Handler::new(handler)));
//         self
//     }
// }

// impl<F, I> ServiceFactory<I> for Service<F, I>
// where
//     F: Factory<I>,
// {
//     fn service_type(&self) -> MessageType {
//         self.message_type
//     }
//     fn serve(&self, ctx:  I) {
// self.handler.unwrap().call(ctx);
//     }
// }

// pub struct Handler<F, I>
// where
//     F: Factory<I>,
// {
//     handler: F,
//     _t: std::marker::PhantomData<I>,
// }

// impl<F, I> Handler<F, I>
// where
//     F: Factory<I>,
// {
//     pub fn new(handler: F) -> Self {
//         Handler {
//             handler,
//             _t: std::marker::PhantomData,
//         }
//     }

//     pub fn call(&self,ctx : I) {
//         self.handler.call(ctx);
//     }
// }

// pub trait Factory<I> {
//      fn call(&self, ctx: I) -> Context;
// }

// impl<F, I> Factory<I> for F
// where
//     F: Fn() -> Context,
// {
//     fn call(&self, input: I) -> Context {
//         self(input)
//     }
// }

