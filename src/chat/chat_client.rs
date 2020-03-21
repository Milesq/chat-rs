use {
    super::types::*,
    std::{
        io::{self, ErrorKind, Read, Write},
        net::{SocketAddr, TcpStream},
        sync::mpsc::{channel, Receiver, Sender, TryRecvError},
        thread,
    },
};

const MAX: usize = std::u8::MAX as usize;

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
                for packet in prepare_packet(packet) {
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

fn prepare_packet(packet: Vec<u8>) -> Vec<Vec<u8>> {
    let mut ret = Vec::new();
    let mut len = packet.len();

    while len > MAX {
        len -= MAX;
        ret.push(MAX as u8);
    }

    ret.push(len as u8);
    ret.push(0);

    ret.extend(packet);

    let mut ret = ret
        .chunks(crate::PACKET_SIZE)
        .map(|el| Vec::from(el))
        .collect::<Vec<_>>();

    for el in &mut ret {
        el.resize(crate::PACKET_SIZE, 0);
    }

    ret
}
