extern crate cpython;

use super::super::game::*;
use super::super::policy::*;
use super::super::pybind::*;
use super::super::Board;

use cpython::*;
use std::cell::RefCell;
use std::io;
use std::time::Instant;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Path {
    pub turn: Player,
    pub board: Board,
    pub pos: (usize, usize),
}

pub struct RunResult {
    pub winner: Player,
    pub path: Vec<Path>,
}

pub struct Agent<'a> {
    game: RefCell<Game>,
    debug: bool,
    policy: &'a mut Policy
}

impl<'a> Agent<'a> {
    pub fn new(policy: &'a mut Policy) -> Agent<'a> {
        Agent {
            game: RefCell::new(Game::new()),
            debug: false,
            policy,
        }
    }

    pub fn debug(policy: &'a mut Policy) -> Agent<'a> {
        Agent {
            game: RefCell::new(Game::new()),
            debug: true,
            policy,
        }
    }

    pub fn play(&mut self) -> Result<RunResult, String> {
        let mut winner = Player::None;
        let mut path = Vec::new();
        let mut game = self.game.borrow_mut();

        loop {
            if self.debug {
                game.print(&mut io::stdout()).unwrap();
            }
            let before = Instant::now();
            let pos = self.policy.next(&game);
            let duration = before.elapsed();

            if pos.is_none() {
                break;
            }
            let pos = pos.unwrap();
            path.push(Path { turn: game.get_turn(), board: *game.get_board(), pos, });

            match game.play(pos) {
                Ok(result) => if self.debug {
                    let (row, col) = result.position;
                    let row = (row as u8 + 0x61) as char;
                    let col = (col as u8 + 0x41) as char;
                    println!("{:?} ({}, {}), remain {}, {} elapsed",
                             result.player, row, col, result.num_remain, duration.as_secs());
                },
                Err(err) => return Err(format!("agent::play : {}", err)),
            };

            let is_end = game.is_game_end();
            if is_end != Player::None {
                winner = is_end;
                break;
            }
        }

        Ok(RunResult { winner, path })
    }
}

impl ToPyObject for Path {
    type ObjectType = PyTuple;
    fn to_py_object(&self, py: Python) -> PyTuple {
        let turn = (self.turn as i32).to_py_object(py).into_object();
        let board = pylist_from_board(py, &self.board);
        let (row, col) = self.pos;

        let row = (row as i32).to_py_object(py).into_object();
        let col = (col as i32).to_py_object(py).into_object();
        let pos_tuple = PyTuple::new(py, &[row, col]).into_object();

        let tuple = PyTuple::new(py, &[turn, board, pos_tuple]);
        tuple
    }
}

impl ToPyObject for RunResult {
    type ObjectType = PyTuple;
    fn to_py_object(&self, py: Python) -> PyTuple {
        let win = (self.winner as i32).to_py_object(py).into_object();
        let path = self.path.iter()
            .map(|x| x.to_py_object(py).into_object())
            .collect::<Vec<_>>();
        let list = PyList::new(py, path.as_slice()).into_object();
        let tuple = PyTuple::new(py, &[win, list]);
        tuple
    }
}