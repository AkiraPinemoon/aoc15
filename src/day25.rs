use std::{fs::File, io::Read, println};

use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_25").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(
        r"(\d+), column (\d+)",
    )
    .unwrap();

    let caps = re.captures(&contents).unwrap();

    let row: usize = caps[1].parse().unwrap();
    let col: usize = caps[2].parse().unwrap();

    println!("row: {}, col: {}", row, col);

    let first_code = 20151125;

    // get the row index if it were in col 1
    let associated_row = row + col - 1;

    // get associated row start number
    let associated_row_start = 1 + (1..associated_row).sum::<usize>();

    // get actual index
    let index = associated_row_start + col - 1;

    let mut code = first_code;
    for _ in 1..index {
        code = get_next_code(code);
    }

    println!("index: {}, code: {}", index, code);
}

fn get_next_code(mut code: usize) -> usize {
    code *= 252533;
    code %= 33554393;
    code
}