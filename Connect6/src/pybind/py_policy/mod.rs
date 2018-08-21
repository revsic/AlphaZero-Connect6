//! Implementation of policy `AlphaZero` based on combined MCTS and non-linear value, prob approximator.
//!
//! `AlphaZero` policy is implemented based on [Mastering the game of Go with deep neural networks and tree search](https://www.nature.com/articles/nature16961)
//! and [Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm](https://arxiv.org/abs/1712.01815).
//! It pass callable python object with method `__call__(self, turn, board): (value, prob)`
//! and make decision with combined mcts and value, probability approximator.
//!
//! # Examples
//! ```rust
//! // python : pyobj = lambda t, b: (np.random.rand(len(b)), np.random.rand(len(b), board_size ** 2))
//! let mut policy = AlphaZero::new(pyobj);
//! let result = Agent::new(&mut policy).play();
//! assert!(result.is_ok());
//! ```
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

/// Tree node, get next node as hash value of board
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
    /// Construct a new Node
    ///
    /// It generate the number of stones in board.
    /// To avoid the overhead, use method `use_with_num`
    ///
    /// # Exmaples
    /// ```rust
    /// let game = Game::new();
    /// let node = Node::new(game.get_board());
    /// ```
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

    /// Construct a Node with given number of stones(player) in board
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let node = Node::new_with_num(game.get_board(), 0);
    /// ```
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

    /// Recalculate q_value
    fn recalc_q(&mut self) {
        self.q_value = self.q_sum / self.visit as f32;
    }
}

/// Hyperparameter for implementing `AlphaZero`.
///
/// Default parameter is based on paper 'AlphaGo Zero'
/// - `num_simulation` : number of simulation in tree search, default 800.
/// - `num_expansion` : number of child node generation in expansion step, default 1.
/// - `epsilon` : param for exploit, exploration, `e * noise + (1 - e) * prob`,  default 0.24.
/// - `dirichlet_alpha` : param for diriclet random distribution, default 0.03.
/// - `c_puct` : param for modulating q_value and probability, default 1..
///
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

/// Implementation of policy `AlphaZero` based on combined MCTS and non-linear value, prob approximator.
///
/// `AlphaZero` policy is implemented based on [Mastering the game of Go with deep neural networks and tree search](https://www.nature.com/articles/nature16961)
/// and [Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm](https://arxiv.org/abs/1712.01815).
/// It pass callable python object with method `__call__(self, turn, board): (value, prob)`
/// and make decision with combined mcts and value, probability approximator.
///
/// # Examples
/// ```rust
/// // python : pyobj = lambda t, b: (np.random.rand(len(b)), np.random.rand(len(b), board_size ** 2))
/// let mut policy = AlphaZero::new(pyobj);
/// let result = Agent::new(&mut policy).play();
/// assert!(result.is_ok());
/// ```
pub struct AlphaZero {
    obj: PyObject,
    map: HashMap<u64, Node>,
    param: HyperParameter,
}

impl AlphaZero {
    /// Construct a new `AlphaZero` policy.
    ///
    /// # Examples
    /// ```rust
    /// // python : pyobj = lambda t, b: (np.random.rand(len(b)), np.random.rand(len(b), board_size ** 2))
    /// let mut policy = AlphaZero::new(pyobj);
    /// ```
    pub fn new(obj: PyObject) -> AlphaZero {
        let param = HyperParameter::default();
        AlphaZero {
            obj,
            map: HashMap::new(),
            param,
        }
    }

    /// Construct a `AlphaZero` with given hyperparameters.
    ///
    /// # Examples
    /// ```rust
    /// let param = HyperParameter::new();
    /// let mut policy = AlphaZero::new(pyobj, param);
    /// ```
    pub fn with_param(obj: PyObject, param: HyperParameter) -> AlphaZero {
        AlphaZero {
            obj,
            map: HashMap::new(),
            param,
        }
    }

    /// Get value and prob from `PyObject`
    ///
    /// # Panics
    /// - If `self.obj` is not callable object, or method `__call__` is not a type of `__call__(self, turn, board): (value, prob)`
    /// - if return value of `self.obj.call()` is not a tuple type object.
    ///
    /// # Errors
    /// - if `value` is not a sequence type object consists of floats.
    /// - if `policy` is not a 2D sequence type object consists of floats.
    /// - if `policy` is not shaped `[boards.len(), BOARD_SIZE ** 2]`
    fn get_from(&self, next_turn: Player, boards: &Vec<Board>)
                -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        let (value_vec, policy_vec) = {
            // acquire python gil
            let gil = Python::acquire_gil();
            let py = gil.python();

            // convert parameter to python object
            let py_turn = (next_turn as i32).to_py_object(py);
            let py_board = pylist_from_multiple(py, boards);
            let res = pycheck!(self.obj.call(py, (py_turn, py_board), None), "alpha_zero::get_from couldn't call pyobject");
            let pytuple = pycheck!(res.cast_into::<PyTuple>(py), "alpha_zero::get_from couldn't cast into pytuple");

            let value = pytuple.get_item(py, 0);
            let policy = pytuple.get_item(py, 1);

            // convert python object to proper vector
            let value_vec = pyseq_to_vec(py, value)?;
            let policy_vec = policy.cast_into::<PySequence>(py).ok()?
                .iter(py).ok()?
                .filter_map(|x| x.ok())
                .filter_map(|x| pyseq_to_vec(py, x))
                .collect::<Vec<Vec<f32>>>();

            (value_vec, policy_vec)
        };

        let mut vec = Vec::new();
        // unpack the board
        for (board, policy) in boards.iter().zip(policy_vec.iter()) {
            let mut temporal = [[0.; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    let mask = (board[i][j] == Player::None) as i32 as f32;
                    // probability masking on already set position
                    temporal[i][j] = policy[i * BOARD_SIZE + j] * mask;
                }
            }
            vec.push(temporal)
        }
        Some((value_vec, vec))
    }

    /// Get best child node from current simulation based on policy.
    fn maximum_from(&self, sim: &Simulate) -> Option<u64> {
        let next_turn = sim.next_turn();
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_nodes = tree_node.next_node.iter()
            .map(|x| self.map.get(x).unwrap())
            .filter(|x| x.turn == next_turn)
            .collect::<Vec<_>>();
        if child_nodes.is_empty() {
            // couldn't get maximum value from empty child
            return None;
        } else if child_nodes.len() == 1 {
            // heuristic
            return Some(hash(&child_nodes[0].board));
        }

        // exploit, exploration
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
        // formula
        let prob = |(node, noise): &(&Node, f64)| node.q_value + c_puct * puct(node, noise);
        child_nodes.into_iter()
            .zip(dirichlet.sample(&mut thread_rng()))
            .max_by(|n1, n2| prob(n1).partial_cmp(&prob(n2)).unwrap())
            .map(|x| hash(&x.0.board))
    }

    /// Initialize Policy
    ///
    /// For the first tree search, tree must be initialized with game status.
    /// `Init` initialize the tree with given `Simulate`
    ///
    /// # Panics
    /// If `init` couldn't get proper value and prob from `get_from`.
    fn init(&mut self, sim: &Simulate) {
        let node = sim.node.borrow();
        let hashed = hash(&node.board);

        // didn't visit current simulation in history
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

    /// Select position based on policy
    ///
    /// *Note* Given simulation must be initialized by `init` or `expand`.
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

    // TODO : evaluate self and make all possible nodes to child nodes, save n_prob
    /// Expand the tree in given simulation
    ///
    /// Child node of given simulation should be empty.
    /// So that `expand` would push the childs with proper value and probability.
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
        let mut hashes2 = Vec::new();
        let mut boards = Vec::new();
        // number of expansion, minimum value of possible selections and `param.num_expansion`
        let n_expansion = min(possible.len(), self.param.num_expansion);
        for (row, col) in possible.into_iter() {
            // collect board
            let board = sim.simulate(row, col).board();
            let hashed = hash(&board);
            let entry = self.map.entry(hashed).or_insert(Node::new_with_num(&board, child_num));
            // if node is initialized just now, node.turn is Player::None
            // if node is exist already and node.turn is different from next turn, it must re-evaluate.
            // if node is exist already and node.turn is same with next turn, it can be hold.
            if entry.turn != next_turn {
                entry.turn = next_turn;
                // for update child node with evaluation
                hashes.push(hashed);

                // collect first `n_expansion` boards for evaluation
                if idx < n_expansion {
                    boards.push(board);
                }
                idx += 1;
            } else {
                // for adding child node to parent node
                hashes2.push(hashed);
            }
            entry.n_prob = prob[row][col];
        }
        { // borrow mut self.map: HashMap
            let parent_node = self.map.get_mut(&parent_hashed).unwrap();
            for hashed in hashes.iter() {
                parent_node.next_node.push(*hashed);
            }
            for hashed in hashes2.into_iter() {
                parent_node.next_node.push(hashed);
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

    // TODO : Renew based on new `expand`
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

    /// Search the tree. Pack of select, expand, update.
    fn search(&mut self, simulate: &Simulate) {
        // 1. initialize
        self.init(&simulate);
        let mut simulate = simulate.deep_clone();
        let mut path = Vec::new();
        // 2. searching the tree with selection policy
        while let Some((row, col)) = self.select(&simulate) {
            path.push((row, col));
            simulate.simulate_in(row, col);
        }

        if simulate.search_winner() != Player::None {
            return;
        }
        // 3. expansion
        self.expand(&simulate);
        // 4. update
        self.update(&simulate, &path);
    }

    /// Generate the policy based on visit count
    ///
    /// # Panics
    /// - If comparison error occured between two floats
    /// - If boards of selected child node and parent node have no difference.
    fn policy(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let next_turn = sim.next_turn();
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_node = tree_node.next_node.iter()
            .map(|x| self.map.get(x).unwrap())
            .filter(|x| x.turn == next_turn)
            .collect::<Vec<_>>();

        // total visit count of child nodes
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

/// Generate hash value from given board
fn hash(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

impl Policy for AlphaZero {
    /// Select next position with `AlphaZero` policy
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