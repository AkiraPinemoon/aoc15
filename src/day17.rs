use std::{fs::File, io::Read};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_17").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let target_volume = 150;

    let containers = contents.lines().map(|line| line.parse::<usize>().unwrap());

    let combinations = containers
        .powerset()
        .filter(|containers| containers.iter().cloned().sum::<usize>() == target_volume)
        .count();

    println!("Possible combinations: {}", combinations)
}

pub fn part_two() {
    let mut file = File::open("input_17").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let target_volume = 150;

    let containers = contents.lines().map(|line| line.parse::<usize>().unwrap());

    let mut needed_length = None;

    let combinations = containers
        .powerset()
        .filter(|containers| {
            if let Some(needed) = needed_length {
                if needed < containers.len() {
                    return false;
                }
            }
            if containers.iter().cloned().sum::<usize>() == target_volume {
                needed_length = Some(containers.len());
                return true;
            }
            false
        })
        .count();

    println!("Possible combinations: {}", combinations)
}
