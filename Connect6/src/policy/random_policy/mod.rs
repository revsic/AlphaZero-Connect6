extern crate rand;

#[cfg(test)]
mod tests;

use self::rand::seq::*;
use super::*;

pub struct RandomPolicy { }

impl RandomPolicy {
    pub fn new() -> RandomPolicy {
        RandomPolicy { }
    }
}

impl Policy for RandomPolicy {
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        let sim = Simulate::from_game(game);
        let node = sim.node.borrow();
        node.possible.choose(&mut rand::thread_rng())
            .map(|x| *x)
    }
}
