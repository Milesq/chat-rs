use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    sync::mpsc::{Sender, TryRecvError},
    thread,
};

mod comunication_types;
use comunication_types as types;

mod chat_client;
mod chat_server;

use chat_client::run_client;
use chat_server::run_server;

const HELP_MSG: &str = "Commands:
- q or quit - exit program
- c or close - close connection with a server
- # or join - join to server
- @ or prv - send private message to user
- ! or create - create server
- ? - show this help message

- ; or clear - clear screen
";

pub fn run_chat(name: &str, port: u16) -> io::Result<()> {
    let stdin = io::stdin();
    let mut command_buf = String::new();
    let mut message_sender: Option<Sender<String>> = None;
    let mut server_terminator: Option<&dyn Fn()> = None;

    loop {
        stdin.read_line(&mut command_buf)?;
        let command = command_buf.as_bytes();

        if command[0] == b':' {
            let command = String::from_utf8(command.to_vec()).unwrap();
            let command = command.chars().skip(1).collect::<String>();
            let command = command.split(' ').map(|el| el.trim()).collect::<Vec<_>>();

            match command[0] {
                "q" | "quit" => break,
                ";" | "clear" => println!("\x1B[2J"),
                "?" => println!("{}", HELP_MSG),
                "#" | "join" => {
                    if let Some(terminator) = server_terminator {
                        terminator();
                    }
                    let server_ip = if command.len() < 2 {
                        if cfg!(debug_assertions) {
                            String::from("127.0.0.1")
                        } else {
                            let mut buf = String::new();
                            println!("Type server ip");

                            stdin.read_line(&mut buf)?;

                            buf.trim().to_string()
                        }
                    } else {
                        command[1].to_string()
                    };

                    let server_socket =
                        SocketAddr::from((server_ip.parse::<Ipv4Addr>().unwrap(), port));

                    let (tx, new_msg) = run_client(name, server_socket)?;
                    message_sender = Some(tx);

                    thread::spawn(move || loop {
                        // display new messages
                        match new_msg.try_recv() {
                            Ok(msg) => {
                                println!("{}", msg);
                            }
                            Err(TryRecvError::Empty) => (),
                            Err(TryRecvError::Disconnected) => break,
                        }
                    });
                }
                "!" | "create" => {
                    message_sender = None;
                    server_terminator = Some(run_server(port).unwrap());
                }
                _ => println!("Unknown command! Type :? to show help message"),
            }
        } else if let Some(message_sender) = &message_sender {
            message_sender
                .send(command_buf.clone())
                .expect("Cannot send message");
        }

        command_buf.clear();
    }

    Ok(())
}
