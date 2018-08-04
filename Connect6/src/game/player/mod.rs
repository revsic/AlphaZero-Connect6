#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    Black = -1,
    White = 1,
    None = 0,
}

impl Player {
    pub fn switch(&self) -> Player {
        match self {
            &Player::Black => Player::White,
            &Player::White => Player::Black,
            &Player::None => Player::None
        }
    }

    pub fn mut_switch(&mut self) {
        *self = self.switch();
    }
}