use regex::Regex;
use std::{fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_08").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r#"\\(\\|"|x\w\w)"#).unwrap();

    let difference = contents
        .lines()
        .map(|line| {
            let quotes = 2;
            let escaped: usize = re
                .captures_iter(line)
                .map(|capture| match capture[1].chars().nth(0).unwrap() {
                    '\\' | '"' => 1,
                    'x' => 3,
                    _ => panic!("Unexpected escape sequence"),
                })
                .sum();
            quotes + escaped
        })
        .sum::<usize>();

    println!("Difference: {}", difference);
}

pub fn part_two() {
    let mut file = File::open("input_08").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let difference = contents
        .lines()
        .map(|line| {
            let quotes = 2;
            let escaped = line
                .chars()
                .filter(|char| ['"', '\\'].contains(char))
                .count();
            quotes + escaped
        })
        .sum::<usize>();

    println!("Difference: {}", difference);
}
