use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn part_one() {
    let mut file = File::open("input_05").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let count = contents
        .lines()
        .filter(|line| {
            let vowels = line
                .chars()
                .filter(|char| vec!['a', 'e', 'i', 'o', 'u'].contains(char))
                .count()
                >= 3;
            let double_letters = line
                .chars()
                .zip(line.chars().skip(1))
                .filter(|(ch1, ch2)| ch1 == ch2)
                .count()
                >= 1;
            let no_bad_letters = line
                .chars()
                .zip(line.chars().skip(1))
                .filter(|chars| match chars {
                    ('a', 'b') => true,
                    ('c', 'd') => true,
                    ('p', 'q') => true,
                    ('x', 'y') => true,
                    _ => false,
                })
                .count()
                == 0;

            vowels && double_letters && no_bad_letters
        })
        .count();

    println!("{}", count);
}

pub fn part_two() {
    let mut file = File::open("input_05").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let count = contents
        .lines()
        .filter(|line| {
            let mut pairs: HashMap<(char, char), (i32, i32)> = HashMap::new();

            for (pos, pair) in line.chars().zip(line.chars().skip(1)).enumerate().map(|(pos, pair)| (pos as i32, pair)) {
                let (count, last_ocurance) = pairs.get(&pair).unwrap_or(&(0, -2));
                if pos - last_ocurance < 2 {
                    continue;
                }
                pairs.insert(pair, (count + 1, pos));
            }

            let repeated_pair = pairs.into_iter().any(|(_, (count, _))| count >= 2);

            let double_letters = line
                .chars()
                .zip(line.chars().skip(2))
                .any(|(ch1, ch2)| ch1 == ch2);

            repeated_pair && double_letters
        })
        .count();

    println!("{}", count);
}
