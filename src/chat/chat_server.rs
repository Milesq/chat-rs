use super::types::*;

use std::{
    io::{self, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener},
    sync::mpsc::channel,
    thread,
};

pub struct Participant {
    pub name: String,
    pub ip: IpAddr,
}

pub fn run_server<'a>(port: u16) -> io::Result<&'a dyn Fn()> {
    static mut SHUTDOWN: bool = false;
    let server = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let (tx, rx) = channel::<String>();
    thread::spawn(move || loop {
        if unsafe { SHUTDOWN } {
            break;
        }

        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();

            thread::spawn(move || loop {
                let mut buff = vec![];

                match socket.read_to_end(&mut buff) {
                    Ok(_) => {
                        println!("{}: {:?}", addr, buff);
                        tx.send(".".into()).expect("failed to send msg to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                crate::sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            println!("{}", msg);
        }

        crate::sleep();
    });

    Ok(&|| unsafe {
        SHUTDOWN = true;
    })
}
