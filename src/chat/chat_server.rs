use super::types::*;

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
                        if let Some(buf) = prepare_packet(buf, &mut packet_config) {
                            packet_config = Default::default();
                            println!("packet receiver: {:?}", buf);
                            // println!("{:?}", handle_request::handler(buf));
                            // let mut buf = "abc".to_string().clone().into_bytes();
                            // buf.resize(crate::PACKET_SIZE, 0);

                            // socket.write_all(&buf).unwrap_or_else(|err| {
                            //     println!("Send response error: {:?}", err);
                            // });
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

#[derive(Default)]
struct PreparePacketConfig {
    packet: Vec<u8>,
    length: usize,
    is_length_calculated: bool,
}

impl PreparePacketConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

fn prepare_packet(part: Vec<u8>, config: &mut PreparePacketConfig) -> Option<Vec<u8>> {
    if !config.is_length_calculated {
        // Calculating length

        if let Some(n) = part.iter().position(|&el| el == 0) {
            let tail = &part[(n + 1)..];
            config.length += sum(Vec::from(&part[..n]));
            config.packet.extend(tail);

            if tail.len() > config.length {
                assert!(config.packet.len() > config.length);
                config.packet.resize(config.length, 0);
                return Some(config.packet.clone());
            } else {
                config.length -= tail.len();
                config.is_length_calculated = true;
            }
        } else {
            config.length += sum(part);
        }

        return None;
    }

    if config.length > 0 {
        config.length -= part.len();
        config.packet.extend(part);
        None
    } else {
        Some(config.packet.clone())
    }
}

fn sum(v: Vec<u8>) -> usize {
    v.iter().fold(0usize, |acc, x| acc + *x as usize)
}
