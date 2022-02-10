use std::{
    cmp, env,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let size = env::args()
        .nth(1)
        .map(|v| usize::from_str_radix(&v, 10).unwrap())
        .unwrap_or(100);
    let steps = env::args()
        .nth(2)
        .map(|v| usize::from_str_radix(&v, 10).unwrap())
        .unwrap_or(100);
    let mut inp_grid = vec![vec![false; size]; size];
    let mut row = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        for (col, c) in line.chars().enumerate() {
            inp_grid[row][col] = c == '#';
        }

        row += 1;
    }

    let mut grid = inp_grid.clone();
    for _ in 0..steps {
        let mut new_grid = vec![vec![false; size]; size];

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let neighbors = count_neighbors(&grid, row, col);
                if grid[row][col] {
                    new_grid[row][col] = neighbors == 2 || neighbors == 3;
                } else {
                    new_grid[row][col] = neighbors == 3;
                }
            }
        }

        grid = new_grid;
    }

    let count = grid.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, &v| if v { acc + 1 } else { acc })
    });

    println!("part1: {}", count);

    //---------------------------part 2---------------------------------------

    let mut grid = inp_grid;
    grid[0][0] = true;
    grid[0][size - 1] = true;
    grid[size - 1][0] = true;
    grid[size - 1][size - 1] = true;

    for _ in 0..steps {
        let mut new_grid = vec![vec![false; size]; size];

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                let neighbors = count_neighbors2(&grid, row, col);
                if grid[row][col] {
                    new_grid[row][col] = neighbors == 2 || neighbors == 3;
                } else {
                    new_grid[row][col] = neighbors == 3;
                }
            }
        }

        grid = new_grid;
    }

    let count = grid.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, &v| if v { acc + 1 } else { acc })
    });

    println!("part2: {}", count);
}

fn count_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut count = 0;

    for r in row.saturating_sub(1)..=cmp::min(row.saturating_add(1), grid.len() - 1) {
        for c in col.saturating_sub(1)..=cmp::min(col.saturating_add(1), grid[r].len() - 1) {
            if r == row && c == col {
                continue;
            }

            if grid[r][c] {
                count += 1;
            }
        }
    }

    count
}

fn count_neighbors2(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut count = 0;

    for r in row.saturating_sub(1)..=cmp::min(row.saturating_add(1), grid.len() - 1) {
        for c in col.saturating_sub(1)..=cmp::min(col.saturating_add(1), grid[r].len() - 1) {
            if r == row && c == col {
                continue;
            }

            if grid[r][c] || is_corner(grid, row, col) {
                count += 1;
            }
        }
    }

    count
}

fn is_corner(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> bool {
    row == 0 && col == 0
        || row == 0 && col == grid[row].len() - 1
        || row == grid.len() - 1 && col == 0
        || row == grid.len() - 1 && col == grid[row].len() - 1
}

fn _print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for col in row {
            print!("{}", if *col { '#' } else { '.' });
        }

        println!();
    }
}
