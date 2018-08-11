#[cfg(test)]
mod tests;

use std::io;
use super::*;
use super::super::{BOARD_SIZE, Board};

#[derive(Debug, PartialEq)]
pub struct PlayResult {
    pub player: Player,
    pub num_remain: i32,
    pub position: (usize, usize),
}

impl PlayResult {
    fn new() -> PlayResult {
        PlayResult {
            player: Player::None,
            num_remain: 0,
            position: (0, 0),
        }
    }

    fn with_game(game: &Game, position: (usize, usize)) -> PlayResult {
        PlayResult {
            player: game.turn,
            num_remain: game.num_remain,
            position,
        }
    }
}

type Msg = &'static str;

pub struct Game {
    turn: Player,
    num_remain: i32,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Player::Black,
            num_remain: 1,
            board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn play(&mut self, pos: (usize, usize)) -> Result<PlayResult, Msg> {
        let (row, col) = pos;
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err("game::play invalid position")
        }

        if self.board[row][col] != Player::None {
            return Err("game::play already set position");
        }
        self.board[row][col] = self.turn;

        self.num_remain -= 1;
        let result = PlayResult::with_game(self, pos);

        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }
        Ok(result)
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn get_remain(&self) -> i32 {
        self.num_remain
    }

    pub fn print(&self, writer: &mut io::Write) -> io::Result<usize> {
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
        paint.write()
    }

    pub fn is_game_end(&self) -> Player {
        use super::search_winner::search;
        search(&self.board)
    }
}

struct Paint<'a> {
    vec: Vec<u8>,
    writer: &'a mut io::Write,
}

impl<'a> Paint<'a> {
    fn new(writer: &'a mut io::Write) -> Paint<'a> {
        Paint {
            vec: Vec::new(),
            writer,
        }
    }

    fn push(&mut self, data: &[u8]) {
        for elem in data {
            self.vec.push(*elem);
        }
    }

    fn push_one(&mut self, data: u8) {
        self.vec.push(data);
    }

    fn write(&mut self) -> io::Result<usize> {
        self.writer.write(&self.vec[..])
    }
}