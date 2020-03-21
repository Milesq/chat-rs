use std::{
    io::{self, ErrorKind, Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
};

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
            println!("Client {} connected", addr);
            let mut packet_config = PreparePacketConfig::new();

            let tcp_handler = Arc::clone(&tcp_handler);
            thread::spawn(move || loop {
                let mut buf = vec![0; crate::PACKET_SIZE];

                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        if let Some(buf) = prepare_to_receive(buf, &mut packet_config) {
                            packet_config = Default::default();
                            let mut tcp_handler = tcp_handler.lock().unwrap();

                            let packet = (*tcp_handler).handler(buf, addr);

                            for part in prepare_to_send(packet) {
                                socket.write_all(&part[..]).unwrap_or_else(|err| {
                                    println!("Send response error: {:?}", err);
                                });
                            }
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
