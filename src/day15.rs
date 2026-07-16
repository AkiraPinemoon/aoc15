use std::{fs::File, io::Read};

use itertools::Itertools;
use regex::Regex;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn part_one() {
    let mut file = File::open("input_15").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re =
        Regex::new(r"\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)")
            .unwrap();

    let ingredients = contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Ingredient {
                capacity: caps[1].parse().unwrap(),
                durability: caps[2].parse().unwrap(),
                flavor: caps[3].parse().unwrap(),
                texture: caps[4].parse().unwrap(),
                calories: caps[5].parse().unwrap(),
            }
        })
        .collect_vec();

    let (best_score, best_calories) = optimize_recipe(
        &ingredients,
        &ingredients.iter().map(|_| 0).collect_vec(),
        100,
        0,
    );

    println!(
        "Best score: {}, Best calories: {}",
        best_score, best_calories
    );
}

pub fn part_two() {
    let mut file = File::open("input_15").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let re =
        Regex::new(r"\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)")
            .unwrap();

    let ingredients = contents
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Ingredient {
                capacity: caps[1].parse().unwrap(),
                durability: caps[2].parse().unwrap(),
                flavor: caps[3].parse().unwrap(),
                texture: caps[4].parse().unwrap(),
                calories: caps[5].parse().unwrap(),
            }
        })
        .collect_vec();

    let (best_score, best_calories) = optimize_recipe_calorie_aware(
        &ingredients,
        &ingredients.iter().map(|_| 0).collect_vec(),
        100,
        0,
        500,
    )
    .unwrap();

    println!(
        "Best score: {}, Best calories: {}",
        best_score, best_calories
    );
}

fn optimize_recipe(
    ingredients: &[Ingredient],
    amounts: &[i32],
    teaspoons: i32,
    finished_ingredients: usize,
) -> (i32, i32) {
    if finished_ingredients == ingredients.len() - 1 {
        let mut new_amounts = amounts.to_vec();
        new_amounts[finished_ingredients] = teaspoons;
        return score_recipe(ingredients, &new_amounts);
    }

    let mut best_score = 0;
    let mut best_calories = 0;

    for i in 0..=teaspoons {
        let mut new_amounts = amounts.to_vec();
        new_amounts[finished_ingredients] = i;
        let (score, calories) = optimize_recipe(
            ingredients,
            &new_amounts,
            teaspoons - i,
            finished_ingredients + 1,
        );
        if score > best_score {
            best_score = score;
            best_calories = calories;
        }
    }

    (best_score, best_calories)
}

fn optimize_recipe_calorie_aware(
    ingredients: &[Ingredient],
    amounts: &[i32],
    teaspoons: i32,
    finished_ingredients: usize,
    max_calories: i32,
) -> Option<(i32, i32)> {
    if finished_ingredients == ingredients.len() - 1 {
        let mut new_amounts = amounts.to_vec();
        new_amounts[finished_ingredients] = teaspoons;
        let (score, calories) = score_recipe(ingredients, &new_amounts);
        if calories == max_calories {
            return Some((score, calories));
        }
        return None;
    }

    let mut best: Option<(i32, i32)> = None;

    for i in 0..=teaspoons {
        let mut new_amounts = amounts.to_vec();
        new_amounts[finished_ingredients] = i;

        let (_, calories) = score_recipe(ingredients, &new_amounts);
        if calories > max_calories {
            break;
        }

        if let Some((score, calories)) = optimize_recipe_calorie_aware(
            ingredients,
            &new_amounts,
            teaspoons - i,
            finished_ingredients + 1,
            max_calories,
        ) {
            if score > best.map(|(score, _)| score).unwrap_or(0) {
                best = Some((score, calories))
            }
        }
    }

    best
}

fn score_recipe(ingredients: &[Ingredient], amounts: &[i32]) -> (i32, i32) {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for (ingredient, &amount) in ingredients.iter().zip(amounts.iter()) {
        capacity += ingredient.capacity * amount;
        durability += ingredient.durability * amount;
        flavor += ingredient.flavor * amount;
        texture += ingredient.texture * amount;
        calories += ingredient.calories * amount;
    }

    capacity = capacity.max(0);
    durability = durability.max(0);
    flavor = flavor.max(0);
    texture = texture.max(0);

    let score = capacity * durability * flavor * texture;

    (score, calories)
}
