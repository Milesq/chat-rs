use std::{
    io::{self, ErrorKind, Read, Write},
    net::{IpAddr, TcpListener},
    thread,
};

mod handle_request;

pub struct Participant {
    pub name: String,
    pub ip: IpAddr,
}

use super::prepare_request::*;

pub fn run_server<'a>(port: u16) -> io::Result<&'a dyn Fn()> {
    static mut SHUTDOWN: bool = false;
    let server = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    thread::spawn(move || loop {
        if unsafe { SHUTDOWN } {
            break;
        }

        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let mut packet_config = PreparePacketConfig::new();

            thread::spawn(move || loop {
                let mut buf = vec![0; crate::PACKET_SIZE];

                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        if let Some(buf) = prepare_to_receive(buf, &mut packet_config) {
                            packet_config = Default::default();

                            let mut buf = handle_request::handler(buf);
                            buf.resize(crate::PACKET_SIZE, 0);

                            socket.write_all(&buf).unwrap_or_else(|err| {
                                println!("Send response error: {:?}", err);
                            });
                        }
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

        crate::sleep();
    });

    Ok(&|| unsafe {
        SHUTDOWN = true;
    })
}
