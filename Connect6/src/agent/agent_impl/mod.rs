//! Implementation of agent.
//!
//! Agent is structure for playing game with given single policy.
//! As we pass the single policy, agent play the game with given policy in the loop.
//! `agent.play` return the `PlayResult` and it can be converted as `PyObject`.
//!
//! # Examples
//! For black-white seperable policy, reference [MultiPolicy](../../policy/multi_policy).
//! ```rust
//! let mut stdin = std::io::stdin();
//! let mut stdout = std::io::stdout();
//! let mut io_policy = IoPolicy::new(&mut stdin, &mut stdout);
//! let mut rand_policy = RandomPolicy::new();
//!
//! let mut multi_policy = policy::MultiPolicy::new(&mut rand_policy, &mut io_policy);
//! let result = Agent::debug(&mut multi_policy).play();
//! ```
extern crate cpython;

use super::super::game::*;
use super::super::policy::*;
use super::super::pybind::*;
use super::super::Board;

use cpython::*;
use std::io;
use std::time::Instant;

#[cfg(test)]
mod tests;

/// Unit of playing history, turn, board and selected position.
#[derive(Debug, PartialEq)]
pub struct Path {
    pub turn: Player,
    pub board: Board,
    pub pos: (usize, usize),
}

/// Result of playing game, winner and path (history of game).
pub struct RunResult {
    pub winner: Player,
    pub path: Vec<Path>,
}

/// Implementation of agent.
///
/// Agent is structure for playing game with given single policy.
/// As we pass the single policy, agent play the game with given policy in the loop.
/// `agent.play` return the `PlayResult` and it can be converted as `PyObject`.
///
/// # Examples
/// For black-white seperable policy, reference [MultiPolicy](../../policy/multi_policy).
/// ```rust
/// let mut stdin = std::io::stdin();
/// let mut stdout = std::io::stdout();
/// let mut io_policy = IoPolicy::new(&mut stdin, &mut stdout);
/// let mut rand_policy = RandomPolicy::new();
///
/// let mut multi_policy = policy::MultiPolicy::new(&mut rand_policy, &mut io_policy);
/// let result = Agent::debug(&mut multi_policy).play();
/// ```
pub struct Agent<'a> {
    game: Game,
    debug: bool,
    policy: &'a mut Policy
}

impl<'a> Agent<'a> {
    /// Construct a new `Agent` with given policy.
    ///
    /// # Examples
    /// ```rust
    /// let mut policy = RandomPolicy::new();
    /// let mut agent = Agent::new(&mut policy);
    /// ```
    pub fn new(policy: &'a mut Policy) -> Agent<'a> {
        Agent {
            game: Game::new(),
            debug: false,
            policy,
        }
    }

    /// Construct a debug mode `Agent` with given policy, it will display the dbg info.
    ///
    /// # Examples
    /// ```rust
    /// let mut policy = RandomPolicy::new();
    /// let mut agent = Agent::debug(&mut policy);
    /// ```
    pub fn debug(policy: &'a mut Policy) -> Agent<'a> {
        Agent {
            game: Game::new(),
            debug: true,
            policy,
        }
    }

    /// Self-play the game with given policy.
    ///
    /// # Examples
    /// ```rust
    /// let mut policy = RandomPolicy::new();
    /// let mut agent = Agent::new(&mut policy);
    ///
    /// let result = agent.play().unwrap();
    /// println!("winner: {:?}", result.winner);
    /// ```
    ///
    /// # Errors
    /// if selected position raise Err at [Game::play](../game/struct.Game.html#method.play).
    pub fn play(&mut self) -> Result<RunResult, String> {
        let mut winner = Player::None;
        let mut path = Vec::new();
        let game = &mut self.game;

        loop {
            if self.debug {
                game.print(&mut io::stdout()).unwrap();
            }
            let before = Instant::now();
            let pos = self.policy.next(&game);
            let duration = before.elapsed();

            // if policy could't generate next selection
            if pos.is_none() {
                break;
            }
            let pos = pos.unwrap();
            path.push(Path { turn: game.get_turn(), board: *game.get_board(), pos, });

            match game.play(pos) {
                Ok(result) => if self.debug {
                    // log the selection info
                    let (row, col) = result.position;
                    let row = (row as u8 + 0x61) as char;
                    let col = (col as u8 + 0x41) as char;
                    println!("{:?} ({}, {}), remain {}, {}.{} elapsed",
                             result.player, row, col, result.num_remain, duration.as_secs(), duration.subsec_millis());
                },
                Err(err) => return Err(format!("agent::play : {}", err)),
            };

            // if game end, method return the winner, or None.
            let is_end = game.is_game_end();
            if is_end != Player::None {
                winner = is_end;
                break;
            }
        }

        if self.debug { game.print(&mut io::stdout()).unwrap(); }
        Ok(RunResult { winner, path })
    }
}

impl ToPyObject for Path {
    type ObjectType = PyTuple;

    /// Return `PyTuple`, (turn: int, board: list(int, board_size ** 2), pos: (int, int))
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

    /// Return `PyTuple`, (winner: int, path: list(Path as PyTuple))
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