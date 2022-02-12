use std::{
    collections::HashSet,
    io::{self, BufRead},
};

enum ParseState {
    ReplaceMap,
    Medicine,
}

fn main() {
    let stdin = io::stdin();
    let mut state = ParseState::ReplaceMap;
    let mut replacements = Vec::new();
    let mut medicine = String::new();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        match state {
            ParseState::ReplaceMap => {
                if line.is_empty() {
                    state = ParseState::Medicine;
                } else {
                    let mut parts = line.split(" => ");
                    let from = parts.next().unwrap();
                    let to = parts.next().unwrap();
                    replacements.push((from.to_string(), to.to_string()));
                }
            }
            ParseState::Medicine => {
                medicine = line;
            }
        }
    }

    println!("{}", part1(&replacements, &medicine));
}

fn part1(replacements: &Vec<(String, String)>, medicine: &str) -> usize {
    let mut molecules = HashSet::new();
    for (from, to) in replacements {
        let mut pos = 0;
        while let Some(npos) = medicine[pos..].find(from) {
            let mut new_medicine = medicine.to_string();
            new_medicine.replace_range((pos + npos)..(pos + npos + from.len()), &to);
            molecules.insert(new_medicine);
            pos += npos + from.len();
        }
    }

    molecules.len()
}
