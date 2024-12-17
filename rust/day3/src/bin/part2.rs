use std::fs;

use day3::shared;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let path = shared::input_path();
    let buffer = fs::read_to_string(&path)?;

    let regexes = vec![
        Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap(),
        Regex::new(r"do\(\)").unwrap(),
        Regex::new(r"don't\(\)").unwrap(),
    ];

    let mut matches: Vec<(usize, String)> = Vec::new();
    for (i, re) in regexes.iter().enumerate() {
        for mat in re.find_iter(&buffer) {
            matches.push((mat.start(), format!("Expresión{}: {}", i + 1, mat.as_str())));
        }
    }

    matches.sort_by_key(|k| k.0);
    let mut counter = 0;
    let mut enabled = true;

    let mul = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let doo = Regex::new(r"do\(\)").unwrap();
    let dont = Regex::new(r"don't\(\)").unwrap();

    for (pos, expr) in matches {
        //println!("Posición: {}, Coincidencia: {}", pos, expr);
        if let Some(cap) = mul.captures(&expr) {
            if enabled {
                let num1: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
                let num2: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
                counter += num1 * num2;
            }
        } else if let Some(_) = doo.captures(&expr) {
            enabled = true;
        } else if let Some(_) = dont.captures(&expr) {
            enabled = false;
        }
    }

    println!("Day 3, part 2 : {}", counter);

    Ok(())
}
