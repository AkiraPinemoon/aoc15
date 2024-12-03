use std::{collections::HashMap, fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_01").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut floor = 0;

    for char in contents.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!(),
        }
    }

    println!("{}", floor);
}

pub fn part_two() {
    let mut file = File::open("input_01").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut floor = 0;

    for (pos, char) in contents.chars().enumerate() {
        match char {
            '(' => floor += 1,
            ')' => {
                if floor == 0 {
                    println!("{}", pos + 1);
                    return;
                }
                floor -= 1
            },
            _ => panic!(),
        }
    }
}
