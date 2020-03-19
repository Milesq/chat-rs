use super::types::*;

use std::{
    io::{self, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    ops::Drop,
    thread,
    time::Duration,
};

pub struct Participant {
    pub name: String,
    pub ip: IpAddr,
}

pub struct ChatServer {
    participants: Vec<Participant>,
    messages: Vec<(String, String)>,
    port: u16,
}

impl ChatServer {
    pub fn new(port: u16) -> Self {
        Self {
            participants: Vec::new(),
            messages: Vec::new(),
            port,
        }
    }

    pub fn serve(mut self) -> io::Result<()> {
        let mut listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        listener
            .set_nonblocking(true)
            .expect("failed to initialize non-blocking");

        loop {
            if let Ok((mut socket, addr)) = listener.accept() {
                println!("Client {} connected", addr);

                thread::spawn(move || loop {
                    let mut buff = vec![0; 20];

                    match socket.read_exact(&mut buff) {
                        Ok(_) => {
                            println!("{:?}", buff);
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("closing connection with: {}", addr);
                            break;
                        }
                    }

                    thread::sleep(Duration::from_millis(100));
                });
            }
        }

        // for stream in listener.incoming() {
        //     self.handle_request(&mut stream?).unwrap_or_else(|err| {
        //         println!("Err: {:?}", err);
        //     });
        // }
    }

    fn handle_request(&mut self, mut req: &mut TcpStream) -> io::Result<()> {
        let request_type: bincode::Result<ReqType> = bincode::deserialize_from(&mut req);
        let user_ip = req.local_addr().unwrap().ip();
        let user = self.participants.iter().find(|user| user.ip == user_ip);

        let resp = if let Err(err) = request_type {
            let response: Result<Participants, ServerErr> = Err(ServerErr::ErrBadRequest400);
            println!("Bad request: {:?}", err);

            bincode::serialize(&response)
        } else {
            match request_type.unwrap() {
                ReqType::GetParticipants => bincode::serialize(
                    &self
                        .participants
                        .iter()
                        .map(|el| el.name.clone())
                        .collect::<Participants>(),
                ),
                ReqType::AddParticipant(name) => {
                    if self.participants.iter().any(|el| el.name.clone() == name) {
                        bincode::serialize(&false)
                    } else {
                        println!("User connected: {}", name);

                        self.participants.push(Participant { name, ip: user_ip });

                        bincode::serialize(&true)
                    }
                }
                ReqType::SendMessage(msg) => match user {
                    None => bincode::serialize(&ServerErr::UnknownUser),
                    Some(user) => {
                        let Participant { name, .. } = user;

                        println!("{}: {}", name, msg);

                        self.messages.push((name.clone(), msg));
                        bincode::serialize(&true)
                    }
                },
            }
        };

        req.write_all(resp.unwrap().as_slice())?;

        Ok(())
    }
}

impl Drop for ChatServer {
    fn drop(&mut self) {}
}
