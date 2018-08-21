//! Predefined policy for agent to play game.
//!
//! Policy represents alogirthm that makes choice in given situation.
//! This module provides 4 predefined policies, random policy, io policy, multi-policy and default MCTS policy.
//!
//! - Policy : trait for playing game with `Agent`.
//! - RandomPolicy : select possible position with randomness.
//! - IoPolicy : read user input.
//! - MultiPolicy : Black-White seperable policy, pass two different policies as initialize parameter.
//! - DefaultPolicy : Pure Monte Carlo tree search implementation.
//!
//! # Examples
//! ```rust
//! struct zero_policy {}
//! impl Policy for zero_policy {
//!     fn next(&mut self, game: &Game) -> Option<(usize, usize)> { Some((0, 0)) }
//! }
//! let mut policy = zero_policy {};
//! let result = Agent::new(&mut policy).play();
//! assert_eq!(result, Err(String::from("game::play already set position")));
//! ```
pub use self::simulate::*;
pub use self::default_policy::*;
pub use self::io_policy::*;
pub use self::multi_policy::*;
pub use self::random_policy::*;

mod default_policy;
mod io_policy;
mod multi_policy;
mod random_policy;
mod simulate;

use super::game::Game;

/// trait for playing game with `Agent`.
pub trait Policy {
    /// get next selection
    fn next(&mut self, game: &Game) -> Option<(usize, usize)>;
}