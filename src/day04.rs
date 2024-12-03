use std::{collections::{HashMap, HashSet}, fs::File, io::Read};

pub fn part_one() {
    let input = "yzbqklnj";

    let mut i = 0;

    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        if format!("{:#?}", hash).starts_with("00000") {
            break;
        }
        i += 1;
    }

    println!("{}", i);
}

pub fn part_two() {
    let input = "yzbqklnj";

    let mut i = 0;

    loop {
        let hash = md5::compute(format!("{}{}", input, i));
        if format!("{:#?}", hash).starts_with("000000") {
            break;
        }
        i += 1;
    }

    println!("{}", i);
}
