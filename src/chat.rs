use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
    thread,
};

mod comunication_types;
use comunication_types as types;

mod chat_client;
mod chat_server;

use chat_client::ChatClient;
use chat_server::ChatServer;

const HELP_MSG: &str = "Commands:
- q or quit - exit program
- c or close - close connection with a server
- # or join - join to server
- @ or prv - send private message to user
- ! or create - create server
- ? - show this help message

- ; or clear - clear screen
";

pub fn run_chat(name: String, port: u16) -> io::Result<()> {
    let stdin = io::stdin();
    let mut command_buf = String::new();
    let mut client: Option<ChatClient> = None;
    #[allow(unused_assignments)]
    let mut server: Option<ChatServer> = None;

    loop {
        stdin.read_line(&mut command_buf)?;
        let command = command_buf.as_bytes();

        if command[0] == b':' {
            let command = String::from_utf8(command.to_vec()).unwrap();
            let command = command.chars().skip(1).collect::<String>();
            let command = command.split(' ').map(|el| el.trim()).collect::<Vec<_>>();

            #[allow(unused_assignments)]
            match command[0] {
                "q" | "quit" => break,
                ";" | "clear" => println!("\x1B[2J"),
                "?" => println!("{}", HELP_MSG),
                "#" | "join" => {
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

                    let mut client_instance = ChatClient::new(name.clone(), server_socket)?;
                    client_instance.on_msg = Some(|msg| {
                        println!("{}: {}", msg.0, msg.1);
                    });

                    server = None;
                    client = Some(client_instance);
                }
                "!" | "create" => {
                    client = None;
                    server = Some(ChatServer::new(port));

                    if let Some(server_instance) = server {
                        thread::spawn(|| {
                            server_instance.serve().unwrap_or_else(|err| {
                                println!("{:?}", err);
                            });
                        });
                    }
                }
                _ => println!("Unknown command! Type :? to show help message"),
            }
        } else if let Some(client) = &client {
            client.send(command_buf.clone())?;
        }

        command_buf.clear();
    }

    Ok(())
}
