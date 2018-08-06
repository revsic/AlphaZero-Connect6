extern crate cpython;

use cpython::*;
use std::io;

use super::*;
use super::super::game::*;
use super::super::agent::*;
use super::super::pybind::*;

#[cfg(test)]
mod tests;

pub struct Path {
    pub turn: Player,
    pub board: [[Player; 19]; 19],
    pub pos: (usize, usize),
}

pub struct RunResult {
    pub winner: Player,
    pub path: Vec<Path>,
}

pub struct SinglePolicyMCTS<'a, P> where P: 'a + Policy + Sized {
    policy: &'a mut P,
    debug: bool,
}

pub struct SeperatePolicyMCTS<'a, 'b, P, Q>
    where P: 'a + Policy + Sized,
          Q: 'b + Policy + Sized {
    black_policy: &'a mut P,
    white_policy: &'b mut Q,
    debug: bool,
}

impl<'a, P> SinglePolicyMCTS<'a, P> where P: 'a + Policy + Sized {
    pub fn new(policy: &'a mut P,) -> SinglePolicyMCTS<'a, P> {
        SinglePolicyMCTS {
            policy,
            debug: false,
        }
    }

    pub fn debug(policy: &'a mut P) -> SinglePolicyMCTS<'a, P> {
        SinglePolicyMCTS {
            policy,
            debug: true,
        }
    }

    pub fn run(&mut self) -> RunResult {
        let mut winner = Player::None;
        let agent = Agent::with_start();
        let game = agent.get_game();

        let mut path = Vec::new();
        loop {
            let (turn, board, (row, col)) = {
                let game = game.read().unwrap();
                let turn = game.get_turn();
                if self.debug { game.print(&mut io::stdout()).unwrap(); }
                (turn, *game.get_board(), self.policy.get_policy(&*game))
            };

            path.push(Path{ turn, board, pos: (row, col) });
            let row = (row as u8 + 0x61) as char;
            let col = (col as u8 + 0x41) as char;
            let query: String = vec![row, col].iter().collect();

            match agent.play(&query) {
                Ok(GameResult::GameEnd(player)) => {
                    winner = player;
                    break;
                },
                Ok(GameResult::Status(_)) => (),
                Err(err) => panic!(format!("single_policy_mcts::run : {}", err)),
            };
        }

        RunResult { winner, path }
    }
}

impl<'a, 'b, P, Q> SeperatePolicyMCTS<'a, 'b, P, Q>
    where P: 'a + Policy + Sized,
          Q: 'b + Policy + Sized
{
    pub fn new(black_policy: &'a mut P, white_policy: &'b mut Q) -> SeperatePolicyMCTS<'a, 'b, P, Q> {
        SeperatePolicyMCTS {
            black_policy,
            white_policy,
            debug: false,
        }
    }

    pub fn debug(black_policy: &'a mut P, white_policy: &'b mut Q) -> SeperatePolicyMCTS<'a, 'b, P, Q> {
        SeperatePolicyMCTS {
            black_policy,
            white_policy,
            debug: true,
        }
    }

    pub fn run(&mut self) -> RunResult {
        let mut winner = Player::None;
        let agent = Agent::with_start();
        let game = agent.get_game();

        let mut path = Vec::new();
        loop {
            let (turn, board, (row, col)) = {
                let game = game.read().unwrap();
                let turn = game.get_turn();
                let pos = match turn {
                    Player::None =>
                        panic!("seperate_policy_mcts::run : couldn't play with none player"),
                    Player::Black => self.black_policy.get_policy(&*game),
                    Player::White => self.white_policy.get_policy(&*game),
                };
                if self.debug { game.print(&mut io::stdout()).unwrap(); }
                (turn, *game.get_board(), pos)
            };

            path.push(Path{ turn, board, pos: (row, col) });
            let row = (row as u8 + 0x61) as char;
            let col = (col as u8 + 0x41) as char;
            let query: String = vec![row, col].iter().collect();

            match agent.play(&query) {
                Ok(GameResult::GameEnd(player)) => {
                    winner = player;
                    break;
                },
                Ok(GameResult::Status(_)) => (),
                Err(err) => panic!(format!("seperate_policy_mcts::run : {}", err)),
            };
        }

        RunResult { winner, path }
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