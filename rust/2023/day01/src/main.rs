use std::{
    env,
    io::{self, BufRead},
};

type Part = fn(String) -> u32;

fn main() {
    let part: Part = env::args()
        .nth(1)
        .map(|p| match p.as_str() {
            "part1" => Some::<Part>(part1),
            "part2" => Some::<Part>(part2),
            _ => None,
        })
        .flatten()
        .unwrap_or_else(|| {
            usage();
            std::process::exit(1);
        });

    let stdin = io::stdin();

    let val = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(part)
        .sum::<u32>();

    println!("{}", val);
}

fn part1(line: String) -> u32 {
    let n0 = line.chars().filter(|c| c.is_digit(10)).nth(0).unwrap();
    let n1 = line
        .chars()
        .rev()
        .filter(|c| c.is_digit(10))
        .next()
        .unwrap();
    format!("{n0}{n1}").parse::<u32>().unwrap()
}

fn part2(line: String) -> u32 {
    let n0 = line
        .char_indices()
        .map(|(i, c)| c.to_digit(10).or_else(|| parse_num(&line[0..=i])))
        .flatten()
        .nth(0)
        .unwrap();
    let n1 = line
        .char_indices()
        .rev()
        .map(|(i, c)| c.to_digit(10).or_else(|| parse_num(&line[i..])))
        .flatten()
        .nth(0)
        .unwrap();

    format!("{n0}{n1}").parse::<u32>().unwrap()
}

fn parse_num(s: &str) -> Option<u32> {
    let patterns = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, p) in patterns.iter().enumerate() {
        if s.contains(p) {
            return Some(i as u32 + 1);
        }
    }

    None
}

fn usage() {
    println!("Usage: {} <part1|part2>", env!("CARGO_PKG_NAME"));
}
