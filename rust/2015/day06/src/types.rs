#[derive(Debug)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug)]
pub enum Instruction {
    TurnOff((Coord, Coord)),
    TurnOn((Coord, Coord)),
    Toggle((Coord, Coord)),
}
