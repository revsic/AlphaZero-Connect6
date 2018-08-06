extern crate cpython;

use cpython::*;
use std::cell::RefCell;
use std::rc::Rc;
use super::super::game::*;

#[cfg(test)]
mod tests;

pub struct Node {
    pub board: [[Player; 19]; 19],
    pub possible: Vec<(usize, usize)>,
}

impl Node {
    #[inline]
    fn possible() -> Vec<(usize, usize)> {
        (0..19).flat_map(|x| (0..19).map(move |y| (x, y))).collect()
    }

    fn new() -> Node {
        Node {
            board: [[Player::None; 19]; 19],
            possible: Self::possible(),
        }
    }

    fn from_board(board: &[[Player; 19]; 19]) -> Node {
        let possible = Self::possible()
            .into_iter()
            .filter(|(r, c)| board[*r][*c] == Player::None)
            .collect();

        Node {
            board: *board,
            possible,
        }
    }
}

pub struct Simulate {
    pub turn: Player,
    pub num_remain: i32,
    pub pos: Option<(usize, usize)>,
    pub node: Rc<RefCell<Node>>,
}

impl Simulate {
    pub fn new() -> Simulate {
        Simulate {
            turn: Player::Black,
            num_remain: 1,
            pos: None,
            node: Rc::new(RefCell::new(Node::new())),
        }
    }

    pub fn from_game(game: &Game) -> Simulate {
        Simulate {
            turn: game.get_turn(),
            num_remain: game.get_remain(),
            pos: None,
            node: Rc::new(RefCell::new(Node::from_board(game.get_board()))),
        }
    }

    pub fn deep_clone(&self) -> Simulate {
        let node = self.node.borrow();
        let board = node.board;

        Simulate {
            turn: self.turn,
            num_remain: self.num_remain,
            pos: None,
            node: Rc::new(RefCell::new(Node::from_board(&board))),
        }
    }

    pub fn board(&self) -> [[Player; 19]; 19] {
        let node = self.node.borrow();
        node.board
    }

    pub fn search_winner(&self) -> Player {
        let board = &self.node.borrow().board;
        search(board)
    }

    pub fn validate(&self, row: usize, col: usize) -> bool {
        if row >= 19 || col >= 19 {
            return false;
        }
        let board = &self.node.borrow().board;
        if board[row][col] != Player::None {
            return false;
        }
        true
    }

    pub fn simulate(&self, row: usize, col: usize) -> Simulate {
        let mut node = self.node.borrow_mut();
        let item = node.possible.iter()
            .position(|x| *x == (row, col));
        node.possible.remove(item.unwrap());

        node.board[row][col] = self.turn;
        let (turn, num_remain) =
            if self.num_remain <= 1 {
                (self.turn.switch(), 2)
            } else {
                (self.turn, 1)
            };

        Simulate {
            turn,
            num_remain,
            pos: Some((row, col)),
            node: self.node.clone(),
        }
    }

    pub fn simulate_in(&mut self, row: usize, col: usize) {
        let mut node = self.node.borrow_mut();
        let item = node.possible.iter()
            .position(|x| *x == (row, col));
        node.possible.remove(item.unwrap());

        node.board[row][col] = self.turn;
        self.num_remain -= 1;

        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }
    }

    pub fn rollback_in(&mut self, row: usize, col: usize) {
        let mut node = self.node.borrow_mut();

        node.board[row][col] = Player::None;
        node.possible.push((row, col));

        self.num_remain += 1;
        if self.num_remain > 2 {
            self.num_remain = 1;
            self.turn.mut_switch();
        }
    }
}

impl Drop for Simulate {
    fn drop(&mut self) {
        if let Some((row, col)) = self.pos {
            let mut node = self.node.borrow_mut();
            node.possible.push((row, col));
            node.board[row][col] = Player::None;
        }
    }
}
