extern crate cpython;

use cpython::{PyResult, Python};

use super::super::{agent::*, game::*, mcts::*};
use std::any::Any;

struct AlphaZero {

}

impl AlphaZero {

}

impl Policy for AlphaZero {
    fn as_any(&self) -> &Any {
        self
    }

    fn num_expand(&self) -> i32 {
        0
    }

    fn select(&self, sim: &Simulate) -> (usize, usize) {
        (0, 0)
    }

    fn update(&mut self, sim: &mut Simulate) {

    }

    fn get_policy(&self, root: &Root) -> (usize, usize) {
        (0, 0)
    }
}