use std::{fs::File, io::BufReader};

pub fn get_input_reader() -> std::io::Result<BufReader<File>> {
    let current = std::env::current_dir().unwrap();
    let path = current.join("input");

    let f = File::open(&path)?;
    let reader = BufReader::new(f);
    Ok(reader)
}

pub fn distance(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

pub fn parse(string: &str) -> Vec<i32> {
    string
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect()
}

pub fn is_linear(vec: &[i32]) -> bool {
    if vec.windows(2).all(|w| w[0] < w[1]) || vec.windows(2).all(|w| w[0] > w[1]) {
        true
    } else {
        false
    }
}

pub fn is_linear_idx(vec: &[i32]) -> (bool, i32) {
    if vec.len() < 2 {
        return (true, -1);
    }

    let mut direction = 0;

    for i in 0..vec.len() - 1 {
        if vec[i] < vec[i + 1] {
            if direction == 0 {
                direction = 1;
            } else if direction == -1 {
                return (false, i as i32);
            }
            // Si hay números iguales, no cambiamos la dirección
            if vec[i] == vec[i + 1] {
                return (false, i as i32);
            }
        } else if vec[i] > vec[i + 1] {
            if direction == 0 {
                direction = -1;
            } else if direction == 1 {
                return (false, i as i32);
            }
            // Si hay números iguales, no cambiamos la dirección
            if vec[i] == vec[i + 1] {
                return (false, i as i32);
            }
        }
    }

    (true, -1)
}

pub fn increase_safe(vec: &[i32], n: i32) -> bool {
    for w in vec.windows(2) {
        let d = distance(w[0], w[1]);
        if d > n || d == 0 {
            return false;
        }
    }

    true
}

pub fn increase_safe_idx(vec: &[i32], n: i32) -> (bool, i32) {
    for (i, w) in vec.windows(2).enumerate() {
        let d = distance(w[0], w[1]);
        if d > n || d == 0 {
            return (false, i as i32);
        }
    }

    (true, -1)
}

pub fn print_v(vec: &[i32]) {
    for &i in vec {
        print!("{} ", i);
    }
}

pub fn best_of_two(vec: &[i32]) -> i32 {
    let increase = increase_safe_idx(vec, 3).1;
    let linear = is_linear_idx(vec).1;

    if increase != linear {
        let mut increase_vec = vec.as_ref().to_owned().clone();
        let mut linear_vec = vec.as_ref().to_owned().clone();

        increase_vec.remove(increase as usize);
        println!("{:?}", vec);
        linear_vec.remove(linear as usize);

        if increase_safe(&increase_vec, 3) && is_linear(&increase_vec) {
            return increase;
        }

        if increase_safe(&linear_vec, 3) && is_linear(&linear_vec) {
            return linear;
        }
    }

    linear
}
