use std::default::Default;

#[cfg(test)]
mod tests;

/// enum `Player`, describe {Black: -1, None: 0, White: 1}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Player {
    Black = -1,
    None = 0,
    White = 1,
}

impl Player {
    /// Switching it, Black to White, White to Black
    ///
    /// # Examples
    /// ```rust
    /// let player = Player::Black;
    /// assert_eq!(player.switch(), Player::White);
    /// ```
    pub fn switch(&self) -> Player {
        match self {
            &Player::Black => Player::White,
            &Player::White => Player::Black,
            &Player::None => Player::None
        }
    }

    /// mutable Switch
    ///
    /// # Examples
    /// ```rust
    /// let mut player = Player::Black;
    /// player.mut_switch();
    /// assert_eq!(player, Player::White);
    /// ```
    pub fn mut_switch(&mut self) {
        *self = self.switch();
    }
}

impl Default for Player {
    fn default() -> Player { Player::None }
}