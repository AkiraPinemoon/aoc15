use std::{fs::File, io::Read};

use regex::Regex;

pub fn part_one() {
    let mut file = File::open("input_12").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"-?\d+").unwrap();

    let sum = re
        .captures_iter(&contents)
        .map(|cap| cap[0].parse::<i32>().unwrap())
        .sum::<i32>();

    println!("Sum of all numbers: {}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_12").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json = serde_json::from_str::<serde_json::Value>(&contents).unwrap();

    let redless_sum = json.redless_sum();
    println!("Sum of all numbers (ignoring red): {}", redless_sum);
}

trait RedlessSum {
    fn redless_sum(&self) -> i32;
}

impl RedlessSum for serde_json::Value {
    fn redless_sum(&self) -> i32 {
        match self {
            serde_json::Value::Null => return 0,
            serde_json::Value::Bool(_) => return 0,
            serde_json::Value::Number(number) => return number.as_i64().unwrap_or(0) as i32,
            serde_json::Value::String(_) => return 0,
            serde_json::Value::Array(values) => {
                return values.iter().map(|v| v.redless_sum()).sum()
            }
            serde_json::Value::Object(map) => {
                if map.values().any(|v| v == "red") {
                    return 0;
                }
                return map.values().map(|v| v.redless_sum()).sum();
            }
        }
    }
}
