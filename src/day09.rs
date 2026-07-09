use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn part_one() {
    let mut file = File::open("input_09").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();

    let mut places = HashSet::new();
    let mut distances = HashMap::new();

    for line in contents.lines() {
        let captures = re.captures(line).unwrap();
        let place1 = captures.get(1).unwrap().as_str();
        let place2 = captures.get(2).unwrap().as_str();
        let distance: u32 = captures.get(3).unwrap().as_str().parse().unwrap();

        places.insert(place1.to_string());
        places.insert(place2.to_string());

        distances.insert((place1.to_string(), place2.to_string()), distance);
        distances.insert((place2.to_string(), place1.to_string()), distance);
    }

    let shortest_route = find_shortest_route(Vec::new(), &places, &distances);

    println!(
        "Distance of the shortest route: {}",
        get_route_distance(&shortest_route, &distances)
    );
}

pub fn part_two() {
    let mut file = File::open("input_09").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();

    let mut places = HashSet::new();
    let mut distances = HashMap::new();

    for line in contents.lines() {
        let captures = re.captures(line).unwrap();
        let place1 = captures.get(1).unwrap().as_str();
        let place2 = captures.get(2).unwrap().as_str();
        let distance: u32 = captures.get(3).unwrap().as_str().parse().unwrap();

        places.insert(place1.to_string());
        places.insert(place2.to_string());

        distances.insert((place1.to_string(), place2.to_string()), distance);
        distances.insert((place2.to_string(), place1.to_string()), distance);
    }

    let longest_route = find_longest_route(Vec::new(), &places, &distances);

    println!(
        "Distance of the longest route: {}",
        get_route_distance(&longest_route, &distances)
    );
}

fn find_shortest_route(
    route: Vec<String>,
    places: &HashSet<String>,
    distances: &HashMap<(String, String), u32>,
) -> Vec<String> {
    if route.len() == places.len() {
        return route;
    }

    places
        .iter()
        .filter(|place| !route.contains(place))
        .map(|place| {
            let mut new_route = route.clone();
            new_route.push(place.clone());
            find_shortest_route(new_route, places, distances)
        })
        .min_by_key(|route| get_route_distance(&route, distances))
        .unwrap()
}

fn find_longest_route(
    route: Vec<String>,
    places: &HashSet<String>,
    distances: &HashMap<(String, String), u32>,
) -> Vec<String> {
    if route.len() == places.len() {
        return route;
    }

    places
        .iter()
        .filter(|place| !route.contains(place))
        .map(|place| {
            let mut new_route = route.clone();
            new_route.push(place.clone());
            find_longest_route(new_route, places, distances)
        })
        .max_by_key(|route| get_route_distance(&route, distances))
        .unwrap()
}

fn get_route_distance(route: &Vec<String>, distances: &HashMap<(String, String), u32>) -> u32 {
    route
        .windows(2)
        .map(|pair| distances.get(&(pair[0].clone(), pair[1].clone())).unwrap())
        .sum()
}
