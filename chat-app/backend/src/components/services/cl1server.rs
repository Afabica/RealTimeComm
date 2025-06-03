use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Message sent between users
#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

/// Message for connecting a new user
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: String,
    pub addr: Recipient<ChatMessage>,
}

/// Message for disconnecting a user
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

/// Chat server
pub struct ChatServer {
    sessions: HashMap<String, Recipient<ChatMessage>>,
}

impl Default for ChatServer {
    fn default() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("User {} connected", msg.id);
        self.sessions.insert(msg.id.clone(), msg.addr);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("User {} disconnected", msg.id);
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ChatMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Context<Self>) {
        for client in self.sessions.values() {
            let _ = client.do_send(ChatMessage(msg.0.clone()));
        }
    }
}

