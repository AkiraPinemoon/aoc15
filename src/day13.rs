use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use itertools::Itertools;
use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_13").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();

    let captures = re.captures_iter(&contents).collect::<Vec<_>>();

    let names = captures
        .iter()
        .map(|cap| cap[1].to_string())
        .collect::<HashSet<String>>();

    let happiness_effects = captures
        .iter()
        .map(|cap| {
            let name = cap[1].to_string();
            let positive = &cap[2] == "gain";
            let amount = cap[3].parse::<i32>().unwrap() * if positive { 1 } else { -1 };
            let other = cap[4].to_string();
            ((name, other), amount)
        })
        .collect::<HashMap<(String, String), i32>>();

    let arrangements = names
        .iter()
        .skip(1)
        .permutations(names.len() - 1)
        .map(|perm| {
            let mut arrangement = vec![names.iter().next().unwrap().clone()];
            arrangement.extend(perm.into_iter().cloned());
            arrangement
        });

    let max = arrangements
        .map(|arrangement| calculate_total_happiness(&arrangement, &happiness_effects))
        .max()
        .unwrap();

    println!("Max happiness: {}", max);
}

pub fn part_two() {
    let mut file = File::open("input_13").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).")
        .unwrap();

    let captures = re.captures_iter(&contents).collect::<Vec<_>>();

    let mut names = captures
        .iter()
        .map(|cap| cap[1].to_string())
        .collect::<HashSet<String>>();

    let mut happiness_effects = captures
        .iter()
        .map(|cap| {
            let name = cap[1].to_string();
            let positive = &cap[2] == "gain";
            let amount = cap[3].parse::<i32>().unwrap() * if positive { 1 } else { -1 };
            let other = cap[4].to_string();
            ((name, other), amount)
        })
        .collect::<HashMap<(String, String), i32>>();

    for name in names.iter() {
        happiness_effects.insert((name.clone(), "Me".to_string()), 0);
        happiness_effects.insert(("Me".to_string(), name.clone()), 0);
    }

    names.insert("Me".to_string());

    let arrangements = names
        .iter()
        .skip(1)
        .permutations(names.len() - 1)
        .map(|perm| {
            let mut arrangement = vec![names.iter().next().unwrap().clone()];
            arrangement.extend(perm.into_iter().cloned());
            arrangement
        });

    let max = arrangements
        .map(|arrangement| calculate_total_happiness(&arrangement, &happiness_effects))
        .max()
        .unwrap();

    println!("Max happiness: {}", max);
}

fn calculate_total_happiness(
    arrangement: &[String],
    happiness_effects: &HashMap<(String, String), i32>,
) -> i32 {
    let mut total_happiness = 0;

    for i in 0..arrangement.len() {
        let left = &arrangement[i];
        let right = &arrangement[(i + 1) % arrangement.len()];

        total_happiness += happiness_effects
            .get(&(left.clone(), right.clone()))
            .unwrap();
        total_happiness += happiness_effects
            .get(&(right.clone(), left.clone()))
            .unwrap();
    }

    total_happiness
}
