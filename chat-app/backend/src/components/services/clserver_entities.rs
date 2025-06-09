use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::components::models::chat_group::ChatGroup;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Clone, Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<WsMessage>>,
    groups: HashMap<Uuid, Uuid>,

}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            groups: HashMap::new(),
        }
    }

    fn send_message_to_group(&self, group_id: &GroupId, message: &str) {
        if let Some(members) = self.groups.get(group_id) {
            for user_id in members {
                if let Some(session) = self.sessions.get(user_id) {
                    let _ = sessions.addr.do_send(SendMessage(message.to_owned()));
                }
            }
        }
    }

//    fn send_message(&self, message: &str, skip_id: usize) {
//        for (id, recp) in self.sessions.iter() {
//            if *id != skip_id {
//                let _ = recp.do_send(WsMessage(message.to_owned()));
//            }
//        }
//    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.id_counter;
        self.id_counter += 1;
        self.sessions.insert(id, msg.addr);
        id
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.msg, msg.id);
    }
}

