use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read, sync::LazyLock};

pub fn part_one() {
    let mut file = File::open("input_07").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut instructions = contents.lines().map(parse_instruction).collect::<Vec<_>>();
    let mut values = std::collections::HashMap::<&str, u16>::new();

    while !values.contains_key("a") {
        instructions.retain(|instruction| {
            let dependencies = instruction.command.get_dependencies();
            if dependencies.iter().any(|dep| !values.contains_key(dep)) {
                return true;
            }

            let value = instruction.command.evaluate(&values);
            values.insert(instruction.product, value);
            println!("{} = {}", instruction.product, value);
            false
        });
    }

    println!("Value of a: {}", values["a"]);
}

pub fn part_two() {
    let mut file = File::open("input_07").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut instructions = contents.lines().map(parse_instruction).collect::<Vec<_>>();
    let mut values = std::collections::HashMap::<&str, u16>::new();

    while !values.contains_key("a") {
        instructions.retain(|instruction| {
            let dependencies = instruction.command.get_dependencies();
            if dependencies.iter().any(|dep| !values.contains_key(dep)) {
                return true;
            }

            let value = instruction.command.evaluate(&values);
            values.insert(instruction.product, value);
            println!("{} = {}", instruction.product, value);
            false
        });
    }

    let mut instructions = contents.lines().map(parse_instruction).collect::<Vec<_>>();
    instructions.retain(|instruction| instruction.product != "b");
    let old_a = values["a"];
    values.drain();
    values.insert("b", old_a);

    while !values.contains_key("a") {
        instructions.retain(|instruction| {
            let dependencies = instruction.command.get_dependencies();
            if dependencies.iter().any(|dep| !values.contains_key(dep)) {
                return true;
            }

            let value = instruction.command.evaluate(&values);
            values.insert(instruction.product, value);
            println!("{} = {}", instruction.product, value);
            false
        });
    }

    println!("Value of a: {}", values["a"]);
}

#[derive(Debug)]
enum Value<'a> {
    Literal(u16),
    Wire(&'a str),
}

impl<'a> Value<'a> {
    fn from_str(s: &'a str) -> Self {
        if let Ok(v) = s.parse::<u16>() {
            Value::Literal(v)
        } else {
            Value::Wire(s)
        }
    }

    fn get_dependencies(&self) -> Vec<&str> {
        match self {
            Value::Literal(_) => vec![],
            Value::Wire(wire) => vec![wire],
        }
    }

    fn get_value(&self, values: &std::collections::HashMap<&str, u16>) -> u16 {
        match self {
            Value::Literal(v) => *v,
            Value::Wire(wire) => values[wire],
        }
    }
}

#[derive(Debug)]
enum Command<'a> {
    Assignment(Value<'a>),
    Not(Value<'a>),
    Or(Value<'a>, Value<'a>),
    And(Value<'a>, Value<'a>),
    RShift(Value<'a>, u16),
    LShift(Value<'a>, u16),
}

impl Command<'_> {
    fn get_dependencies(&self) -> Vec<&str> {
        match self {
            Command::Assignment(a) => a.get_dependencies(),
            Command::Not(a) => a.get_dependencies(),
            Command::Or(a, b) => {
                let mut deps = a.get_dependencies();
                deps.extend(b.get_dependencies());
                deps
            }
            Command::And(a, b) => {
                let mut deps = a.get_dependencies();
                deps.extend(b.get_dependencies());
                deps
            }
            Command::RShift(a, _) => a.get_dependencies(),
            Command::LShift(a, _) => a.get_dependencies(),
        }
    }

    fn evaluate(&self, values: &std::collections::HashMap<&str, u16>) -> u16 {
        match self {
            Command::Assignment(v) => v.get_value(values),
            Command::Not(a) => !a.get_value(values),
            Command::Or(a, b) => a.get_value(values) | b.get_value(values),
            Command::And(a, b) => a.get_value(values) & b.get_value(values),
            Command::RShift(a, n) => a.get_value(values) >> n,
            Command::LShift(a, n) => a.get_value(values) << n,
        }
    }
}

struct Instruction<'a> {
    command: Command<'a>,
    product: &'a str,
}

static RE_VALUE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+|\d+) -> ([a-z]+)$").unwrap());
static RE_NOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^NOT ([a-z]+|\d+) -> ([a-z]+)$").unwrap());
static RE_OR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+|\d+) OR ([a-z]+|\d+) -> ([a-z]+)$").unwrap());
static RE_AND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+|\d+) AND ([a-z]+|\d+) -> ([a-z]+)$").unwrap());
static RE_RSHIFT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+|\d+) RSHIFT (\d+) -> ([a-z]+)$").unwrap());
static RE_LSHIFT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([a-z]+|\d+) LSHIFT (\d+) -> ([a-z]+)$").unwrap());

fn parse_instruction(line: &'_ str) -> Instruction<'_> {
    if let Some(caps) = RE_VALUE.captures(line) {
        return Instruction {
            command: Command::Assignment(Value::from_str(caps.get(1).unwrap().as_str())),
            product: caps.get(2).unwrap().as_str(),
        };
    } else if let Some(caps) = RE_NOT.captures(line) {
        return Instruction {
            command: Command::Not(Value::from_str(caps.get(1).unwrap().as_str())),
            product: caps.get(2).unwrap().as_str(),
        };
    } else if let Some(caps) = RE_OR.captures(line) {
        return Instruction {
            command: Command::Or(
                Value::from_str(caps.get(1).unwrap().as_str()),
                Value::from_str(caps.get(2).unwrap().as_str()),
            ),
            product: caps.get(3).unwrap().as_str(),
        };
    } else if let Some(caps) = RE_AND.captures(line) {
        return Instruction {
            command: Command::And(
                Value::from_str(caps.get(1).unwrap().as_str()),
                Value::from_str(caps.get(2).unwrap().as_str()),
            ),
            product: caps.get(3).unwrap().as_str(),
        };
    } else if let Some(caps) = RE_RSHIFT.captures(line) {
        return Instruction {
            command: Command::RShift(
                Value::from_str(caps.get(1).unwrap().as_str()),
                caps[2].parse::<u16>().unwrap(),
            ),
            product: caps.get(3).unwrap().as_str(),
        };
    } else if let Some(caps) = RE_LSHIFT.captures(line) {
        return Instruction {
            command: Command::LShift(
                Value::from_str(caps.get(1).unwrap().as_str()),
                caps[2].parse::<u16>().unwrap(),
            ),
            product: caps.get(3).unwrap().as_str(),
        };
    }

    panic!("Unknown instruction: {}", line);
}
