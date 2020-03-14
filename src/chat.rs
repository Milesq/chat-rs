use std::io;

mod chat_client;
mod chat_server;

const HELP_MSG: &str = "Commands:
- q or quit - exit program
- c or close - close connection with a server
- # or join - join to server
- @[user] or prv-[user] - send private message to user
- ![name] - create server
- ? - show this help message

- ; or clear - clear screen
";

pub fn run_chat() -> io::Result<()> {
    let stdin = io::stdin();
    let mut command_buf = String::new();

    loop {
        stdin.read_line(&mut command_buf)?;
        let command = command_buf.as_bytes();

        if command[0] == ':' as u8 {
            let command = &command_buf[1..];

            match command.trim() {
                "q" | "quit" => break,
                ";" | "clear" => println!("\x1B[2J"),
                "?" => println!("{}", HELP_MSG),
                _ => println!("Unknown command! Type :? to show help message"),
            }
        }

        command_buf.clear();
    }

    Ok(())
}
