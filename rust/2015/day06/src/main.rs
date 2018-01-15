#[macro_use]
extern crate pest;

mod parser;
mod part1;
mod part2;
mod types;

fn main() {
    part2::run();
}
