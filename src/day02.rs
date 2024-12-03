use std::{collections::HashMap, fs::File, io::Read};

pub fn part_one() {
    let mut file = File::open("input_02").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum: u32 = contents.lines()
        .map(|line| line.split('x'))
        .map(|mut dimensions| (
            dimensions.next().unwrap().parse::<u32>().unwrap(),
            dimensions.next().unwrap().parse::<u32>().unwrap(),
            dimensions.next().unwrap().parse::<u32>().unwrap()
        ))
        .map(|(x, y, z)| {
            let a1 = x * y;
            let a2 = x * z;
            let a3 = y * z;

            let smallest =if a1 < a2 {
                if a1 < a3 {
                    a1
                } else {
                    a3
                }
            } else {
                if a2 < a3 {
                    a2
                } else {
                    a3
                }
            };

            2 * (a1 + a2 + a3) + smallest
        })
        .sum();

    println!("{}", sum);
}

pub fn part_two() {
    let mut file = File::open("input_02").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sum: u32 = contents.lines()
        .map(|line| line.split('x').map(|dim| dim.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .map(|mut dimensions| { dimensions.sort(); dimensions })
        .map(|dimensions| (
            *dimensions.get(0).unwrap(),
            *dimensions.get(1).unwrap(),
            *dimensions.get(2).unwrap(),
        ))
        .map(|(x, y, z)| {
            2 * (x + y) + x * y * z
        })
        .sum();

    println!("{}", sum);
}
