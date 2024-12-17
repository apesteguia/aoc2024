use std::{fs::File, io::BufReader, path::PathBuf};

pub fn get_input_reader() -> std::io::Result<BufReader<File>> {
    let current = std::env::current_dir().unwrap();
    let path = current.join("input");

    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    Ok(reader)
}

pub fn input_path() -> PathBuf {
    let current = std::env::current_dir().unwrap();
    let path = current.join("input");
    path
}
