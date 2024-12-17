use std::io::BufRead;

use day1::shared;

fn main() -> std::io::Result<()> {
    let reader = shared::get_input_reader().unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parsed = shared::parse(&line);
        left.push(parsed.0);
        right.push(parsed.1);
    }

    left.sort();
    right.sort();

    let mut count = 0;
    for (&a, &b) in left.iter().zip(right.iter()) {
        count += shared::distance(a, b);
    }

    println!("Day 1, part1: {}", count);
    Ok(())
}
