//! Predefined policy for agent to play game.
//!
//! Policy represents algorithm that make choice in given situation.
//! This module provides 5 predefined policies, AlphaZero policy, random policy, io policy, multi-policy and pure MCTS policy.
//!
//! - Policy : trait for playing game with `Agent`.
//! - AlphaZero : implementation of policy [AlphaZero](https://arxiv.org/abs/1712.01815).
//! - RandomPolicy : select possible position with randomness.
//! - IoPolicy : read user input.
//! - MultiPolicy : Black-White seperable policy, pass two different policies as initialize parameter.
//! - DefaultPolicy : Pure Monte Carlo tree search implementation.
//!
//! # Examples
//! ```rust
//! # extern crate connect6;
//! # use connect6::{agent::Agent, game::Game, policy::Policy};
//! struct ZeroPolicy {}
//! impl Policy for ZeroPolicy {
//!     fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
//!         Some((0, 0))
//!     }
//! }
//!
//! let mut policy = ZeroPolicy {};
//! let result = Agent::new(&mut policy).play();
//! assert!(result.is_err());
//! ```
pub use self::alphazero_policy::*;
pub use self::default_policy::*;
pub use self::io_policy::*;
pub use self::multi_policy::*;
pub use self::random_policy::*;
pub use self::simulate::*;

mod alphazero_policy;
mod default_policy;
mod io_policy;
mod multi_policy;
mod random_policy;
mod simulate;

use game::Game;

/// trait for playing game with Agent.
pub trait Policy {
    /// generate next selection
    fn next(&mut self, game: &Game) -> Option<(usize, usize)>;
}
