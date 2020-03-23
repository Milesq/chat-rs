use {
    super::types::*,
    std::{
        io::{self, ErrorKind, Read, Write},
        net::{SocketAddr, TcpStream},
        sync::{
            mpsc::{channel, Receiver, Sender, TryRecvError},
            Arc, Mutex,
        },
        thread,
    },
};

use super::prepare_request::*;

fn serialize(req: &Request) -> Vec<u8> {
    bincode::serialize(req).unwrap()
}

pub fn run_client(
    name: String,
    server_ip: SocketAddr,
) -> io::Result<(Sender<String>, Receiver<WhatsUp>)> {
    // let mut participants = Vec::new();
    let messages = Vec::<WhatsUp>::new();

    let (tx_raw_msg, rx_raw_msg) = channel::<Vec<u8>>();
    let (tx_user_msg, rx_user_msg) = channel::<String>();
    let (_tx_ext_msg, rx_ext_msg) = channel::<WhatsUp>();

    tx_raw_msg
        .send(serialize(&Request {
            req_type: ReqType::AddParticipant,
            user_name: name.clone(),
        }))
        .expect("Cannot pass data to sender");

    let tx_raw_msg = Arc::new(Mutex::new(tx_raw_msg));

    let mut client = TcpStream::connect(server_ip)?;
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    let mut packet_config = PreparePacketConfig::new();

    let tx_raw_msg_whatsup = Arc::clone(&tx_raw_msg);
    let name_whatsup = name.clone();
    thread::spawn(move || loop {
        let tx_raw_msg = tx_raw_msg_whatsup.lock().unwrap();
        let req = Request {
            user_name: name_whatsup.clone(),
            req_type: ReqType::WhatsUp(messages.len()),
        };
        (*tx_raw_msg).send(serialize(&req)).unwrap();

        thread::sleep(std::time::Duration::from_millis(250));
    });

    let tx_raw_msg = Arc::clone(&tx_raw_msg);
    thread::spawn(move || loop {
        let mut buf = vec![0; crate::PACKET_SIZE];
        match client.read_exact(&mut buf) {
            Ok(_) => {
                if let Some(buf) = prepare_to_receive(buf, &mut packet_config) {
                    packet_config = Default::default();
                    let resp = bincode::deserialize::<Response>(&buf)
                        .expect("Expected new messages")
                        .expect("Server error");

                    println!("{:?}", buf);
                    // tx_ext_msg.send(resp).unwrap();
                }
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }

        match rx_user_msg.try_recv() {
            Ok(msg) => {
                let packet = serialize(&Request {
                    user_name: name.clone(),
                    req_type: ReqType::SendMessage(msg.trim().to_string()),
                });

                (*tx_raw_msg.lock().unwrap())
                    .send(packet)
                    .expect("Cannot pass data to sender");
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
