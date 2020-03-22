use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read, Write},
    path::Path,
};

use dialoguer::Confirmation;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub nick: Option<String>,
}

impl Config {
    pub fn load_from(file_name: &str) -> io::Result<Self> {
        let mut file = File::open(file_name)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if let Ok(c) = bincode::deserialize::<Self>(&buf) {
            Ok(c)
        } else {
            if Confirmation::new()
                .with_text("Config file is damaged. Dou you want delete them?")
                .interact()
                .unwrap()
            {
                fs::remove_file(file_name).expect("Cannot remove damaged file");
                Err(ErrorKind::NotFound.into())
            } else {
                panic!("Config file must be repaired or removed!");
            }
        }
    }

    pub fn save(&self, file_name: &str) -> io::Result<()> {
        let data = bincode::serialize(&self).unwrap();
        let mut file = if Path::new(file_name).exists() {
            File::open(file_name)
        } else {
            File::create(file_name)
        }?;

        file.write_all(&data)
    }
}
