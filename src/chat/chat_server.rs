use super::types::*;

use std::{
    io::{self, Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    ops::Drop,
};

pub struct Participant {
    pub name: String,
    pub ip: IpAddr,
}

pub struct ChatServer {
    participants: Vec<Participant>,
    pub port: u16,
}

impl ChatServer {
    pub fn new(port: u16) -> Self {
        Self {
            participants: vec!["milesq"]
                .iter()
                .map(|el| Participant {
                    name: el.to_string(),
                    ip: "127.0.0.1".parse().unwrap(),
                })
                .collect(),
            port,
        }
    }

    pub fn serve(mut self) -> io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port));

        for stream in listener?.incoming() {
            self.handle_request(&mut stream?).unwrap_or_else(|err| {
                println!("Err: {:?}", err);
            });
        }

        Ok(())
    }

    fn handle_request(&mut self, mut req: &mut TcpStream) -> io::Result<()> {
        let request_type: bincode::Result<ReqType> = bincode::deserialize_from(&mut req);

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

                        self.participants.push(Participant {
                            name,
                            ip: req.peer_addr().unwrap().ip(),
                        });

                        bincode::serialize(&true)
                    }
                }
            }
        };

        req.write_all(resp.unwrap().as_slice())?;

        Ok(())
    }
}

impl Drop for ChatServer {
    fn drop(&mut self) {}
}
