#[cfg(test)]
mod tests;

use std::io;
use super::player::Player;
use super::position::Pos;

#[derive(Debug, PartialEq)]
pub struct PlayResult {
    pub player: Player,
    pub num_remain: i32,
    pub position: (char, char),
}

impl PlayResult {
    fn new() -> PlayResult {
        PlayResult {
            player: Player::None,
            num_remain: 0,
            position: ('a', 'A')
        }
    }

    fn with_game(game: &Game, pos: Pos) -> PlayResult {
        PlayResult {
            player: game.turn,
            num_remain: game.num_remain,
            position : pos.to_char(),
        }
    }
}

pub struct Game {
    turn: Player,
    num_remain: i32,
    board: [[Player; 19]; 19],
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Player::Black,
            num_remain: 1,
            board: [[Player::None; 19]; 19],
        }
    }

    pub fn play(&mut self, query: &str) -> Result<PlayResult, &'static str> {
        let position = match Pos::from(query) {
            Some(pos) => pos,
            None => return Err("Invalid Query")
        };

        let player = self.turn;
        if !self.set(position, player) {
            return Err("Already set position");
        }

        self.num_remain -= 1;
        let result = PlayResult::with_game(self, position);

        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }

        return Ok(result);
    }

    fn set(&mut self, pos: Pos, player: Player) -> bool {
        if !pos.validate() {
            return false;
        }

        let (row, col) = pos.to_usize();
        if self.board[row][col] != Player::None {
            return false;
        }

        self.board[row][col] = player;
        true
    }

    pub fn get_board(&self) -> &[[Player; 19]; 19] {
        &self.board
    }

    pub fn get_turn(&self) -> Player {
        self.turn
    }

    pub fn get_remain(&self) -> i32 {
        self.num_remain
    }

    pub fn print<'a>(&self, writer: &'a mut io::Write) -> io::Result<usize> {
        let mut paint = Paint::new(writer);
        paint.push(b"0 A B C D E F G H I J K L M N O P Q R S\n");

        for i in 0..19 {
            let row_name = [0x61 + i as u8, ' ' as u8];
            paint.push(&row_name);

            for j in 0..19 {
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
        use super::winner_searcher::search;
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