use std::{
    env,
    io::{self, BufRead},
};

fn main() {
    let inp = io::stdin()
        .lock()
        .lines()
        .map(|l| i32::from_str_radix(&l.unwrap(), 10).unwrap())
        .collect::<Vec<_>>();

    let available_eggnog = env::args()
        .nth(1)
        .map(|v| i32::from_str_radix(&v, 10).unwrap())
        .unwrap_or(150);

    part1(&inp, available_eggnog);
    part2(&inp, available_eggnog);
}

fn part1(inp: &Vec<i32>, available_eggnog: i32) {
    let mut vals = vec![];
    let mut combinations = 0;

    let mut cb = |v: &[i32]| {
        let sum = v.iter().fold(0, &|acc, v| acc + v);
        if sum == available_eggnog {
            combinations += 1;
        }
    };

    doit(0, &inp, &mut vals, &mut cb);
    println!("part1: {combinations}");
}

fn part2(inp: &Vec<i32>, available_eggnog: i32) {
    let mut vals = vec![];
    let mut min_len = inp.len();
    let mut combinations = 0;

    let mut cb = |v: &[i32]| {
        let sum = v.iter().fold(0, &|acc, v| acc + v);
        if sum == available_eggnog {
            if min_len > v.len() {
                min_len = v.len();
                combinations = 0;
            }

            if min_len == v.len() {
                combinations += 1;
            }
        }
    };

    doit(0, &inp, &mut vals, &mut cb);
    println!("part2: {combinations}");
}

fn doit<F>(start: usize, inp: &Vec<i32>, vals: &mut Vec<i32>, cb: &mut F)
where
    F: FnMut(&[i32]) -> (),
{
    for i in start..inp.len() {
        vals.push(inp[i]);
        cb(&vals);
        doit(i + 1, inp, vals, cb);
        vals.pop();
    }
}
