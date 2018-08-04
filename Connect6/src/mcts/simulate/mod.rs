extern crate cpython;

use cpython::*;
use super::super::game::*;

#[cfg(test)]
mod tests;

pub struct Root {
    pub turn: Player,
    pub num_remain: i32,
    pub board: [[Player; 19]; 19],
    pub possible: Vec<(usize, usize)>,
}

pub struct SimulateBack {
    turn: Player,
    num_remain: i32,
    pos: Option<(usize, usize)>,
    board: [[Player; 19]; 19],
    possible: Vec<(usize, usize)>,
}

pub struct Simulate<'a> {
    pub turn: Player,
    pub num_remain: i32,
    pub pos: Option<(usize, usize)>,
    pub board: &'a mut [[Player; 19]; 19],
    pub possible: &'a mut Vec<(usize, usize)>,
}

impl Root {
    fn possible() -> Vec<(usize, usize)> {
        (0..19).flat_map(|x| (0..19).map(move |y| (x, y)))
               .collect()
    }

    pub fn new() -> Root {
        Root {
            turn: Player::Black,
            num_remain: 1,
            board: [[Player::None; 19]; 19],
            possible: Self::possible()
        }
    }

    pub fn from_game(game: &Game) -> Root {
        let board = game.get_board();
        let possible = Self::possible().into_iter().filter(
            |(r, c)| board[*r][*c] == Player::None
        ).collect();

        Root {
            turn: game.get_turn(),
            num_remain: game.get_remain(),
            board: *game.get_board(),
            possible,
        }
    }

    pub fn to_simulate(&mut self) -> Simulate {
        Simulate {
            turn: self.turn,
            num_remain: self.num_remain,
            pos: None,
            board: &mut self.board,
            possible: &mut self.possible,
        }
    }
}

impl<'a> Simulate<'a> {
    pub fn validate(&self, row: usize, col: usize) -> bool {
        if row >= 19 || col >= 19 {
            return false;
        }
        if self.board[row][col] != Player::None {
            return false;
        }
        true
    }

    pub fn is_game_end(&self) -> Player {
        search(self.board)
    }

    pub fn simulate(&mut self, row: usize, col: usize) -> Simulate {
        let pos = (row, col);
        let item = self.possible.iter()
            .position(|x| *x == pos);
        self.possible.remove(item.unwrap());

        self.board[row][col] = self.turn;

        let (turn, num_remain) = {
            if self.num_remain <= 1 {
                (self.turn.switch(), 2)
            } else {
                (self.turn, 1)
            }
        };

        Simulate {
            turn,
            num_remain,
            pos: Some(pos),
            board: self.board,
            possible: self.possible,
        }
    }

    pub fn simulate_mut(&mut self, row: usize, col: usize) {
        let pos = (row, col);
        let item = self.possible.iter()
            .position(|x| *x == pos);
        self.possible.remove(item.unwrap());

        self.board[row][col] = self.turn;

        self.num_remain -= 1;
        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }
    }

    pub fn backup(&mut self) -> SimulateBack {
        SimulateBack {
            turn: self.turn,
            num_remain: self.num_remain,
            pos: self.pos,
            board: *self.board,
            possible: self.possible.clone(),
        }
    }

    pub fn recover(&mut self, backup: SimulateBack) {
        self.turn = backup.turn;
        self.num_remain = backup.num_remain;
        self.pos = backup.pos;
        *self.board = backup.board;
        *self.possible = backup.possible;
    }
}

impl<'a> Drop for Simulate<'a> {
    fn drop(&mut self) {
        if let Some((row, col)) = self.pos {
            self.possible.push((row, col));
            self.board[row][col] = Player::None;
        }
    }
}

impl<'a> ToPyObject for Simulate<'a> {
    type ObjectType = PyTuple;
    fn to_py_object(&self, py: Python) -> Self::ObjectType {
        let player = (self.turn as i32).to_py_object(py).into_object();
        let num_remain = self.num_remain.to_py_object(py).into_object();

        let mut board: Vec<PyObject> = Vec::new();
        for i in 0..19 {
            for j in 0..19 {
                board[i * 19 + j] = (self.board[i][j] as i32).to_py_object(py).into_object();
            }
        }
        let list = PyList::new(py, board.as_slice()).into_object();
        let tuple = [player, num_remain, list];

        PyTuple::new(py, &tuple)
    }
}