use std::{fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_10").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result = contents.trim().to_string();
    for _ in 0..40 {
        result = look_and_say(result);
    }

    println!("Result length: {}", result.len());
}

pub fn part_two() {
    let mut file = File::open("input_10").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result = contents.trim().to_string();
    for _ in 0..50 {
        result = look_and_say(result);
    }

    println!("Result length: {}", result.len());
}

fn look_and_say(input: String) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next_c) = chars.peek() {
            if next_c == c {
                count += 1;
                chars.next();
            } else {
                break;
            }
        }
        result.push_str(&format!("{}{}", count, c));
    }
    result
}
