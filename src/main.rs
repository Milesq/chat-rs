use std::io::{self, ErrorKind, Write};

mod chat;
mod config;
mod utils;

use config::Config;

fn sleep() {
    use std::{thread, time::Duration};
    thread::sleep(Duration::from_millis(100));
}

const PACKET_SIZE: usize = 32;
const CONFIG_FILE_NAME: &str = "chat-user-data.bin";

fn main() -> io::Result<()> {
    let config_file_name = dirs::home_dir().unwrap().join(CONFIG_FILE_NAME);
    let config_file_name = config_file_name.to_str().unwrap();

    let config = match Config::load_from(config_file_name) {
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let mut nick = String::new();
            println!("Podaj nick: ");
            io::stdin().read_line(&mut nick)?;
            nick = nick.trim().to_string();

            let config = Config { nick: Some(nick) };
            config.save(config_file_name)?;
            config
        }
        rest => rest?,
    };

    chat::run_chat(config.nick.unwrap().trim(), 6000).unwrap_or_else(|err| {
        utils::open_file("error-log.txt")
            .unwrap()
            .write_all(err.to_string().as_bytes())
            .unwrap();
    });

    Ok(())
}
