use std::{collections::{HashMap, HashSet}, fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_03").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut visited = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited.insert((x, y));

    for char in contents.chars() {
        match char {
            '^' => y -= 1,
            'v' => y += 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!(),
        }
        visited.insert((x, y));
    }

    println!("{}", visited.len());
}

pub fn part_two() {
    let mut file = File::open("input_03").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut visited = HashSet::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robot_x = 0;
    let mut robot_y = 0;
    visited.insert((santa_x, santa_y));
    visited.insert((robot_x, robot_y));

    for (i, char) in contents.chars().enumerate() {
        if i % 2 == 0 {
            match char {
                '^' => santa_y -= 1,
                'v' => santa_y += 1,
                '<' => santa_x -= 1,
                '>' => santa_x += 1,
                _ => panic!(),
            }
            visited.insert((santa_x, santa_y));
        } else {
            match char {
                '^' => robot_y -= 1,
                'v' => robot_y += 1,
                '<' => robot_x -= 1,
                '>' => robot_x += 1,
                _ => panic!(),
            }
            visited.insert((robot_x, robot_y));
        }
    }

    println!("{}", visited.len());
}
