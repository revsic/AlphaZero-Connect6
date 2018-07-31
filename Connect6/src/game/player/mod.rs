#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Black,
    White,
    None,
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