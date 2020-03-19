use {
    super::types::*,
    std::{
        io::{self, Write},
        net::{SocketAddr, TcpStream},
        ops::Drop,
    },
};

pub struct ChatClient {
    name: String,
    server: TcpStream,
    others_participants: Option<Vec<String>>,

    pub on_msg: Option<fn(msg: (String, String))>,
}

impl ChatClient {
    pub fn new(name: String, server_ip: SocketAddr) -> io::Result<Self> {
        let mut server = TcpStream::connect(server_ip)?;
        server
            .set_nonblocking(true)
            .expect("failed to initiate non-blocking");
        let mut instance = Self {
            name,
            server,
            others_participants: None,
            on_msg: None,
        };

        instance.register()?;
        instance.others_participants = Some(instance.get_participants()?);

        Ok(instance)
    }

    fn get_participants(&self) -> io::Result<Participants> {
        // let req = &bincode::serialize(&ReqType::GetParticipants).unwrap()[..];

        // let mut socket = TcpStream::connect(ip)?;
        // socket
        //     .set_nonblocking(true)
        //     .expect("failed to initiate non-blocking");
        // socket.write_all(&[1, 2, 3])?;

        // let participants: bincode::Result<Participants> = bincode::deserialize_from(&socket);

        // Ok(participants.expect("Cannot connect with server and download users data!"))
        Ok(Vec::new())
    }

    fn register(&mut self) -> io::Result<()> {
        let req = bincode::serialize(&ReqType::AddParticipant(self.name.clone())).unwrap();

        self.server.write_all(&req[..])?;

        // let resp: bincode::Result<bool> = bincode::deserialize_from(&socket);

        // if !resp.expect("Register error") {
        //     println!("User name already taken! Consider to choose another");
        // } else {
        //     self.stay_up_to_date();
        // }

        Ok(())
    }

    pub fn send(&self, msg: String) -> io::Result<()> {
        /* let req = bincode::serialize(&ReqType::SendMessage(msg)).unwrap();
        let mut client = TcpStream::connect(self.server_ip)?;
        client
            .set_nonblocking(true)
            .expect("failed to initiate non-blocking");

        client.write_all(&req[..])?; */

        Ok(())
    }

    pub fn stay_up_to_date(&self) {
        // thread::spawn(|| {
        //     let req = bincode::serialize(&ReqType::SendMessage(msg)).unwrap();
        //     let mut socket = TcpStream::connect(self.server_ip)?;
        // });
    }
}

impl Drop for ChatClient {
    fn drop(&mut self) {
        println!("Destroy client!");
    }
}
