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

    let mut count = 0;
    for &i in &left {
        count += i * right.iter().filter(|&f| *f == i).count() as i32;
    }

    println!("Day 1, part 2: {}", count);
    Ok(())
}
