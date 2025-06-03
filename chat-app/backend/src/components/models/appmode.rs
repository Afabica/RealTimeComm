//use std::{env, fs::File, io::{Cursor, Write}};
//
//#[impl_enum::with_methods {
//    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>
//        pub fn write(&mut self, bufL &[u8]) -> std::io::Result<usize>
//}]
//
//#[derive(Debug)]
//pub enum ModeSwitch {
//    Military,
//    Citizen
//}
//
//#[derive(Debug)]
//pub enum UserStatus {
//    Active,
//    Inactive,
//    Banned,
//}
//
//#[derive(Debug)]
//pub enum Message {
//    Text(String),
//    Email { sender: String, receiver: String, body: String},
//    Notification(i32, String),
//}
//
//
//
//fn process_message(msg: Message) {
//    match msg {
//        Message::Text(content) => println!("Text message: {}",  content),
//        Message::Email { sender, receiver, body} => {
//            println!("Email from {} to {}: {}", sender, receiver, body);
//        }
//        Message::Notification(code, message)  => {
//            println!("Notification [{}]: {}", code, message);
//        }
//    }
//}
//
//fn check_status(status: UsersStatus) {
//    match status  {
//        UserStatus::Active => println!("User is active."),
//        UserStatus::Inactive => println!("User is inactive.")
//        UserStatus::Banned => println!("User is banned."),
//    }
//}
