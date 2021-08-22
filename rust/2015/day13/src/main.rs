use std::collections::BTreeMap;
use std::io::{self, BufRead};

use itertools::Itertools;
use regex::Regex;

fn main() {
    let re =
        Regex::new("([^ ]+) would (gain|lose) ([0-9]+) happiness units by sitting next to ([^.]+)")
            .unwrap();
    let stdin = io::stdin();

    // BTreeMap<String, BTreeMap<String, i32>>
    let mut inp = BTreeMap::new();

    for line in stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
    {
        for cap in re.captures_iter(&line) {
            let name = &cap[1];
            let mut points = i32::from_str_radix(&cap[3], 10).unwrap();
            if &cap[2] == "lose" {
                points *= -1;
            }
            let name_to = &cap[4];

            let rel = inp.entry(name.to_string()).or_insert(BTreeMap::new());
            *rel.entry(name_to.to_string()).or_insert(points) = points;
        }
    }

    let mut max_happiness = 0 as i64;
    for set in inp.keys().permutations(inp.len()) {
        let mut happiness = 0 as i64;
        for (i, name) in set.iter().enumerate() {
            let rel = &inp[*name];

            if i > 0 {
                happiness += rel[set[i - 1]] as i64;
            } else {
                happiness += rel[set[set.len() - 1]] as i64;
            }

            if i < set.len() - 1 {
                happiness += rel[set[i + 1]] as i64;
            } else {
                happiness += rel[set[0]] as i64;
            }
        }

        if max_happiness < happiness {
            max_happiness = happiness;
        }
    }

    println!("---\nMax happiness = {}", max_happiness);
}
