use std::io::{self, Write};

mod chat;
mod utils;

fn sleep() {
    use std::{thread, time::Duration};
    thread::sleep(Duration::from_millis(100));
}

const PACKET_SIZE: usize = 32;

fn main() {
    println!("Podaj nick: ");
    let mut nick = String::new();
    io::stdin().read_line(&mut nick).unwrap();

    chat::run_chat(nick.trim(), 6000).unwrap_or_else(|err| {
        utils::open_file("error-log.txt")
            .unwrap()
            .write_all(err.to_string().as_bytes())
            .unwrap();
    });
}
