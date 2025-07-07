//use actix::*;
//use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
//use actix_web_actors::ws;
//use std::time::{Duration, Instant};
//
//struct ChatSession;
//
//impl Actor for ChatSession {
//    type Context = ws::WebsocketContext<Self>;
//}
//
//impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
//    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//        if let Ok(ws::Message::Text(text)) = msg {
//            ctx.text(format!("You said: {}", text));
//        }
//    }
//}
//
//pub async fn chat_route(req: HttpRequest, stream: web::Payload) -> HttpResponse {
//    ws::start(ChatSession {}, &req, stream).unwrap()
//}
//
//
