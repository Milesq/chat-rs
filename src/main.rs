// use std::env;

use std::io::Write;

mod chat;
mod utils;

fn main() {
    chat::run_chat("Milesq".into(), 54923).unwrap_or_else(|err| {
        utils::open_file("error-log.txt")
            .unwrap()
            .write_all(err.to_string().as_bytes())
            .unwrap();
    });

    // let args: Vec<String> = env::args().collect();

    // chat::run_chat(args.get(1).map(|x| x.clone()));
}
