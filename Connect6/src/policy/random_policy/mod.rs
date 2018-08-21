//! Play game with random selection.
//!
//! # Examples
//! ```rust
//! let mut policy = RandomPolicy();
//! let result = Agent::new(&mut policy).play();
//! assert!(result.is_ok());
//! ```
extern crate rand;

#[cfg(test)]
mod tests;

use self::rand::seq::*;
use super::*;

/// Play game with random selection.
///
/// # Examples
/// ```rust
/// let mut policy = RandomPolicy();
/// let result = Agent::new(&mut policy).play();
/// assert!(result.is_ok());
/// ```
pub struct RandomPolicy { }

impl RandomPolicy {
    /// Construct a new RandomPolicy
    pub fn new() -> RandomPolicy {
        RandomPolicy { }
    }
}

impl Policy for RandomPolicy {
    /// make random selection
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        let sim = Simulate::from_game(game);
        let node = sim.node.borrow();
        // choose position from vector `possible`
        node.possible.choose(&mut rand::thread_rng())
            .map(|x| *x)
    }
}
