use std::{fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_11").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut password = contents.trim().to_string();

    while !check_straight(&password) || !check_forbidden(&password) || !check_pairs(&password) {
        password = increment_password(&password);
    }

    println!("The next valid password is: {}", password);
}

pub fn part_two() {
    let mut file = File::open("input_11").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut password = contents.trim().to_string();

    while !check_straight(&password) || !check_forbidden(&password) || !check_pairs(&password) {
        password = increment_password(&password);
    }

    password = increment_password(&password);

    while !check_straight(&password) || !check_forbidden(&password) || !check_pairs(&password) {
        password = increment_password(&password);
    }

    println!("The next valid password is: {}", password);
}

fn increment_password(password: &str) -> String {
    let mut password_chars: Vec<char> = password.chars().collect();
    let mut index = password_chars.len() - 1;

    loop {
        if password_chars[index] == 'z' {
            password_chars[index] = 'a';
            if index == 0 {
                break;
            }
            index -= 1;
        } else {
            password_chars[index] = ((password_chars[index] as u8) + 1) as char;
            break;
        }
    }

    password_chars.into_iter().collect()
}

fn check_straight(password: &str) -> bool {
    let password_chars: Vec<char> = password.chars().collect();
    for i in 0..password_chars.len() - 2 {
        if (password_chars[i] as u8 + 1 == password_chars[i + 1] as u8)
            && (password_chars[i + 1] as u8 + 1 == password_chars[i + 2] as u8)
        {
            return true;
        }
    }
    false
}

fn check_forbidden(password: &str) -> bool {
    !password.contains('i') || !password.contains('o') || !password.contains('l')
}

fn check_pairs(password: &str) -> bool {
    let password_chars: Vec<char> = password.chars().collect();
    let mut pairs = 0;
    let mut i = 0;
    while i < password_chars.len() - 1 {
        if password_chars[i] == password_chars[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    pairs >= 2
}
