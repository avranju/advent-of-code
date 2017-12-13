use std::process::exit;
use std::env::args;

fn main() {
    let args = args();
    if args.len() < 2 {
        usage();
    }

    let input = args.skip(1)
        .map(|a| a.trim().to_owned())
        .fold(String::from(""), |acc, s| acc + &s);

    let mut floor = 0i32;
    let mut basement_index = -1i32;
    for (index, c) in input.char_indices() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        };

        if basement_index == -1 && floor == -1 {
            basement_index = index as i32;
        }
    }
    println!("{} {}", floor, basement_index + 1);
}

fn usage() {
    eprintln!("Usage: day01 <input>");
    exit(1);
}
