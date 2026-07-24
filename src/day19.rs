use std::{collections::HashSet, fmt::Display, fs::File, io::Read, vec};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_19").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines().rev();
    let target_str = lines.next().unwrap();
    let rules = lines
        .skip(1)
        .map(|line| {
            let (from_str, to_str) = line.split_once(" => ").unwrap();
            let from = Atom(from_str.to_string());
            let to = parse_molecule(to_str);
            Replacement(from, to)
        })
        .collect_vec();

    let target = parse_molecule(target_str);

    let replacement_count = rules
        .into_iter()
        .flat_map(|rule| {
            let mut replacements_for_rule = Vec::new();
            for i in 0..target.len() {
                if target[i] != rule.0 {
                    continue;
                }
                let mut applied = target[0..i].to_vec();
                applied.append(&mut rule.1.clone());
                applied.append(&mut target[i + 1..].to_vec());
                replacements_for_rule.push(applied);
            }
            replacements_for_rule
        })
        .unique()
        .count();

    println!("There are {} valid replacements", replacement_count);
}

pub fn part_two() {
    let mut file = File::open("input_19").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = contents.lines().rev();
    let target_str = lines.next().unwrap();

    let target = parse_molecule(target_str);

    let brackets = target
        .iter()
        .filter(|atom| vec![Atom("Rn".to_string()), Atom("Ar".to_string())].contains(*atom))
        .count();

    let commas = target
        .iter()
        .filter(|atom| Atom("Y".to_string()) == **atom)
        .count();

    let steps = target.len() - brackets - 2 * commas - 1;

    println!("making the cure taes {} steps", steps);
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Atom(String);

type Molecule = Vec<Atom>;

fn parse_molecule(molecule: &str) -> Molecule {
    molecule
        .chars()
        .peekable()
        .batching(|it| {
            let first = it.next()?;
            let mut atom = String::new();
            atom.push(first);

            while let Some(&c) = it.peek() {
                if c.is_ascii_lowercase() {
                    atom.push(it.next().unwrap());
                } else {
                    break;
                }
            }

            Some(Atom(atom))
        })
        .collect()
}

struct Replacement(Atom, Vec<Atom>);
