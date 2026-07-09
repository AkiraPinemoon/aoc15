use regex::Regex;
use std::{cmp::max, fs::File, io::Read};

#[derive(Debug)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

pub fn part_one() {
    let mut file = File::open("input_06").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    let instructions = contents
        .lines()
        .map(|line| {
            re.captures(line)
                .map(|cap| {
                    let command = match &cap[1] {
                        "turn on" => Command::TurnOn,
                        "turn off" => Command::TurnOff,
                        "toggle" => Command::Toggle,
                        _ => panic!("Unknown command"),
                    };
                    let x1 = cap[2].parse::<usize>().unwrap();
                    let y1 = cap[3].parse::<usize>().unwrap();
                    let x2 = cap[4].parse::<usize>().unwrap();
                    let y2 = cap[5].parse::<usize>().unwrap();
                    (command, (x1, y1), (x2, y2))
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            let mut light_on = false;
            for (command, (x1, y1), (x2, y2)) in instructions.iter() {
                if x >= *x1 && x <= *x2 && y >= *y1 && y <= *y2 {
                    match command {
                        Command::TurnOn => light_on = true,
                        Command::TurnOff => light_on = false,
                        Command::Toggle => light_on = !light_on,
                    }
                }
            }

            if light_on {
                count += 1;
            }
        }
    }

    println!("{}", count);
}

pub fn part_two() {
    let mut file = File::open("input_06").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    let instructions = contents
        .lines()
        .map(|line| {
            re.captures(line)
                .map(|cap| {
                    let command = match &cap[1] {
                        "turn on" => Command::TurnOn,
                        "turn off" => Command::TurnOff,
                        "toggle" => Command::Toggle,
                        _ => panic!("Unknown command"),
                    };
                    let x1 = cap[2].parse::<usize>().unwrap();
                    let y1 = cap[3].parse::<usize>().unwrap();
                    let x2 = cap[4].parse::<usize>().unwrap();
                    let y2 = cap[5].parse::<usize>().unwrap();
                    (command, (x1, y1), (x2, y2))
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut count = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            let mut brightness = 0;
            for (command, (x1, y1), (x2, y2)) in instructions.iter() {
                if x >= *x1 && x <= *x2 && y >= *y1 && y <= *y2 {
                    match command {
                        Command::TurnOn => brightness += 1,
                        Command::TurnOff => brightness = max(0, brightness - 1),
                        Command::Toggle => brightness += 2,
                    }
                }
            }

            count += brightness;
        }
    }

    println!("{}", count);
}
