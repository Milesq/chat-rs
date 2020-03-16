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
        Ok(Self {
            name,
            server_ip,
            others_participants: Self::get_participants(server_ip)?,
        })
    }

    fn get_participants(ip: SocketAddr) -> io::Result<Participants> {
        let mut buf = Vec::<u8>::new();
        let packet = &bincode::serialize(&MsgType::GetParticipants).unwrap()[..];

        let mut socket = TcpStream::connect(ip)?;
        socket.write_all(packet)?;
        socket.read_to_end(&mut buf)?;

        Ok(bincode::deserialize::<Participants>(&buf[..]).unwrap_or_default())
    }
}

impl Drop for ChatClient {
    fn drop(&mut self) {
        println!("Destroy client!");
    }
}
