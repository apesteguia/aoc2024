use std::fs;

use day3::shared;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let path = shared::input_path();
    let buffer = fs::read_to_string(&path)?;

    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    let mut total_sum = 0;
    let mut occurrences = 0;

    for cap in re.captures_iter(&buffer) {
        let num1: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let num2: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

        total_sum += num1 * num2;
        occurrences += 1;

        println!("mul({}, {})", num1, num2);
    }

    println!("Occurrences: {}", occurrences);
    println!("Day 3, part 1: {}", total_sum);

    Ok(())
}
