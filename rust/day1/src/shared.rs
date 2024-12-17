use std::{fs::File, io::BufReader};

pub fn get_input_reader() -> std::io::Result<BufReader<File>> {
    let current = std::env::current_dir().unwrap();
    let path = current.join("input");

    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    Ok(reader)
}

pub fn parse(string: &str) -> (i32, i32) {
    let splitted: Vec<i32> = string
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();

    (*splitted.first().unwrap(), *splitted.last().unwrap())
}

pub fn distance(a: i32, b: i32) -> i32 {
    (a - b).abs()
}
