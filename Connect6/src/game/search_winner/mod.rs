//! Algorithm implementation for finding winner of the Connect6.
//!
//! Algorithm finds the continuous 6 stones on 4 directions, vertical, horizontal, two diagonals.
//! Use dynamic programming to implement algorithm and swaping memories to obtain the memory efficiency.
//!
//! # Examples
//! ```rust
//! let mut game = Game::new();
//! game.play((3, 4)).unwrap();
//!
//! let winner = search(game.get_board());
//! assert_eq!(winner, Player::None);
//! ```
use super::Player;

use super::super::{BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

/// Searching direction.
///
/// For top-left to bottom-right search, only four direction is required to find the continuous 6 stones.
/// Right-Horizontal, Down-Vertial, RightDown-Diagonal, LeftDown-Diagonal.
#[derive(Copy, Clone, Debug)]
enum Path {
    Right,
    Down,
    RightDown,
    LeftDown,
}

/// Number of the continuous stones for each directions.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Cumulative {
    right: i32,
    down: i32,
    right_down: i32,
    left_down: i32,
}

impl Cumulative {
    /// Construct a new Cumulative
    fn new() -> Cumulative {
        Cumulative {
            right: 0,
            down: 0,
            right_down: 0,
            left_down: 0,
        }
    }

    /// Get a sum of specific path
    ///
    /// # Examples
    /// ```rust
    /// let mut cum = Cumulative::new();
    /// cum.right = 10;
    /// assert_eq!(cum.get(&Path::Right), 10);
    /// ```
    fn get(&self, path: &Path) -> i32 {
        match path {
            &Path::Right => self.right,
            &Path::Down => self.down,
            &Path::RightDown => self.right_down,
            &Path::LeftDown => self.left_down,
        }
    }

    /// Get a sum mutable reference of specific path
    ///
    /// # Examples
    /// ```rust
    /// let mut cum = Cumulative::new();
    /// *cum.get_mut(&Path::Right) = 10;
    /// assert_eq!(cum.right, 10);
    /// ```
    fn get_mut(&mut self, path: &Path) -> &mut i32 {
        match path {
            &Path::Right => &mut self.right,
            &Path::Down => &mut self.down,
            &Path::RightDown => &mut self.right_down,
            &Path::LeftDown => &mut self.left_down,
        }
    }
}

/// Swapable two Cumulative arrays for dynamic programming
///
/// `flag` represent index of previous arrays.
/// Method swap can be implemented as just flip the flag bit.
struct Block {
    flag: usize,
    mem: [[Cumulative; BOARD_SIZE+2]; 2],
}

impl Block {
    /// Construct a new Block.
    fn new() -> Block {
        Block {
            flag: 0,
            mem: [[Cumulative::new(); BOARD_SIZE+2]; 2],
        }
    }

    /// Get a tuple representation of block, (prev, current).
    fn as_tuple(&self) -> (&[Cumulative; BOARD_SIZE+2], &[Cumulative; BOARD_SIZE+2]) {
        let f = self.flag;
        (&self.mem[f], &self.mem[1 - f])
    }

    /// Get a previous `Cumulative` cell for specific direction.
    ///
    /// If right direction is given, left cell of current column is return,
    /// if down direction is given, upper cell of current column is return,
    /// if left_down direction is given, upper-right cell of current column is return,
    /// if right_down direction is given, upper-left cell of current column is return.
    ///
    /// # Examples
    /// ```rust
    /// let block = Block::new();
    /// let (prev, current) = block.as_tuple();
    /// let result = block.get_prev(1, &Path::Right);
    /// assert_eq!(result, current[0]);
    /// ```
    fn get_prev(&self, col: usize, path: &Path) -> &Cumulative {
        let (prev, now) = self.as_tuple();
        match path {
            &Path::Right => &now[col - 1],
            &Path::Down => &prev[col],
            &Path::RightDown => &prev[col - 1],
            &Path::LeftDown => &prev[col + 1],
        }
    }

    /// Update current row with given update rule
    ///
    /// # Examples
    /// ```rust
    /// let mut block = Block::new();
    /// block.update_now(|row| row.iter_mut(|c| *c.get_mut(&Path::Right) = 1));
    /// ```
    fn update_now<F>(&mut self, update: F)
        where F: Fn(&mut [Cumulative; BOARD_SIZE+2])
    {
        let f = self.flag;
        let now = &mut self.mem[1 - f];
        update(now);
    }

    /// Swap the row and clear the current.
    ///
    /// # Examples
    /// ```rust
    /// let mut block = Block::new();
    /// let current_backup = {
    ///     let (_, current) = block.as_tuple();
    ///     *current
    /// };
    /// block.update_row();
    /// let (prev, current) = block.as_tuple();
    /// assert_eq!(*prev, current_backup);
    /// assert_eq!(*current, [Cumulative::new(); BOARD_SIZE+2]);
    /// ```
    fn update_row(&mut self) {
        self.flag = 1 - self.flag;
        let now = &mut self.mem[1 - self.flag];

        for i in 0..BOARD_SIZE+2 {
            now[i] = Cumulative::new();
        }
    }
}

/// Algorithm implementation for finding winner of the Connect6.
///
/// Algorithm finds the continuous 6 stones on 4 directions, vertical, horizontal, two diagonals.
/// Use dynamic programming to implement algorithm and swaping memories to obtain the memory efficiency.
///
/// # Examples
/// ```rust
/// let mut game = Game::new();
/// game.play((3, 4)).unwrap();
///
/// let winner = search(game.get_board());
/// assert_eq!(winner, Player::None);
/// ```
pub fn search(table: &Board) -> Player {
    let mut black = Block::new();
    let mut white = Block::new();

    // update the block if cell has stones
    fn path_iter(block: &mut Block, col: usize) -> bool {
        // convert to one-indexed array, for convenience
        let col = col + 1;
        let paths = [Path::Right, Path::Down, Path::RightDown, Path::LeftDown];

        for path in paths.iter() {
            // update with previous cell
            let updated = block.get_prev(col, path).get(path) + 1;
            // find continuous six stones
            if updated >= 6 { return true; }

            block.update_now(
                |now| *now[col].get_mut(path) = updated);
        }
        false
    }

    for row in 0..BOARD_SIZE {
        black.update_row();
        white.update_row();

        for col in 0..BOARD_SIZE {
            match table[row][col] {
                Player::None => continue,
                Player::Black =>
                    if path_iter(&mut black, col) { return Player::Black; },
                Player::White =>
                    if path_iter(&mut white, col) { return Player::White; },
            };
        }
    }

    Player::None
}
