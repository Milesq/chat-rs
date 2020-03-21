use {
    super::types::*,
    std::{
        io::{self, ErrorKind, Read, Write},
        net::{SocketAddr, TcpStream},
        sync::mpsc::{channel, Receiver, Sender, TryRecvError},
        thread,
    },
};

use super::prepare_request::*;

pub fn run_client(
    name: &str,
    server_ip: SocketAddr,
) -> io::Result<(Sender<String>, Receiver<WhatsUp>)> {
    let (tx_raw_msg, rx_raw_msg) = channel::<Vec<u8>>();
    let (tx_user_msg, rx_user_msg) = channel::<String>();
    let (tx_ext_msg, rx_ext_msg) = channel::<WhatsUp>();

    tx_raw_msg
        .send(bincode::serialize(&ReqType::AddParticipant(name.to_string())).unwrap())
        .expect("Cannot pass data to sender");

    let mut client = TcpStream::connect(server_ip)?;
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    thread::spawn(move || loop {
        let mut buf = vec![0; crate::PACKET_SIZE];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                println!("message recv {:?}", buf);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx_user_msg.try_recv() {
            Ok(msg) => {
                let packet = bincode::serialize(&ReqType::SendMessage(msg)).unwrap();
                tx_raw_msg.send(packet).expect("Cannot pass data to sender");
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        match rx_raw_msg.try_recv() {
            Ok(packet) => {
                for packet in prepare_to_send(packet) {
                    client
                        .write_all(&packet[..])
                        .expect("Cannot send TCP packet");
                }
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        crate::sleep();
    });

    Ok((tx_user_msg, rx_ext_msg))
}
