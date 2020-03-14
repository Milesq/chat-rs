use std::net::IpAddr;

pub struct ChatClient {
    pub name: String,
    pub server_ip: IpAddr,
    pub others_participants: Vec<String>,
}

impl ChatClient {
    pub fn new(name: String, server_ip: IpAddr) -> Self {
        Self {
            name,
            server_ip,
            others_participants: vec![],
        }
    }
}
