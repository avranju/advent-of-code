use crate::parser::parse;
use crate::types::{Coord, Instruction};
use std::cmp;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn run() {
    let mut grid = Grid::new(1000, 1000);

    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        if line.len() > 0 {
            let instruction = parse(&line).unwrap();
            match instruction {
                Instruction::TurnOn((c1, c2)) => grid.turn_on(&c1, &c2),
                Instruction::TurnOff((c1, c2)) => grid.turn_off(&c1, &c2),
                Instruction::Toggle((c1, c2)) => grid.toggle(&c1, &c2),
            }
        }
    }

    println!(
        "Brightness: {}",
        grid.values.iter().fold(0, |acc, &x| acc + x)
    );
}

struct Grid {
    width: u32,
    values: Vec<i32>,
}

impl Grid {
    fn new(width: u32, height: u32) -> Self {
        Grid {
            width,
            values: vec![0; (width * height) as usize],
        }
    }

    fn to_index(&self, c: &Coord) -> usize {
        (c.x * self.width + c.y) as usize
    }

    fn map_cell<F>(&mut self, from: &Coord, to: &Coord, cb: F)
    where
        F: Fn(&Self, usize) -> i32,
    {
        for x in from.x..=to.x {
            for y in from.y..=to.y {
                let index = self.to_index(&Coord::new(x, y));
                self.values[index] = cb(self, index);
            }
        }
    }

    fn turn_on(&mut self, from: &Coord, to: &Coord) {
        self.map_cell(from, to, |g, index| g.values[index] + 1)
    }

    fn turn_off(&mut self, from: &Coord, to: &Coord) {
        self.map_cell(from, to, |g, index| cmp::max(0, g.values[index] - 1))
    }

    fn toggle(&mut self, from: &Coord, to: &Coord) {
        self.map_cell(from, to, |g, index| g.values[index] + 2)
    }
}
