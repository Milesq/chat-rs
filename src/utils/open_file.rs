use std::{fs::File, io, path::Path};

pub fn open_file(path: &str) -> io::Result<File> {
    if Path::new(path).exists() {
        File::open(path)
    } else {
        File::create(path)
    }
}
