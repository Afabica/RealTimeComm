use std::process::Command;

pub fn start_turn_server() {
    let output = Command::new("turnserver")
        .arg("-c")
        .arg("/etc/turnserver.cong")
        .output()
            .expect("Failed to start TURN server");

        println!("TURN Server started: {:?}", output);
}
