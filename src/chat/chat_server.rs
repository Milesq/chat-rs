use std::{
    io::{self, ErrorKind, Read, Write},
    net::{SocketAddr, TcpListener},
    sync::{Arc, Mutex},
    thread,
};

use super::types::*;

mod handle_request;
use handle_request::Handler;

use super::prepare_request::*;

pub fn run_server<'a>(port: u16) -> io::Result<&'a dyn Fn()> {
    static mut SHUTDOWN: bool = false;
    let server = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");
    let tcp_handler = Arc::new(Mutex::new(Handler::new()));

    thread::spawn(move || loop {
        if unsafe { SHUTDOWN } {
            break;
        }

        if let Ok((mut socket, addr)) = server.accept() {
            let mut packet_config = PreparePacketConfig::new();

            let tcp_handler = Arc::clone(&tcp_handler);
            thread::spawn(move || loop {
                let mut buf = vec![0; crate::PACKET_SIZE];
                let mut tcp_handler = tcp_handler.lock().unwrap();

                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        if let Some(buf) = packet_config.prepare_to_receive(buf) {
                            packet_config = Default::default();
                            let response = (*tcp_handler).handler(buf, addr);

                            if let Some(response) = response {
                                let packet = bincode::serialize(&response).unwrap();

                                for part in prepare_to_send(packet.clone()) {
                                    socket.write_all(&part[..]).unwrap_or_else(|err| {
                                        println!("Send response error: {:?}", err);
                                    });
                                }
                            }
                        }
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        let users = (*tcp_handler).participants.clone();
                        let news =
                            WhatsUp::ParticipantDisconected(match_user_name_with_ip(addr, users));
                        println!("{}", news);
                        (*tcp_handler).messages.push(news);
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

fn match_user_name_with_ip(addr: SocketAddr, users: Vec<Participant>) -> String {
    users
        .iter()
        .find(|user| user.ip == addr)
        .map(|user| user.name.clone())
        .unwrap_or("Unknown".to_string())
}
