use super::types::*;

use std::{
    io::{self, Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    ops::Drop,
};

pub struct Participant {
    pub name: String,
    pub ip: IpAddr,
}

pub struct ChatServer {
    others_participants: Vec<Participant>,
    pub port: u16,
}

impl ChatServer {
    pub fn new(port: u16) -> Self {
        Self {
            others_participants: Vec::new(),
            port,
        }
    }

    pub fn serve(self) -> io::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port));

        for stream in listener?.incoming() {
            self.handle_request(&mut stream?)?;
        }

        Ok(())
    }

    fn handle_request(&self, req: &mut TcpStream) -> io::Result<()> {
        println!("serve");
        let mut buf = [0; 2];
        req.read(&mut buf)?;

        req.write_all(&[0])?;

        println!("{:?}", buf);
        Ok(())
    }
}

impl Drop for ChatServer {
    fn drop(&mut self) {}
}
