use std::{fs::File, io::Read, println, todo, vec};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_22").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut values = contents
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|str| str.parse::<usize>().unwrap());

    let hitpoints = values.next().unwrap();
    let damage = values.next().unwrap();

    
}
