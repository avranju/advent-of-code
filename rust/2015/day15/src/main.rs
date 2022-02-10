use std::cmp;
use std::io::{self, BufRead};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let re =
        Regex::new("([^:]+): capacity ([-0-9]+), durability ([-0-9]+), flavor ([-0-9]+), texture ([-0-9]+), calories ([-0-9]+)")
            .unwrap();
    let stdin = io::stdin();

    let mut ingredients = vec![];

    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
    {
        for cap in re.captures_iter(&line) {
            ingredients.push(Ingredient {
                capacity: i32::from_str_radix(&cap[2], 10).unwrap(),
                durability: i32::from_str_radix(&cap[3], 10).unwrap(),
                flavor: i32::from_str_radix(&cap[4], 10).unwrap(),
                texture: i32::from_str_radix(&cap[5], 10).unwrap(),
                calories: i32::from_str_radix(&cap[6], 10).unwrap(),
            });
        }
    }

    let mut total_score = 0;
    let mut calories_total_score = 0;
    for combo in (1..=100)
        .permutations(ingredients.len())
        .filter(|v| v.iter().fold(0, |acc, x| acc + x) == 100)
    {
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;
        for i in 0..ingredients.len() {
            capacity += ingredients[i].capacity * combo[i];
            durability += ingredients[i].durability * combo[i];
            flavor += ingredients[i].flavor * combo[i];
            texture += ingredients[i].texture * combo[i];
            calories += ingredients[i].calories * combo[i];
        }

        let score = cmp::max(0, capacity)
            * cmp::max(0, durability)
            * cmp::max(0, flavor)
            * cmp::max(0, texture);
        if score > total_score {
            total_score = score;
        }
        if calories == 500 && score > calories_total_score {
            calories_total_score = score;
        }
    }

    println!(
        "total_score = {}, calories_total_score = {}",
        total_score, calories_total_score
    );
}
