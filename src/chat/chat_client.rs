use {
    super::types::*,
    std::{
        io::{self, ErrorKind, Read, Write},
        net::{SocketAddr, TcpStream},
        sync::mpsc::{channel, Receiver, Sender, TryRecvError},
        thread,
    },
};

pub fn run_client(
    name: &str,
    server_ip: SocketAddr,
) -> io::Result<(Sender<String>, Receiver<WhatsUp>)> {
    let (tx_msg, rx_msg) = channel::<String>();
    let (tx_ext_msg, rx_ext_msg) = channel::<WhatsUp>();

    let mut client = TcpStream::connect(server_ip)?;
    client
        .set_nonblocking(true)
        .expect("failed to initiate non-blocking");

    thread::spawn(move || loop {
        let mut buf = vec![0; 32];
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

        match rx_msg.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(32, 0);
                client.write_all(&buff).unwrap();
            }
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }

        crate::sleep();
    });

    Ok((tx_msg, rx_ext_msg))
}
