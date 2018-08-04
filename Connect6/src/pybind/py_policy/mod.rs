extern crate cpython;

use cpython::*;
use std::collections::HashMap;

use super::super::{game::*, mcts::*};

struct Node {
    visit: i32,
    value: f32, //probability of black win
    q_value: f32,
    policy: [[f32; 19]; 19],
    prev: Vec<u64>,
}

impl Node {
    fn new() -> Node {
        Node {
            visit: 0,
            value: 0.,
            q_value: 0.,
            policy: [[0.; 19]; 19],
            prev: Vec::new(),
        }
    }

    fn with_output(value: f32, policy: &[[f32; 19]; 19]) -> Node {
        Node {
            visit: 1,
            value,
            q_value: 0.,
            policy: *policy,
            prev: Vec::new(),
        }
    }
}

pub struct AlphaZero<'a> {
    py: Python<'a>,
    obj: PyObject,
    map: HashMap<[[Player; 19]; 19], Node>,
}

impl<'a> AlphaZero<'a> {
    pub fn new(py: Python<'a>, obj: PyObject) -> AlphaZero {
        AlphaZero {
            py,
            obj,
            map: HashMap::new(),
        }
    }

    fn get_from(&self, sim: &Simulate) -> Option<(f32, [[f32; 19]; 19])> {
        let pytuple = sim.to_py_object(self.py);
        let res = self.obj.call(self.py, (pytuple, ), None).ok()?;
        let pytuple = res.cast_into::<PyTuple>(self.py).ok()?;

        let value = pytuple.get_item(self.py, 0).extract::<f32>(self.py).ok()?;
        let policy = pytuple.get_item(self.py, 1);

        let pyseq = policy.cast_into::<PySequence>(self.py).ok()?;
        let pyiter = pyseq.iter(self.py).ok()?;

        let vec = pyiter
            .flat_map(|x| x.ok())
            .flat_map(|x| x.extract::<f32>(self.py).ok())
            .collect::<Vec<_>>();

        if vec.len() != 361 {
            return None;
        }

        let mut policy = [[0.; 19]; 19];
        for i in 0..19 {
            for j in 0..19 {
                let mask = (sim.board[i][j] as i32 as f32).abs();
                policy[i][j] = vec[i * 19 + j] * mask;
            }
        }

        Some((value, policy))
    }
}

impl<'a> Policy for AlphaZero<'a> {
    fn num_iter(&self) -> i32 {
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