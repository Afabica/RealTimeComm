use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use crate::components::models::model_postgres::{RegisteredUser, RegisterRequest };
use crate::components::models::messages::Message;
use crate::components::models::chat_group::ChatGroup;


// Represents a WebSocket session (one client).
// hb tracks the last hearbeat time(used to detect timeouts)
struct MyWs {
    hb: Instant, 
}

// Actor implementation
// started() is called when the WebSocket connection is established
// It triggers hb() to start a hearbeat check loop
impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl MyWs {
    fn new() -> Self {
        Self { hb: Instant::now()} 
    }

    // Sends a PING every 5 seconds
    // If no PONG is received in 10 seconds, disconnect the client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.hb) > Duration::from_secs(10) {
                println!("WebSocket Client hearbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

// Handles incoming messages (text, binary,ping/pong, close)
// Currently, it just echoes back the text 
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut ws::WebsocketContext<Self>){
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}


// Entry point for a client to connect to WebSocket (e.g., /ws route)
// Starts a new MyWs actor per connection
pub async fn ws_index(r: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(MyWs::new(), &r, stream)
}

