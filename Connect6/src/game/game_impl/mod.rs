//! Implementation of Game Connect6
//!
//! It defines the game connect6 with some visualization utilities.
//!
//! # Examples
//! ```rust
//! let mut game = Game::new();
//! let result = game.play((0, 0));
//! game.print(&mut std::io::stdout()).unwrap();
//!
//! let winner = game.is_game_end();
//! if winner != Player::None {
//!     println!("game end, winner: {:?}", winner);
//! } else {
//!     println!("playing result: {:?}", result);
//! }
//! ```
use std::io;
use super::*;
use super::super::{BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

/// Result of playing game for each turn.
#[derive(Debug, PartialEq)]
pub struct PlayResult {
    pub player: Player,
    pub num_remain: i32,
    pub position: (usize, usize),
}

impl PlayResult {
    /// Construct a new `PlayResult`
    fn new() -> PlayResult {
        PlayResult {
            player: Player::None,
            num_remain: 0,
            position: (0, 0),
        }
    }

    /// Construct a `PlayResult` with given game state and position.
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let position = (0, 0);
    ///
    /// let play_result = PlayResult::new(&game, position);
    /// assert_eq!(play_result, PlayResult{ player: Player::Black, num_remain: 1, position: (0, 0) });
    /// ```
    fn with_game(game: &Game, position: (usize, usize)) -> PlayResult {
        PlayResult {
            player: game.turn,
            num_remain: game.num_remain,
            position,
        }
    }
}

type Msg = &'static str;

/// Define connect6 game status
pub struct Game {
    turn: Player,
    num_remain: i32,
    board: Board,
}

impl Game {
    /// Construct a new `Game`
    pub fn new() -> Game {
        Game {
            turn: Player::Black,
            num_remain: 1,
            board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// Set the stone of current player with given position, zero-indexed (row, col).
    ///
    /// # Examples
    /// ```rust
    /// let mut game = Game::new();
    /// let result = game.play((3, 4));
    /// assert_eq!(result.unwrap(), PlayResult{ player: Player::Black, num_remain: 0, position: (3, 4) });
    /// ```
    ///
    /// # Errors
    /// 1. If given position out of board.
    /// 2. If other stone place already in given position.
    pub fn play(&mut self, pos: (usize, usize)) -> Result<PlayResult, Msg> {
        let (row, col) = pos;
        // position param validation
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err("game::play invalid position")
        }
        // in-board validation
        if self.board[row][col] != Player::None {
            return Err("game::play already set position");
        }
        self.board[row][col] = self.turn;

        self.num_remain -= 1;
        let result = PlayResult::with_game(self, pos);

        // if turn end, switch player
        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }
        Ok(result)
    }

    /// Return board
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Return current player
    pub fn get_turn(&self) -> Player {
        self.turn
    }

    /// Return num_remain
    pub fn get_remain(&self) -> i32 {
        self.num_remain
    }

    /// Print the board status
    ///
    /// # Examples
    /// ```rust
    /// let mut game = Game::new();
    /// let result = game.play((3, 4)).unwrap(); // black
    /// let result = game.play((3, 3)).unwrap(); // white
    ///
    /// game.print(&mut std::io::stdout());
    /// ```
    /// Expected results
    /// ```
    /// 0 A B C D E F G H I J K L M N O P Q R S
    /// a _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// b _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// c _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// d _ _ _ O X _ _ _ _ _ _ _ _ _ _
    /// e _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// f _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// g _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// h _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// i _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// j _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// k _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// l _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// m _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// n _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// o _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    /// ```
    pub fn print(&self, writer: &mut io::Write) -> io::Result<usize> {
        // generate ascii canvas
        let mut paint = Paint::new(writer);
        paint.push(b"0 A B C D E F G H I J K L M N O P Q R S\n");

        for i in 0..BOARD_SIZE {
            let row_name = [0x61 + i as u8, ' ' as u8];
            paint.push(&row_name);

            for j in 0..BOARD_SIZE {
                match self.board[i][j] {
                    Player::Black => paint.push(b"X "),
                    Player::White => paint.push(b"O "),
                    Player::None => paint.push(b"_ "),
                }
            }
            paint.push_one('\n' as u8);
        }
        // make output to writer
        paint.write()
    }

    /// Return game winner if game end, else Player::None
    ///
    /// # Examples
    /// ```rust
    /// let mut game = Game::new();
    /// game.play((3, 4)).unwrap();
    /// assert_eq!(game.is_game_end(), Player::None);
    /// ```
    pub fn is_game_end(&self) -> Player {
        use super::search_winner::search;
        search(&self.board)
    }
}

/// Simple ascii buffer
///
/// # Examples
///```rust
/// let mut stdout = std::io::stdout();
/// let mut paint = Paint::new(&mut stdout);
/// paint.push(b"ABC");
/// paint.push_one('\n' as u8);
/// paint.write();
/// ```
/// Expected results
/// ```
/// ABC
///
/// ```
struct Paint<'a> {
    vec: Vec<u8>,
    writer: &'a mut io::Write,
}

impl<'a> Paint<'a> {
    /// Construct a new `Paint`.
    ///
    /// # Examples
    /// ```rust
    /// let mut stdout = std::io::stdout();
    /// let mut paint = Paint::new(&mut stdout);
    /// ```
    fn new(writer: &'a mut io::Write) -> Paint<'a> {
        Paint {
            vec: Vec::new(),
            writer,
        }
    }

    /// Push a byte slice to the buffer
    ///
    /// # Examples
    /// ```rust
    /// let mut stdout = std::io::stdout();
    /// let mut paint = Paint::new(&mut stdout);
    /// paint.push(b"ABC");
    /// ```
    fn push(&mut self, data: &[u8]) {
        for elem in data {
            self.vec.push(*elem);
        }
    }

    /// Push a single u8 to the buffer
    ///
    /// # Examples
    /// ```rust
    /// let mut stdout = std::io::stdout();
    /// let mut paint = Paint::new(&mut stdout);
    /// paint.push_one('\n' as u8);
    /// ```
    fn push_one(&mut self, data: u8) {
        self.vec.push(data);
    }

    /// Write buffer to the io stream
    ///
    /// # Examples
    /// ```rust
    /// let mut stdout = std::io::stdout();
    /// let mut paint = Paint::new(&mut stdout);
    /// paint.push(b"ABC");
    /// paint.push_one('\n' as u8);
    /// paint.write();
    /// ```
    /// Expected results
    /// ```
    /// ABC
    ///
    /// ```
    fn write(&mut self) -> io::Result<usize> {
        self.writer.write(&self.vec[..])
    }
}