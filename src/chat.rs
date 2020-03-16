use std::{
    io,
    net::{Ipv4Addr, SocketAddr},
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
    let mut server: Option<ChatServer> = None;

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
                    let server_ip = if command.len() < 2 {
                        let mut buf = String::new();
                        println!("Type server ip");

                        stdin.read_line(&mut buf)?;

                        buf.trim().to_string()
                    } else {
                        command[1].to_string()
                    };

                    let server_socket =
                        SocketAddr::from((server_ip.parse::<Ipv4Addr>().unwrap(), port));

                    // drop(server);
                    server = None;
                    client = Some(ChatClient::new(name.clone(), server_socket)?);
                }
                "!" | "create" => {
                    // drop(client);
                    client = None;
                    server = Some(ChatServer::new(port));
                    server.unwrap().serve()?;
                }
                _ => println!("Unknown command! Type :? to show help message"),
            }
        }

        command_buf.clear();
    }

    Ok(())
}
