extern crate cpython;
extern crate rand;

use cpython::*;
use self::rand::Rng;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::pybind_impl::*;
use super::super::{game::*, mcts::*};

#[cfg(test)]
mod tests;

type Board = [[Player; 19]; 19];

struct Node {
    visit: i32,
    value: f32, //probability of black win
    q_value: f32,
    n_prob: f32,
    prob: [[f32; 19]; 19],
    board: [[Player; 19]; 19],
    next_node: Vec<u64>,
}

impl Node {
    fn new(board: &[[Player; 19]; 19]) -> Node {
        Node {
            visit: 0,
            value: 0.,
            q_value: 0.,
            n_prob: 0.,
            prob: [[0.; 19]; 19],
            board: *board,
            next_node: Vec::new(),
        }
    }

    fn prob(player: Player) -> (fn(&Node) -> f32) {
        match player {
            Player::None => panic!("node::prob couldn't get probability from Player::None"),
            Player::Black => |node: &Node|
                node.q_value / (node.visit as f32) + node.n_prob / (1. + node.visit as f32),
            Player::White => |node: &Node|
                (1. - node.q_value / node.visit as f32) + (1. - node.n_prob) / (1. + node.visit as f32),
        }
    }
}

pub struct AlphaZero<'a> {
    py: Python<'a>,
    obj: PyObject,
    num_iter: i32,
    map: HashMap<u64, Node>,
}

impl<'a> AlphaZero<'a> {
    fn default_num_iter() -> i32 {
        1
    }

    pub fn new(py: Python<'a>, obj: PyObject) -> AlphaZero {
        AlphaZero {
            py,
            obj,
            num_iter: Self::default_num_iter(),
            map: HashMap::new(),
        }
    }

    pub fn with_num_iter(py: Python<'a>, obj: PyObject, num_iter: i32) -> AlphaZero {
        AlphaZero {
            py,
            obj,
            num_iter,
            map: HashMap::new(),
        }
    }

    fn get_from(&self, boards: &Vec<Board>) -> Option<(Vec<f32>, Vec<[[f32; 19]; 19]>)> {
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

        let mut masked = Vec::new();
        for (board, policy) in boards.iter().zip(policy_vec.iter()) {
            let mut temporal = [[0.; 19]; 19];
            for i in 0..19 {
                for j in 0..19 {
                    let mask = (board[i][j] == Player::None) as i32 as f32;
                    temporal[i][j] = policy[i * 19 + j] * mask;
                }
            }
            masked.push(temporal);
        }
        Some((value_vec, masked))
    }

    fn maximum_from(&self, sim: &Simulate) -> Option<u64> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();

        let prob = Node::prob(sim.turn);
        tree_node.next_node.iter()
            .max_by(|n1, n2| {
                let node1 = self.map.get(*n1).unwrap();
                let node2 = self.map.get(*n2).unwrap();
                prob(node1).partial_cmp(&prob(node2)).unwrap()
            })
            .map(|x| *x)
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

    fn expand(&mut self, sim: &Simulate) -> (usize, usize) {
        let (possible, parent_hashed) = {
            let node = sim.node.borrow();
            let board = &node.board;
            (node.possible.clone(), hash(board))
        };
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
                self.map.insert(hashed_board, Node::new(&board));

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
        (0, 0)
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
        { // boroow mut map: HashMap
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

    fn policy(&self, sim: &Simulate) -> (usize, usize) {
        if let Some(pos) = self.select(sim) {
            pos
        } else {
            let node = sim.node.borrow();
            let mut rng = rand::thread_rng();
            *rng.choose(&node.possible).unwrap()
        }
    }

    fn search(&mut self, game: &Game) {
        let mut simulate = Simulate::from_game(game);
        self.init(&simulate);

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

    fn get_policy(&mut self, game: &Game) -> (usize, usize) {
        for _ in 0..self.num_iter {
            self.search(game);
        }

        let simulate = Simulate::from_game(game);
        self.policy(&simulate)
    }
}