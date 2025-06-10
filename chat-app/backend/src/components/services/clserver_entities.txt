use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::components::models::chat_group::{ChatGroup, ChatMessage};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Clone, Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<WsMessage>>,
    groups: HashMap<String, HashMap<Uuid, Recipient<WsMessage>>>,
}

impl ChatServer {
    pub fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            groups: HashMap::new(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub addr: Recipient<ChatMessage>,
    pub group: String,
}

impl Handler<Join> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Self::Context) {
        slef.groups
            .entry(msg.group.clone())
            .or_default()
            .insert(msg.addr);
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}


//impl ChatServer {
//    pub fn new() -> ChatServer {
//        ChatServer {
//            sessions: HashMap::new(),
//            groups: HashMap::new(),
//        }
//    }
//
//    fn send_message_to_group(&self, group_id: &GroupId, message: &str) {
//        if let Some(members) = self.groups.get(group_id) {
//            for user_id in members {
//                if let Some(session) = self.sessions.get(user_id) {
//                    let _ = sessions.addr.do_send(SendMessage(message.to_owned()));
//                }
//            }
//        }
//    }
//
////    fn send_message(&self, message: &str, skip_id: usize) {
////        for (id, recp) in self.sessions.iter() {
////            if *id != skip_id {
////                let _ = recp.do_send(WsMessage(message.to_owned()));
////            }
////        }
////    }
//}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub group: String,
    pub id: Uuid,
}

impl Handler<Connect> for ChatServer {
    type Result = (); 

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("User {} conected to group {}", msg.id, msg.group);
        self.groups
            .entry(msg.group.clone())
            .or_default()
            .insert(msg.id, msg.addr);
    }
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub group: String,
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if let Some(group) = self.groups.get_mut(&msg.group) {
            group.remove(&msg.id);
            if group.is_empty() {
                self.groups.remove(&msg.group);
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: Uuid,
    pub group: String,
    pub text: String,
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _: &mut Self::Context) {
        if let Some(users) = self.groups.get(&msg.group) {
            for user in users {
                let _ = user.do_send(ChatMessage {
                    group: msg.group.clone(),
                    message: msg.message.clone(),
                });
            }
        }
    }
}
