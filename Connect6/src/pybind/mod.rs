//! Rust bindings for python interface and impl of AlphaZero policy
//!
//! It provides `rust-cpython` based bindings of some utilities to implement policy `AlphaZero`.
//! `AlphaZero` policy is implemented based on [Mastering the game of Go with deep neural networks and tree search](https://www.nature.com/articles/nature16961)
//! and [Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm](https://arxiv.org/abs/1712.01815).
//! It pass callable python object with method `__call__(self, turn, board): (value, prob)`
//! and make decision with combined mcts and value, probability approximator.
//!
//! # Examples
//! ```rust
//!
//! ```
pub use self::py_policy::*;
pub use self::pybind_impl::*;

#[macro_use]
mod pybind_impl;
mod py_policy;
