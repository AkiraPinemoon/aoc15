use std::{fs::File, io::Read};

use itertools::Itertools;

type Lights = [[bool; 100]; 100];

pub fn part_one() {
    let mut file = File::open("input_18").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lights: Lights = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == '#')
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();

    for _ in 0..100 {
        lights = step(&lights);
    }

    let turned_on_count: usize = lights
        .iter()
        .map(|line| line.iter().filter(|cell| **cell).count())
        .sum();

    println!("Turned on lights: {}", turned_on_count);
}

pub fn part_two() {
    let mut file = File::open("input_18").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lights: Lights = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == '#')
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();

    lights[0][0] = true;
    lights[0][99] = true;
    lights[99][0] = true;
    lights[99][99] = true;

    for _ in 0..100 {
        lights = step(&lights);
        lights[0][0] = true;
        lights[0][99] = true;
        lights[99][0] = true;
        lights[99][99] = true;
    }

    let turned_on_count: usize = lights
        .iter()
        .map(|line| line.iter().filter(|cell| **cell).count())
        .sum();

    println!("Turned on lights: {}", turned_on_count);
}

fn step(lights: &Lights) -> Lights {
    let mut new_state: Lights = [[false; 100]; 100];

    for y in 0..100 {
        for x in 0..100 {
            let alive_neighbours: usize = if y > 0 && x > 0 {
                lights[y - 1][x - 1].into()
            } else {
                0
            } + if y > 0 { lights[y - 1][x].into() } else { 0 }
                + if y > 0 && x < 99 {
                    lights[y - 1][x + 1].into()
                } else {
                    0
                }
                + if x > 0 { lights[y][x - 1].into() } else { 0 }
                + if x < 99 { lights[y][x + 1].into() } else { 0 }
                + if y < 99 && x > 0 {
                    lights[y + 1][x - 1].into()
                } else {
                    0
                }
                + if y < 99 { lights[y + 1][x].into() } else { 0 }
                + if y < 99 && x < 99 {
                    lights[y + 1][x + 1].into()
                } else {
                    0
                };
            new_state[y][x] = match (lights[y][x], alive_neighbours) {
                (true, 2 | 3) => true,
                (false, 3) => true,
                _ => false,
            }
        }
    }

    new_state
}
