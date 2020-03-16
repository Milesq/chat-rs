use {
    serde::Serialize,
    std::{
        io::{self, Read, Write},
        net::{IpAddr, TcpStream},
    },
};

pub struct ChatClient {
    pub name: String,
    pub server_ip: IpAddr,
    pub others_participants: Vec<String>,
}

#[derive(Serialize, Debug)]
enum MsgType {
    GetParticipants,
    AddParticipant(String),
}

impl ChatClient {
    pub fn new(name: String, server_ip: IpAddr) -> io::Result<Self> {
        Ok(Self {
            name,
            server_ip,
            others_participants: Self::get_participants(server_ip)?,
        })
    }

    fn get_participants(ip: IpAddr) -> io::Result<Vec<String>> {
        let packet = bincode::serialize(&MsgType::GetParticipants)
            .unwrap()
            .bytes();
        TcpStream::connect("192:80")?.write_all(packet);
        Ok(vec![])
    }
}
