//! Black-White seperable policy
//!
//! Because `Agent` get single policy to play game, policy structure for black-white seperation is required.
//! It get two different policies and playing game with given seperately, black and white.
//!
//! # Examples
//! ```rust
//! io_policy_stdio!(io_policy);
//! let mut rand_policy = RandomPolicy::new();
//!
//! let mut multi_policy = policy::MultiPolicy::new(&mut rand_policy, &mut io_policy);
//! Agent::debug(&mut multi_policy).play().unwrap();
//! ```
use policy::Policy;
use game::{Game, Player};

#[cfg(test)]
mod tests;

/// Black-White seperable policy
///
/// Because `Agent` get single policy to play game, policy structure for black-white seperation is required.
/// It get two different policies and playing game with given seperately, black and white.
///
/// # Examples
/// ```rust
/// io_policy_stdio!(io_policy);
/// let mut rand_policy = RandomPolicy::new();
///
/// let mut multi_policy = policy::MultiPolicy::new(&mut rand_policy, &mut io_policy);
/// Agent::debug(&mut multi_policy).play().unwrap();
/// ```
pub struct MultiPolicy<'a, 'b> {
    black_policy: &'a mut Policy,
    white_policy: &'b mut Policy,
}

impl<'a, 'b> MultiPolicy<'a, 'b> {
    /// Construct a new `MultiPolicy`
    ///
    /// # Examples
    /// ```rust
    /// let mut rand_policy = RandomPolicy::new();
    /// let mut default_policy = DefaultPolicy::new();
    /// let mut multi_policy = MultiPolicy::new(&mut rand_policy, &mut default_policy);
    /// ```
    pub fn new(black_policy: &'a mut Policy, white_policy: &'b mut Policy) -> MultiPolicy<'a, 'b> {
        MultiPolicy { black_policy, white_policy }
    }
}

impl<'a, 'b> Policy for MultiPolicy<'a, 'b> {
    /// Condition on `game.turn` to pass policy seperately
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        match game.get_turn() {
            Player::None => panic!("seperate_policy::init couldn't get next policy for player none"),
            Player::Black => self.black_policy.next(game),
            Player::White => self.white_policy.next(game),
        }
    }
}