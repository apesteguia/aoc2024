use std::io::BufRead;

use day2::shared::{self, best_of_two, increase_safe_idx, is_linear_idx, print_v};

fn main() {
    let reader = shared::get_input_reader().unwrap();

    let mut first_safe = 0;
    let mut second_safe = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let parsed = shared::parse(&line);

        let linear = shared::is_linear(&parsed);
        let increased_problem = shared::increase_safe(&parsed, 3);

        if linear && increased_problem {
            first_safe += 1;
        } else {
            let idx = match (linear, increased_problem) {
                (true, false) => {
                    //println!("Increase problem");
                    increase_safe_idx(&parsed, 3).1
                }
                (false, true) => {
                    //println!("Linear problem");
                    is_linear_idx(&parsed).1
                }
                (false, false) => {
                    println!("Both problem");
                    //is_linear_idx(&parsed).1
                    //print_v(&parsed);
                    //println!("");
                    best_of_two(&parsed)
                }
                (true, true) => {
                    println!("IMPOSIBLE");
                    println!("");
                    -1
                }
            };

            if idx != -1 {
                let mut new = parsed.clone();
                new.remove(idx as usize);

                let linear = shared::is_linear(&new);
                let increased_problem = shared::increase_safe(&new, 3);
                if linear && increased_problem {
                    second_safe += 1;
                }
            }
        }
    }

    println!("{} : {}", first_safe, second_safe);
    println!("Day 2, part 2: {}", second_safe + first_safe);
}
