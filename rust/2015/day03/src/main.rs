use std::collections::BTreeSet;

mod house;
use house::House;

struct Santa {
    row: i32,
    col: i32,
}

impl Clone for Santa {
    fn clone(&self) -> Santa {
        Santa {
            row: self.row,
            col: self.col,
        }
    }
}

fn main() {
    let mut visited_houses = BTreeSet::new();

    let mut santas = vec![
        Santa {
            row: 0i32,
            col: 0i32,
        };
        2
    ];

    // add current house to visited houses since santa always delivers the gift
    // first to the current house
    visited_houses.insert(House::new(0i32, 0i32));

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    for (index, c) in input.char_indices() {
        let si = index % santas.len();
        let new_house = match c {
            '^' => House::new(santas[si].row - 1, santas[si].col),
            '>' => House::new(santas[si].row, santas[si].col + 1),
            'v' => House::new(santas[si].row + 1, santas[si].col),
            '<' => House::new(santas[si].row, santas[si].col - 1),
            _ => panic!("Unrecognized direction symbol found"),
        };

        santas[si].row = new_house.row;
        santas[si].col = new_house.col;
        visited_houses.replace(new_house);
    }

    println!("{}", visited_houses.len());
}
