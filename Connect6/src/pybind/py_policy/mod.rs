extern crate cpython;
extern crate rand;

use cpython::*;
use self::rand::distributions::{Distribution, Dirichlet};
use self::rand::prelude::*;
use self::rand::seq::*;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::pybind_impl::*;
use super::super::{game::*, mcts::*, BOARD_SIZE, BOARD_CAPACITY, Board};

#[cfg(test)]
mod tests;

#[derive(Clone)]
struct Node {
    visit: i32,
    value: f32, //probability of black win
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
            visit: 0,
            value: 0.,
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
            visit: 0,
            value: 0.,
            q_value: 0.,
            n_prob: 0.,
            prob: [[0.; BOARD_SIZE]; BOARD_SIZE],
            num_player,
            board: *board,
            next_node: Vec::new(),
        }
    }
}

pub struct HyperParameter {
    pub num_simulation: i32,
    pub num_expansion: usize,
    pub initial_tau: f32,
    pub updated_tau: f32,
    pub tau_update_term: usize,
    pub epsilon: f32,
    pub dirichlet_alpha: f64,
    pub c_puct: f32,
}

impl HyperParameter {
    fn default() -> HyperParameter {
        HyperParameter {
            num_simulation: 150,
            num_expansion: 1,
            initial_tau: 1.,
            updated_tau: 1e-4,
            tau_update_term: 30,
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
    tau: f32,
    param: HyperParameter,
}

impl<'a> AlphaZero<'a> {
    pub fn new(py: Python<'a>, obj: PyObject) -> AlphaZero {
        let param = HyperParameter::default();
        AlphaZero {
            py,
            obj,
            map: HashMap::new(),
            tau: param.initial_tau,
            param,
        }
    }

    pub fn with_param(py: Python<'a>, obj: PyObject, param: HyperParameter) -> AlphaZero {
        AlphaZero {
            py,
            obj,
            map: HashMap::new(),
            tau: param.initial_tau,
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
            let mut dirichlet = Dirichlet::new_with_param(alpha, count);
            let sample = dirichlet.sample(&mut rand::thread_rng());
            for (i, j) in vec {
                count -= 1;
                temporal[i][j] = (1. - epsilon) * temporal[i][j] + epsilon * sample[count] as f32;
            }
            masked.push(temporal);
        }
        Some((value_vec, masked))
    }

    fn maximum_from(&self, sim: &Simulate) -> Option<u64> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();

        let c_puct = self.param.c_puct;
        let parent_visit = tree_node.visit as f32;
        let prob = |unary: fn(f32) -> f32|
            move |node: &Node| unary(node.q_value / (node.visit as f32))
                + c_puct * unary(node.n_prob) * (parent_visit - node.visit as f32).sqrt() / (1. + node.visit as f32);
        let prob = match sim.turn {
            Player::None => panic!("alpha_zero::maximum_from couldn't get prob from none"),
            Player::Black => prob(|x| x),
            Player::White => prob(|x| 1. - x),
        };
        tree_node.next_node.iter()
            .max_by(|n1, n2| {
                let node1 = self.map.get(*n1).unwrap();
                let node2 = self.map.get(*n2).unwrap();
                prob(node1).partial_cmp(&prob(node2)).unwrap()
            })
            .map(|x| *x)
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

            if node.possible.len() < self.param.num_expansion {
                (node.possible.clone(), hashed)
            } else {
                let mut rng = thread_rng();
                let sampled = node.possible
                    .choose_multiple(&mut rng, self.param.num_expansion)
                    .cloned()
                    .collect::<Vec<_>>();
                (sampled, hashed)
            }
        };
        let child_num = self.map.get(&parent_hashed).unwrap().num_player + 1;
        let mut boards = Vec::new();
        let mut hashes = Vec::new();

        for (row, col) in possible.iter() {
            let board = sim.simulate(*row, *col).board();
            let hashed_board = hash(&board);
            { // borrow mut HashMap
                let parent_node = self.map.get_mut(&parent_hashed).unwrap();
                parent_node.next_node.push(hashed_board);
            }
            if self.map.get(&hashed_board).is_none() {
                self.map.insert(hashed_board, Node::new_with_num(&board, child_num));

                boards.push(board);
                hashes.push(hashed_board);
            }
        }

        let result = self.get_from(&boards);
        if let Some((values, policies)) = result {
            let n_prob = {
                let parent_node = self.map.get(&parent_hashed).unwrap();
                parent_node.prob
            };

            for (((value, policy), hashed), pos) in values.iter()
                .zip(policies.iter())
                .zip(hashes.iter())
                .zip(possible.iter())
            {
                let node = self.map.get_mut(hashed).unwrap();
                node.visit += 1;
                node.value = *value;
                node.prob = *policy;

                let (row, col) = *pos;
                node.n_prob = n_prob[row][col];
            }
        } else {
            panic!("alpha_zero::couldn't get from python object")
        }
    }

    fn update(&mut self, sim: &Simulate, path: &Vec<(usize, usize)>) {
        let node = sim.node.borrow();
        let hashed = hash(&node.board);
        let q_value = {
            let tree_node = self.map.get(&hashed).unwrap();
            tree_node.next_node.iter()
                .map(|x| self.map.get(x).unwrap().value)
                .sum::<f32>()
        };
        { // borrow mut map: HashMap
            let tree_node = self.map.get_mut(&hashed).unwrap();
            tree_node.visit += 1;
            tree_node.q_value = q_value;
        }

        let mut sim = sim.deep_clone();
        for (row, col) in path.iter().rev() {
            sim.rollback_in(*row, *col);

            let node = self.map.get_mut(&hash(&sim.board())).unwrap();
            node.visit += 1;
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

    fn policy(&self, sim: &Simulate) -> (usize, usize) {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();

        let tau = 1. / self.tau;
        let parent_visit = tree_node.visit as f32;
        let prob = |node: &Node| -> f32 {
            let visit = node.visit as f32;
            visit.powf(tau) / (parent_visit - visit + 1.).powf(tau)
        };

        let hashed = tree_node.next_node.iter()
            .max_by(|n1, n2| {
                let node1 = self.map.get(*n1).unwrap();
                let node2 = self.map.get(*n2).unwrap();
                prob(node1).partial_cmp(&prob(node2)).unwrap()
            })
            .map(|x| *x)
            .unwrap();

        let max_node = self.map.get(&hashed).unwrap();
        diff_board(&node.board, &max_node.board).unwrap()
    }

    fn get_policy(&mut self, game: &Game) -> (usize, usize) {
        let simulate = Simulate::from_game(game);
        self.init(&simulate);

        let node = self.map.get(&hash(&simulate.board())).unwrap().clone();
//        if node.num_player > self.param.tau_update_term {
//            self.tau = self.param.updated_tau;
//        }
        for _ in 0..self.param.num_simulation {
            self.search(&simulate);
        }
        let res = self.policy(&simulate);

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