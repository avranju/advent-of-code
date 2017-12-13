use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut total_paper = 0u64;
    let mut total_ribbon = 0u64;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let numbers: Vec<u64> = line.split('x').map(|s| u64::from_str(s).unwrap()).collect();

        let (l, w, h) = (numbers[0], numbers[1], numbers[2]);
        let (lw, wh, hl) = (l * w, w * h, h * l);
        total_paper += (2 * lw) + (2 * wh) + (2 * hl) + lw.min(wh).min(hl);

        let side1 = (2 * l) + (2 * w);
        let side2 = (2 * w) + (2 * h);
        let side3 = (2 * l) + (2 * h);
        let cubic_feet = l * w * h;
        total_ribbon += side1.min(side2).min(side3) + cubic_feet;
    }

    println!("{} {}", total_paper, total_ribbon);
}
