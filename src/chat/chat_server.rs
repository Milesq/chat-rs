// use std::net::IpAddr;
use std::ops::Drop;

// pub struct Participant {
//     pub name: String,
//     pub ip: Option<IpAddr>,
// }

pub struct ChatServer;
//     pub me: Participant,
//     pub others_participants: Vec<Participant>,
// }

impl ChatServer {
    pub fn new() -> Self {
        Self
    }

    pub fn serve() {}
}

impl Drop for ChatServer {
    fn drop(&mut self) {}
}
