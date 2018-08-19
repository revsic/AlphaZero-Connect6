extern crate cpython;
extern crate rand;

use cpython::*;
use self::rand::distributions::{Distribution, Dirichlet};
use self::rand::prelude::*;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::pybind_impl::*;
use super::super::game::*;
use super::super::policy::{Policy, Simulate, diff_board};
use super::super::{BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
struct Node {
    turn: Player,
    visit: i32,
    value: f32,
    q_sum: f32,
    q_value: f32,
    n_prob: f32,
    prob: [[f32; BOARD_SIZE]; BOARD_SIZE],
    num_player: usize,
    board: Board,
    next_node: Vec<u64>,
}

impl Node {
    fn new(board: &Board) -> Node {
        let num_player = board.iter()
            .map(|x| x.iter().filter(|x| **x != Player::None).count())
            .sum();
        Node {
            turn: Player::None,
            visit: 0,
            value: 0.,
            q_sum: 0.,
            q_value: 0.,
            n_prob: 0.,
            prob: [[0.; BOARD_SIZE]; BOARD_SIZE],
            num_player,
            board: *board,
            next_node: Vec::new(),
        }
    }

    fn new_with_num(board: &Board, num_player: usize) -> Node {
        Node {
            turn: Player::None,
            visit: 0,
            value: 0.,
            q_sum: 0.,
            q_value: 0.,
            n_prob: 0.,
            prob: [[0.; BOARD_SIZE]; BOARD_SIZE],
            num_player,
            board: *board,
            next_node: Vec::new(),
        }
    }

    fn recalc_q(&mut self) {
        self.q_value = self.q_sum / self.visit as f32;
    }
}

#[derive(Copy, Clone)]
pub struct HyperParameter {
    pub num_simulation: i32,
    pub num_expansion: usize,
    pub epsilon: f32,
    pub dirichlet_alpha: f64,
    pub c_puct: f32,
}

impl HyperParameter {
    pub fn default() -> HyperParameter {
        HyperParameter {
            num_simulation: 800,
            num_expansion: 1,
            epsilon: 0.25,
            dirichlet_alpha: 0.03,
            c_puct: 1.,
        }
    }
}

pub struct AlphaZero {
    obj: PyObject,
    map: HashMap<u64, Node>,
    param: HyperParameter,
}

impl AlphaZero {
    pub fn new(obj: PyObject) -> AlphaZero {
        let param = HyperParameter::default();
        AlphaZero {
            obj,
            map: HashMap::new(),
            param,
        }
    }

    pub fn with_param(obj: PyObject, param: HyperParameter) -> AlphaZero {
        AlphaZero {
            obj,
            map: HashMap::new(),
            param,
        }
    }

    fn get_from(&self, next_turn: Player, boards: &Vec<Board>)
                -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        let (value_vec, policy_vec) = {
            let gil = Python::acquire_gil();
            let py = gil.python();

            let py_turn = (next_turn as i32).to_py_object(py);
            let py_board = pylist_from_multiple(py, boards);
            let res = pycheck!(self.obj.call(py, (py_turn, py_board), None), "alpha_zero::get_from couldn't call pyobject");
            let pytuple = pycheck!(res.cast_into::<PyTuple>(py), "alpha_zero::get_from couldn't cast into pytuple");

            let value = pytuple.get_item(py, 0);
            let policy = pytuple.get_item(py, 1);

            let value_vec = pyseq_to_vec(py, value)?;
            let policy_vec = policy.cast_into::<PySequence>(py).ok()?
                .iter(py).ok()?
                .filter_map(|x| x.ok())
                .filter_map(|x| pyseq_to_vec(py, x))
                .collect::<Vec<Vec<f32>>>();

            (value_vec, policy_vec)
        };

        let mut vec = Vec::new();
        for (board, policy) in boards.iter().zip(policy_vec.iter()) {
            let mut temporal = [[0.; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    let mask = (board[i][j] == Player::None) as i32 as f32;
                    temporal[i][j] = policy[i * BOARD_SIZE + j] * mask;
                }
            }
            vec.push(temporal)
        }
        Some((value_vec, vec))
    }

    fn maximum_from(&self, sim: &Simulate) -> Option<u64> {
        let next_turn = sim.next_turn();
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_nodes = tree_node.next_node.iter()
            .map(|x| self.map.get(x).unwrap())
            .filter(|x| x.turn == next_turn)
            .collect::<Vec<_>>();
        if child_nodes.is_empty() {
            return None;
        } else if child_nodes.len() == 1 {
            return Some(hash(&child_nodes[0].board));
        }

        let epsilon = self.param.epsilon;
        let alpha = self.param.dirichlet_alpha;
        let dirichlet = Dirichlet::new_with_param(alpha, child_nodes.len());

        let c_puct = self.param.c_puct;
        let visit_sum = child_nodes.iter().map(|x| x.visit).sum::<i32>() as f32;
        let puct = |node: &Node, noise: &f64| {
            let noise = *noise as f32;
            let visit = node.visit as f32;
            let prob = epsilon * noise + (1. - epsilon) * node.n_prob;
            prob * (visit_sum - visit).sqrt() / (1. + visit)
        };
        let prob = |(node, noise): &(&Node, f64)| node.q_value + c_puct * puct(node, noise);
        child_nodes.into_iter()
            .zip(dirichlet.sample(&mut thread_rng()))
            .max_by(|n1, n2| prob(n1).partial_cmp(&prob(n2)).unwrap())
            .map(|x| hash(&x.0.board))
    }

    fn init(&mut self, sim: &Simulate) {
        let node = sim.node.borrow();
        let hashed = hash(&node.board);

        if self.map.get(&hashed).is_none() {
            if let Some((value, policy)) = self.get_from(sim.turn, &vec![node.board]) {
                let entry = self.map.entry(hashed).or_insert(Node::new(&node.board));
                entry.turn = sim.turn;
                entry.value = value[0];
                entry.prob = policy[0];
            } else {
                panic!("alpha_zero::init couldn't get from py policy");
            }
        }
    }

    fn select(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let hashed = self.maximum_from(sim);
        if let Some(hashed) = hashed {
            let node = self.map.get(&hashed).unwrap();
            diff_board(&node.board, &tree_node.board)
        } else {
            None
        }
    }

    fn expand(&mut self, sim: &Simulate) {
        let (possible, parent_hashed) = {
            let node = sim.node.borrow();
            let hashed = hash(&node.board);
            let mut possible = node.possible.clone();

            possible.shuffle(&mut thread_rng());
            (possible, hashed)
        };
        let (child_num, prob) = {
            let parent_node = self.map.get(&parent_hashed).unwrap();
            (parent_node.num_player + 1, parent_node.prob)
        };
        let mut idx = 0;
        let next_turn = sim.next_turn();
        let mut hashes = Vec::new();
        let mut boards = Vec::new();
        let n_expansion = min(possible.len(), self.param.num_expansion);
        for (row, col) in possible.into_iter() {
            let board = sim.simulate(row, col).board();
            let hashed = hash(&board);
            hashes.push(hashed);

            let entry = self.map.entry(hashed).or_insert(Node::new_with_num(&board, child_num));
            if entry.turn != next_turn {
                entry.turn = next_turn;
                if idx < n_expansion {
                    boards.push(board);
                }
                idx += 1;
            }
            entry.n_prob = prob[row][col];
        }
        { // borrow mut self.map: HashMap
            let parent_node = self.map.get_mut(&parent_hashed).unwrap();
            for hashed in hashes.iter() {
                parent_node.next_node.push(*hashed);
            }
        }

        if boards.len() > 0 {
            if let Some((values, policies)) = self.get_from(next_turn, &boards) {
                for ((value, policy), hashed) in values.iter()
                    .zip(policies.iter())
                    .zip(hashes.iter())
                    {
                        let node = self.map.get_mut(hashed).unwrap();
                        node.visit += 1;
                        node.value = *value;
                        node.prob = *policy;
                    }
            } else {
                panic!("alpha_zero::expand couldn't get from python object")
            }
        }
    }

    fn update(&mut self, sim: &Simulate, path: &Vec<(usize, usize)>) {
        let hashed = hash(&sim.board());
        let q_sum = {
            let tree_node = self.map.get(&hashed).unwrap();
            tree_node.next_node.iter()
                .map(|x| self.map.get(x).unwrap().value)
                .sum::<f32>()
        };
        {
            let tree_node = self.map.get_mut(&hashed).unwrap();
            tree_node.visit += 1;
            tree_node.q_sum =
                if sim.next_turn() == sim.turn { q_sum } else { -q_sum };
            tree_node.recalc_q();
        }
        let mut sim = sim.deep_clone();
        for (row, col) in path.iter().rev() {
            sim.rollback_in(*row, *col);
            let node = self.map.get_mut(&hash(&sim.board())).unwrap();
            node.visit += 1;
            node.recalc_q();
        }
    }

    fn search(&mut self, simulate: &Simulate) {
        self.init(&simulate);
        let mut simulate = simulate.deep_clone();
        let mut path = Vec::new();
        while let Some((row, col)) = self.select(&simulate) {
            path.push((row, col));
            simulate.simulate_in(row, col);
        }

        if simulate.search_winner() != Player::None {
            return;
        }
        self.expand(&simulate);
        self.update(&simulate, &path);
    }

    fn policy(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let next_turn = sim.next_turn();
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_node = tree_node.next_node.iter()
            .map(|x| self.map.get(x).unwrap())
            .filter(|x| x.turn == next_turn)
            .collect::<Vec<_>>();

        let visit_sum = child_node.iter().map(|x| x.visit).sum::<i32>() as f32;
        let prob = |node: &Node| -> f32 {
            let visit = node.visit as f32;
            visit / (visit_sum - visit + 1.)
        };
        child_node.into_iter()
            .max_by(|n1, n2| prob(n1).partial_cmp(&prob(n2)).unwrap())
            .map(|max_node| diff_board(&node.board, &max_node.board).unwrap())
    }
}

fn hash(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

impl Policy for AlphaZero {
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        let simulate = Simulate::from_game(game);
        self.init(&simulate);

        for _ in 0..self.param.num_simulation {
            self.search(&simulate);
        }
        let res = self.policy(&simulate);

        let node = self.map.get(&hash(&simulate.board())).unwrap().clone();
        let num_player = node.num_player;
        let sibling = self.map.iter()
            .filter(|(_, node)| node.num_player == num_player)
            .map(|(hash, _)| *hash)
            .collect::<Vec<u64>>();

        for hashed in sibling {
            self.map.remove(&hashed);
        }
        self.map.insert(hash(&node.board), node);
        res
    }
}