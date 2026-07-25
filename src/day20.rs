use std::{fs::File, io::Read, println};

pub fn part_one() {
    let mut file = File::open("input_20").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let target = contents.trim().parse::<usize>().unwrap() / 10;

    let mut giftsArray = Vec::with_capacity(target);
    giftsArray.extend((0..target).map(|_| 0));

    for i in 1..=target {
        for j in (i..=target).step_by(i) {
            giftsArray[j - 1] += i;
        }
    }

    let (result, gifts) = giftsArray
        .into_iter()
        .enumerate()
        .filter(|(_, gifts)| *gifts >= target)
        .next()
        .unwrap();

    println!("house {} got {}0 gifts", result + 1, gifts)
}

pub fn part_two() {
    let mut file = File::open("input_20").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let target = contents.trim().parse::<usize>().unwrap();

    let mut giftsArray = Vec::with_capacity(target / 11);
    giftsArray.extend((0..target).map(|_| 0));

    for i in 1..=target {
        for j in (i..=target).step_by(i).take(50) {
            giftsArray[j - 1] += 11 * i;
        }
    }

    let (result, gifts) = giftsArray
        .into_iter()
        .enumerate()
        .filter(|(_, gifts)| *gifts >= target)
        .next()
        .unwrap();

    println!("house {} got {} gifts", result + 1, gifts)
}
