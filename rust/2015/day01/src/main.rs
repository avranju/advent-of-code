fn main() {
    let mut floor = 0i32;
    let mut basement_index = -1i32;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

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
