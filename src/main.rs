use std::io::Write;

mod chat;
mod utils;

fn sleep() {
    use std::{thread, time::Duration};
    thread::sleep(Duration::from_millis(100));
}

const PACKET_SIZE: usize = 128;

fn main() {
    chat::run_chat("Milesq", 6000).unwrap_or_else(|err| {
        utils::open_file("error-log.txt")
            .unwrap()
            .write_all(err.to_string().as_bytes())
            .unwrap();
    });
}
