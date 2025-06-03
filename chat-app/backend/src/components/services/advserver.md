use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use crate::components::models::model_postgres::{RegisteredUser, RegisterRequest };
use crate::components::models::messages::Message;
use crate::components::models::chat_group::ChatGroup;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

pub struct ChatServer {
    sessions: HashMap<String, Recipient<ChatMessage>>,
}


pub struct WsChatSession {
    pub  id: String,
    pub hb: Instant,
    pub adr: Addr<ChatServer>, 
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .do_send(Connect {
                id: self.id.clone(),
                addr: addr.recipient(),
            })
    }
}
