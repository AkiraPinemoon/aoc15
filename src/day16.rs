use std::{collections::HashMap, fs::File, io::Read};

use itertools::Itertools;
use regex::Regex;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn part_one() {
    let mut file = File::open("input_16").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let datapoints = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let re = Regex::new(
        "(trees|akitas|goldfish|samoyeds|vizslas|cars|perfumes|children|cats|pomeranians): (\\d+)",
    )
    .unwrap();

    let aunts = contents.lines().enumerate().map(|(i, line)| {
        let aunt_number = i + 1;
        let properties = re
            .captures_iter(line)
            .map(|caps| (caps[1].to_string(), caps[2].parse::<usize>().unwrap()))
            .collect_vec();

        (aunt_number, properties)
    });

    let passing_aunts = aunts
        .filter(|(aunt_number, properties)| {
            properties
                .iter()
                .all(|(key, value)| datapoints.get(key.as_str()).unwrap() == value)
        })
        .collect_vec();

    println!("Passing aunts: {:#?}", passing_aunts)
}

pub fn part_two() {
    let mut file = File::open("input_16").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let datapoints = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let re = Regex::new(
        "(trees|akitas|goldfish|samoyeds|vizslas|cars|perfumes|children|cats|pomeranians): (\\d+)",
    )
    .unwrap();

    let aunts = contents.lines().enumerate().map(|(i, line)| {
        let aunt_number = i + 1;
        let properties = re
            .captures_iter(line)
            .map(|caps| (caps[1].to_string(), caps[2].parse::<usize>().unwrap()))
            .collect_vec();

        (aunt_number, properties)
    });

    let passing_aunts = aunts
        .filter(|(aunt_number, properties)| {
            properties.iter().all(|(key, value)| match key.as_str() {
                "cats" | "trees" => datapoints.get(key.as_str()).unwrap() < value,
                "pomeranians" | "goldfish" => datapoints.get(key.as_str()).unwrap() > value,
                _ => datapoints.get(key.as_str()).unwrap() == value,
            })
        })
        .collect_vec();

    println!("Passing aunts: {:#?}", passing_aunts)
}
