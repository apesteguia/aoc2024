use std::io::BufRead;

use day2::shared;

fn main() {
    let reader = shared::get_input_reader().unwrap();

    let mut safe_lines = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parsed = shared::parse(&line);

        if shared::is_linear(&parsed) && shared::increase_safe(&parsed, 3) {
            safe_lines += 1;
        }
    }

    println!("Day 2, part 1: {}", safe_lines);
}
