use std::{fs::File, io::Read, println};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_24").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let weights = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .sorted()
        .rev()
        .collect_vec();

    let total_weight = weights.iter().sum::<usize>();
    let weight_per_set = total_weight / 3;

    let mut solutions = Vec::new();
    find_smallest_set(&weights, &[], weight_per_set, &mut solutions);

    println!("found {} solutions for group 1 with size {}", solutions.len(), solutions.first().unwrap().len());

    let mut possible = solutions.iter().filter(|solution| {
        let leftover = weights.iter().filter(|weight| !solution.contains(*weight)).cloned().collect_vec();
        set_exists(&leftover, weight_per_set)
    }).collect_vec();

    println!("filtered to {} solutions where Group 2 & 3 exist", possible.len());

    possible.sort_by_cached_key(|solution| solution.iter().product::<usize>());

    println!("the solution with the lowest entanglement has {}", possible.first().unwrap().iter().product::<usize>())
}

pub fn part_two() {
    let mut file = File::open("input_24").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let weights = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .sorted()
        .rev()
        .collect_vec();

    let total_weight = weights.iter().sum::<usize>();
    let weight_per_set = total_weight / 4;

    let mut solutions = Vec::new();
    find_smallest_set(&weights, &[], weight_per_set, &mut solutions);

    println!("found {} solutions for group 1 with size {}", solutions.len(), solutions.first().unwrap().len());

    let mut possible = solutions.iter().filter(|solution| {
        println!("processing powerset");
        let leftover = weights.iter().filter(|weight| !solution.contains(*weight)).cloned().collect_vec();
        leftover.iter().powerset()
            .map(|subset| subset.iter().map(|element| **element).collect_vec())
            .filter(|subset| subset.iter().sum::<usize>() == 2 * weight_per_set)
            .any(|subset| set_exists(&subset, weight_per_set))
    }).collect_vec();

    println!("filtered to {} solutions where Group 2,3 & 4 exist", possible.len());

    possible.sort_by_cached_key(|solution| solution.iter().product::<usize>());

    println!("the solution with the lowest entanglement has {}", possible.first().unwrap().iter().product::<usize>())
}

fn find_smallest_set(set: &[usize], path: &[usize], difference: usize, solutions: &mut Vec<Vec<usize>>) {
    // return if the current path is too long to be optimal
    if let Some(solution) = solutions.first() {
        if path.len() > solution.len() { return; }
    }

    // check for solution and return if found
    if difference == 0 {

        // if bigger solutions exist, clear them
        if let Some(solution) = solutions.first() {
            if path.len() < solution.len() {
                // println!("found solution using {}. clearing {}", path.len(), solution.len());
                solutions.clear();
            }
        }

        solutions.push(path.to_vec());
        return;
    }

    // return if no more options are available
    if set.len() == 0 { return; }
    

    // if the next element fits, try using it
    let next = set[0];

    if next <= difference {
        find_smallest_set(&set[1..], &[path, &[next]].concat(), difference - next, solutions);
    }
    
    find_smallest_set(&set[1..], path, difference, solutions);
}

fn set_exists(set: &[usize], difference: usize) -> bool {
    // check for solution and return if found
    if difference == 0 {
        return  true;
    }

    // return if no more options are available
    if set.len() == 0 { return false; }

    // if the next element fits, try using it
    let next = set[0];

    if next <= difference {
        if set_exists(&set[1..], difference - next) {
            return true;
        }
    }
    
    set_exists(&set[1..], difference)
}
