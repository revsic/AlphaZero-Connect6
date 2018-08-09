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
use super::super::{game::*, mcts::*, BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
struct Node {
    visit: i32,
    value: f32, //probability of black win
    q_value: f32,
    q_sum: f32,
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
            visit: 0,
            value: 0.,
            q_value: 0.,
            q_sum: 0.,
            n_prob: 0.,
            prob: [[0.; BOARD_SIZE]; BOARD_SIZE],
            num_player,
            board: *board,
            next_node: Vec::new(),
        }
    }

    fn new_with_num(board: &Board, num_player: usize) -> Node {
        Node {
            visit: 0,
            value: 0.,
            q_value: 0.,
            q_sum: 0.,
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

pub struct HyperParameter {
    pub num_simulation: i32,
    pub num_expansion: usize,
    pub epsilon: f32,
    pub dirichlet_alpha: f64,
    pub c_puct: f32,
}

impl HyperParameter {
    fn default() -> HyperParameter {
        HyperParameter {
            num_simulation: 800,
            num_expansion: 1,
            epsilon: 0.25,
            dirichlet_alpha: 0.03,
            c_puct: 1.,
        }
    }
}

pub struct AlphaZero<'a> {
    py: Python<'a>,
    obj: PyObject,
    map: HashMap<u64, Node>,
    param: HyperParameter,
}

impl<'a> AlphaZero<'a> {
    pub fn new(py: Python<'a>, obj: PyObject) -> AlphaZero {
        let param = HyperParameter::default();
        AlphaZero {
            py,
            obj,
            map: HashMap::new(),
            param,
        }
    }

    pub fn with_param(py: Python<'a>, obj: PyObject, param: HyperParameter) -> AlphaZero {
        AlphaZero {
            py,
            obj,
            map: HashMap::new(),
            param,
        }
    }

    fn get_from(&self, boards: &Vec<Board>) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        let pylist = pylist_from_multiple(self.py, boards);
        let res = self.obj.call(self.py, (pylist, ), None).ok()?;
        let pytuple = res.cast_into::<PyTuple>(self.py).ok()?;

        let value = pytuple.get_item(self.py, 0);
        let policy = pytuple.get_item(self.py, 1);

        let value_vec = pyseq_to_vec(self.py, value)?;
        let policy_vec = policy.cast_into::<PySequence>(self.py).ok()?
            .iter(self.py).ok()?
            .filter_map(|x| x.ok())
            .filter_map(|x| pyseq_to_vec(self.py, x))
            .collect::<Vec<Vec<f32>>>();

        let alpha = self.param.dirichlet_alpha;
        let epsilon = self.param.epsilon;
        let mut masked = Vec::new();
        for (board, policy) in boards.iter().zip(policy_vec.iter()) {
            let mut count = 0;
            let mut vec = Vec::new();
            let mut temporal = [[0.; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    let mask = (board[i][j] == Player::None) as i32 as f32;
                    temporal[i][j] = policy[i * BOARD_SIZE + j] * mask;

                    if mask != 0. {
                        count += 1;
                        vec.push((i, j));
                    }
                }
            }
            if count > 1 {
                let mut dirichlet = Dirichlet::new_with_param(alpha, count);
                let sample = dirichlet.sample(&mut rand::thread_rng());
                for (i, j) in vec {
                    count -= 1;
                    temporal[i][j] = (1. - epsilon) * temporal[i][j] + epsilon * sample[count] as f32;
                }
            }
            masked.push(temporal);
        }
        Some((value_vec, masked))
    }

    fn maximum_from(&self, sim: &Simulate) -> Option<u64> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_nodes = tree_node.next_node.iter()
            .map(|x| self.map.get(x).unwrap()).collect::<Vec<_>>();

        let c_puct = self.param.c_puct;
        let visit_sum = child_nodes.iter().map(|x| x.visit).sum::<i32>() as f32;
        let prob = |unary: fn(f32) -> f32|
            move |node: &Node| unary(node.q_value)
                + c_puct * unary(node.n_prob) * (visit_sum - node.visit as f32).sqrt() / (1. + node.visit as f32);

        let prob = match sim.turn {
            Player::None => panic!("alpha_zero::maximum_from couldn't get prob from none"),
            Player::Black => prob(|x| 1. - x),
            Player::White => prob(|x| x),
        };
        child_nodes.into_iter()
            .max_by(|n1, n2| prob(n1).partial_cmp(&prob(n2)).unwrap())
            .map(|x| hash(&x.board))
    }

    fn select(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        if tree_node.next_node.is_empty() {
            return None;
        }

        let hashed = self.maximum_from(sim).unwrap();
        let node = self.map.get(&hashed).unwrap();
        return diff_board(&node.board, &tree_node.board);
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
        let mut hashes = Vec::new();
        let mut boards = Vec::new();
        let n_expansion = min(possible.len(), self.param.num_expansion);
        for (row, col) in possible.iter() {
            let board = sim.simulate(*row, *col).board();
            let hashed = hash(&board);
            { // borrow mut self.map: HashMap
                let parent_node = self.map.get_mut(&parent_hashed).unwrap();
                parent_node.next_node.push(hashed);
            }
            if self.map.get(&hashed).is_none() {
                self.map.insert(hashed, Node::new_with_num(&board, child_num));
                if idx < n_expansion {
                    boards.push(board);
                    hashes.push(hashed);
                }
                idx += 1;
            }
            let node = self.map.get_mut(&hashed).unwrap();
            node.n_prob = prob[*row][*col];
        }
        if let Some((values, policies)) = self.get_from(&boards) {
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
            panic!("alpha_zero::couldn't get from python object")
        }
    }

    fn update(&mut self, sim: &Simulate, path: &Vec<(usize, usize)>) {
        let node = sim.node.borrow();
        let hashed = hash(&node.board);
        let q_sum = {
            let tree_node = self.map.get(&hashed).unwrap();
            tree_node.next_node.iter()
                .map(|x| self.map.get(x).unwrap().value)
                .sum::<f32>()
        };
        { // borrow mut map: HashMap
            let tree_node = self.map.get_mut(&hashed).unwrap();
            tree_node.visit += 1;
            tree_node.q_sum = q_sum;
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
}

fn hash(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

impl<'a> Policy for AlphaZero<'a> {
    fn init(&mut self, sim: &Simulate) {
        let node = sim.node.borrow();
        let hashed = hash(&node.board);

        if self.map.get(&hashed).is_none() {
            if let Some((value, policy)) = self.get_from(&vec![node.board]) {
                let entry = self.map.entry(hashed).or_insert(Node::new(&node.board));

                entry.value = value[0];
                entry.prob = policy[0];
            } else {
                panic!("alpha_zero::init couldn't get from py policy");
            }
        }
    }

    fn policy(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_node = tree_node.next_node.iter()
            .map(|x| self.map.get(x).unwrap())
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

    fn get_policy(&mut self, game: &Game) -> Option<(usize, usize)> {
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