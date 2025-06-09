use actix::{Actor, Addr, AsyncContext, Handler, StreamHandler, ContextFutureSpawner};
use actix_web_actors::ws;
use actix_web::{HttpRequest, HttpResponse, Error, web, Responder};
use std::time::{Duration, Instant};
use crate::components::services::clserver_entities::{ChatServer, Connect, Disconnect, ClientMessage, WsMessage};
use futures_util::future::{ready, Ready};
use actix::{WrapFuture, ActorFutureExt, ActorContext};
use crate::components::models::model_mongo::AppState;

pub struct MyWs {
    id: usize,
    hb: Instant,
    addr: Addr<ChatServer>,
}

impl MyWs {
    pub fn new(addr: Addr<ChatServer>) -> Self {
        MyWs {
            id: 0,
            hb: Instant::now(),
            addr,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                println!("WebSocket Client heartbeat failed, disconnecting!");
                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        // Send Connect message to ChatServer, wait for response to get assigned id
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self) // convert future to actor context
            .then(|res, act, ctx| {
                match res {
                    Ok(id) => {
                        act.id = id;
                        println!("Assigned ID: {}", id);
                    }
                    _ => {
                        println!("Failed to connect to chat server");
                        ctx.stop();
                    }
                }
                ready(())
            })
            .wait(ctx);

        self.hb(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        self.addr.do_send(Disconnect { id: self.id });
        actix::Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
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
                // Forward text message to chat server for broadcast
                self.addr.do_send(ClientMessage {
                    id: self.id,
                    msg: text.to_string(),
                });
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<WsMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.text(msg.0);
    }
}

//pub async fn ws_index(
//    req: HttpRequest,
//    stream: web::Payload,
//    srv: web::Data<Addr<ChatServer>>,
//) -> Result<HttpResponse, Error> {
//    ws::start(MyWs::new(srv.get_ref().clone()), &req, stream)
//}

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<AppState>,) -> Result<HttpResponse, Error> {
    let chat_server = srv.chat_server.clone();
    ws::start(MyWs::new(chat_server), &req, stream)
}

