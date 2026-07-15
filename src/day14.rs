use std::{fs::File, io::Read};

use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_14").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(
        r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let reindeer = contents.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        Reindeer {
            speed: caps[1].parse().unwrap(),
            fly_time: caps[2].parse().unwrap(),
            rest_time: caps[3].parse().unwrap(),
        }
    });

    let total_time = 2503;

    let max_distance = reindeer
        .map(|r| {
            let cycle_time = r.fly_time + r.rest_time;
            let full_cycles = total_time / cycle_time;
            let remaining_time = total_time % cycle_time;

            let distance = full_cycles * r.speed * r.fly_time
                + r.speed * std::cmp::min(remaining_time, r.fly_time);

            distance
        })
        .max()
        .unwrap();

    println!("Max distance: {}", max_distance);
}

pub fn part_two() {
    let mut file = File::open("input_14").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(
        r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let reindeer = contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Reindeer {
                speed: caps[1].parse().unwrap(),
                fly_time: caps[2].parse().unwrap(),
                rest_time: caps[3].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let total_time = 2503;

    let mut distances = vec![0; reindeer.len()];
    let mut scores = vec![0; reindeer.len()];

    for i in 0..total_time {
        for (j, r) in reindeer.iter().enumerate() {
            let cycle_time = r.fly_time + r.rest_time;
            let time_in_cycle = i % cycle_time;

            if time_in_cycle < r.fly_time {
                distances[j] += r.speed;
            }
        }

        let max_distance = *distances.iter().max().unwrap();
        for (j, &distance) in distances.iter().enumerate() {
            if distance == max_distance {
                scores[j] += 1;
            }
        }
    }

    let max_score = scores.iter().max().unwrap();
    println!("Max score: {}", max_score);
}

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}
