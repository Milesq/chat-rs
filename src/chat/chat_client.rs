use {
    super::types::*,
    std::{
        io::{self, Read, Write},
        net::{SocketAddr, TcpStream},
        ops::Drop,
    },
};

pub struct ChatClient {
    pub name: String,
    pub server_ip: SocketAddr,
    pub others_participants: Vec<String>,
}

impl ChatClient {
    pub fn new(name: String, server_ip: SocketAddr) -> io::Result<Self> {
        Self::get_participants(server_ip)?;
        Ok(Self {
            name,
            server_ip,
            others_participants: Self::get_participants(server_ip)?,
        })
    }

    fn get_participants(ip: SocketAddr) -> io::Result<Participants> {
        let req = &bincode::serialize(&ReqType::GetParticipants).unwrap()[..];

        let mut socket = TcpStream::connect(ip)?;
        socket.write_all(req)?;

        let participants: bincode::Result<Participants> = bincode::deserialize_from(&socket);

        Ok(participants.expect("Cannot connect with server and download users data!"))
    }
}

impl Drop for ChatClient {
    fn drop(&mut self) {
        println!("Destroy client!");
    }
}
