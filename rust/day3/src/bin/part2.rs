use std::fs;

use day3::shared;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let path = shared::input_path();
    let buffer = fs::read_to_string(&path)?;

    let mul_re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut total_sum = 0;
    let mut mul_enabled = true;

    for line in buffer.lines() {
        // Check for do() and don't() instructions
        println!("{}", line);
        if do_re.is_match(line) {
            mul_enabled = true;
        } else if dont_re.is_match(line) {
            mul_enabled = false;
        }

        // Find and process mul instructions
        for cap in mul_re.captures_iter(line) {
            if mul_enabled {
                let num1: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                let num2: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

                total_sum += num1 * num2;
            }
        }
    }

    println!("Total sum of enabled multiplications: {}", total_sum);

    Ok(())
}
