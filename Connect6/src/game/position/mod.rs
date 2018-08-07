#[cfg(test)]
pub mod tests;
use super::super::BOARD_SIZE;

const END_LOWER: char = (BOARD_SIZE as u8 + 0x61) as char;
const END_UPPER: char = (BOARD_SIZE as u8 + 0x41) as char;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Row(char);

impl Row {
    fn to_char(&self) -> char {
        self.0
    }

    fn to_usize(&self) -> usize {
        self.0 as usize - 0x61
    }

    fn validate(&self) -> bool {
        self.0 >= 'a' && self.0 <= END_LOWER
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Col(char);

impl Col {
    fn to_char(&self) -> char {
        self.0
    }

    fn to_usize(&self) -> usize {
        self.0 as usize - 0x41
    }

    fn validate(&self) -> bool {
        self.0 >= 'A' && self.0 <= END_UPPER
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos(Row, Col);

impl Pos {
    pub fn from(query: &str) -> Option<Pos> {
        if query.len() != 2 {
            return None;
        }

        let mut qchars = query.chars();
        let row = match qchars.next() {
            Some(c) => Row(c),
            None => return None
        };
        let col = match qchars.next() {
            Some(c) => Col(c),
            None => return None
        };
        let pos = Pos(row, col);
        if !pos.validate() {
            return None;
        }

        Some(pos)
    }

    pub fn to_char(&self) -> (char, char) {
        (self.0.to_char(), self.1.to_char())
    }

    pub fn to_usize(&self) -> (usize, usize) {
        (self.0.to_usize(), self.1.to_usize())
    }

    pub fn validate(&self) -> bool {
        self.0.validate() && self.1.validate()
    }
}