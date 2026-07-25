use std::{fs::File, io::Read, println, todo, vec};

use itertools::Itertools;

pub fn part_one() {
    let mut file = File::open("input_21").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut values = contents
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|str| str.parse::<usize>().unwrap());

    let boss_hp = values.next().unwrap();
    let boss_damage = values.next().unwrap();
    let boss_armor = values.next().unwrap();

    let weapons = vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];

    let armor = vec![
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];

    let rings = vec![
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];

    let armor_options = armor
        .iter()
        .map(|item| vec![item])
        .chain(vec![Vec::new()].into_iter());

    let ring_options = (0..=2).flat_map(|n| rings.iter().combinations(n));

    let total_options = weapons
        .iter()
        .cartesian_product(armor_options)
        .cartesian_product(ring_options)
        .map(|((weapon, armor), rings)| {
            let mut items = vec![weapon];
            items.extend(armor);
            items.extend(rings);
            items
        });

    let winning_options =
        total_options.filter(|items| is_battle_won(items, boss_hp, boss_damage, boss_armor));

    let best_cost = winning_options
        .map(|items| items.iter().map(|item| item.cost).sum::<usize>())
        .sorted()
        .next()
        .unwrap();

    println!("the best winning option cost {} gold", best_cost)
}

pub fn part_two() {
    let mut file = File::open("input_21").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut values = contents
        .lines()
        .map(|line| line.split_whitespace().last().unwrap())
        .map(|str| str.parse::<usize>().unwrap());

    let boss_hp = values.next().unwrap();
    let boss_damage = values.next().unwrap();
    let boss_armor = values.next().unwrap();

    let weapons = vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];

    let armor = vec![
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];

    let rings = vec![
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];

    let armor_options = armor
        .iter()
        .map(|item| vec![item])
        .chain(vec![Vec::new()].into_iter());

    let ring_options = (0..=2).flat_map(|n| rings.iter().combinations(n));

    let total_options = weapons
        .iter()
        .cartesian_product(armor_options)
        .cartesian_product(ring_options)
        .map(|((weapon, armor), rings)| {
            let mut items = vec![weapon];
            items.extend(armor);
            items.extend(rings);
            items
        });

    let losing_options =
        total_options.filter(|items| !is_battle_won(items, boss_hp, boss_damage, boss_armor));

    let worst_cost = losing_options
        .map(|items| items.iter().map(|item| item.cost).sum::<usize>())
        .sorted()
        .last()
        .unwrap();

    println!("the worst losing option cost {} gold", worst_cost)
}

pub fn is_battle_won(
    items: &Vec<&Item>,
    boss_hp: usize,
    boss_damage: usize,
    boss_armor: usize,
) -> bool {
    let damage = items.iter().map(|item| item.damage).sum::<usize>();
    let armor = items.iter().map(|item| item.armor).sum::<usize>();

    let player_hitrate = 1.max(damage as isize - boss_armor as isize) as usize;
    let boss_hitrate = 1.max(boss_damage as isize - armor as isize) as usize;

    let player_win_time = boss_hp / player_hitrate;
    let boss_win_time = 100 / boss_hitrate;

    player_win_time <= boss_win_time
}

struct Item {
    name: String,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    pub fn new(name: &str, cost: usize, damage: usize, armor: usize) -> Self {
        Item {
            name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }
}
