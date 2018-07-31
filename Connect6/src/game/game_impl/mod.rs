#[cfg(test)]
mod tests;

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

    pub fn print(&self) {
        fn idx2alpha(idx: u8) -> char {
            return (idx + 0x61) as char;
        }

        println!("0 A B C D E F G H I J K L M N O P Q R S");
        for i in 0..19 {
            print!("{} ", idx2alpha(i as u8));
            for j in 0..19 {
                match self.board[i][j] {
                    Player::Black => print!("X "),
                    Player::White => print!("O "),
                    Player::None => print!("_ "),
                }
            }
            println!();
        }
    }

    pub fn is_game_end(&self) -> Player {
        use super::winner_searcher::search;
        search(&self.board)
    }
}
