#![doc(html_root_url = "https://revsic.github.io/AlphaZero-Connect6/")]
//! [Rust](https://www.rust-lang.org) implementation of connect6 and policies
//! for learning [AlphaZero](https://arxiv.org/abs/1712.01815) with [Python](https://www.python.org/) interface.
//!
//! Connect6 provides game environment and policy based self-playing agent with some pre-defined policies.
//! It also provides multi-threading async agent and MCTS policy [AlphaZero](https://arxiv.org/abs/1712.01815)
//! with some hyperparameters control.
//!
extern crate futures;
extern crate rand;
extern crate tokio;

#[macro_use]
mod macro_def;

pub mod agent;
pub mod game;
pub mod policy;

pub const BOARD_SIZE: usize = 15;
pub const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;

pub type Board = [[game::Player; BOARD_SIZE]; BOARD_SIZE];
