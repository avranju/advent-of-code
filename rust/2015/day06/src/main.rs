#[macro_use]
extern crate pest;

mod parser;
mod types;
mod part1;
mod part2;

fn main() {
    part2::run();
}
