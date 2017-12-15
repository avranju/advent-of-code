use std::cmp::Ordering;

#[derive(Eq)]
pub struct House {
    pub row: i32,
    pub col: i32,
}

impl House {
    pub fn new(row: i32, col: i32) -> House {
        House { row: row, col: col }
    }
}

impl Ord for House {
    fn cmp(&self, other: &House) -> Ordering {
        match self.row.cmp(&other.row) {
            Ordering::Equal => self.col.cmp(&other.col),
            ord => ord,
        }
    }
}

impl PartialOrd for House {
    fn partial_cmp(&self, other: &House) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for House {
    fn eq(&self, other: &House) -> bool {
        self.row == other.row && self.col == other.col
    }
}
