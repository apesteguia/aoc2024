use day4::shared::read_input;

const WORD: &str = "XMAS";

pub enum Position {
    UP,
    DOWN,
    LEFT,
    RIGTH,
    DrU,
    DrD,
    DlU,
    DlD,
}

fn has_xmas(mat: &[Vec<u8>], i: usize, j: usize) -> bool {
    let mut posibles: Vec<(usize, usize, Position)> = Vec::new();

    //primera iteracion y descartamos
    if (i as i32 - 1) >= 0 {
        let x = i - 1;
    }

    true
}

fn main() {
    let mat = read_input();
    // 140
    let len_x = mat[0].len();
    let len_y = mat.len();

    let mut w_idx: usize = 0;
    let mut counter = 0;

    for (i, row) in mat.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column as char == 'X' {
                match has_xmas(&mat, i, j) {
                    true => counter += 1,
                    false => (),
                }
            }
        }
    }

    println!("{} {}", len_x, len_y);
}
