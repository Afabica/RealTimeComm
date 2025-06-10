use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Responder, Error};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use uuid::Uuid;
use crate::components::services::clserver_entities::{ChatServer, Connect, Disconnect, ClientMessage, WsMessage};
use crate::components::models::model_mongo::AppState;

/// WebSocket session actor
pub struct WsSession {
    id: Uuid,
    hb: Instant,
    group: String,
    server_addr: Addr<ChatServer>,
}

impl WsSession {
    pub fn new(group: String, server_addr: Addr<ChatServer>) -> Self {
        Self {
            id: Uuid::new_v4(),
            hb: Instant::now(),
            group,
            server_addr,
        }
    }

    fn start_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                println!("Client heartbeat failed, disconnecting.");
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_heartbeat(ctx);

        self.server_addr.do_send(Connect {
            id: self.id,
            group: self.group.clone(),
            addr: ctx.address().recipient(),
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        self.server_addr.do_send(Disconnect {
            id: self.id,
            group: self.group.clone(),
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut ws::WebsocketContext<Self>) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.server_addr.do_send(ClientMessage {
                    id: self.id,
                    group: self.group.clone(),
                    text,
                });
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(msg.0);
    }
}

