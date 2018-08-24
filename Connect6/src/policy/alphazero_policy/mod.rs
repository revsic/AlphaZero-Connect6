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

use cpython::{Python, PyObject, PySequence, PyTuple, ObjectProtocol, ToPyObject};
use self::rand::distributions::{Distribution, Dirichlet};
use self::rand::thread_rng;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::{Policy, Simulate, diff_board};
use super::super::pybind::{pylist_from_multiple, pyseq_to_vec};
use super::super::game::{Game, Player};
use super::super::{BOARD_SIZE, Board};

mod augment;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod augment_test;

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
    pub epsilon: f32,
    pub dirichlet_alpha: f64,
    pub c_puct: f32,
}

impl HyperParameter {
    pub fn default() -> HyperParameter {
        HyperParameter {
            num_simulation: 800,
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
    fn get_from(&self, turn: Player, board: &Board)
                -> Option<(f32, [[f32; BOARD_SIZE]; BOARD_SIZE])> {
        let (value_vec, policy_vec) = {
            // acquire python gil
            let gil = Python::acquire_gil();
            let py = gil.python();

            // convert parameter to python object
            let py_turn = (turn as i32).to_py_object(py);
            let py_board = pylist_from_multiple(py, &augment::augment_way8(board));
            let res = must!(self.obj.call(py, (py_turn, py_board), None), "alpha_zero::get_from couldn't call pyobject");
            let pytuple = must!(res.cast_into::<PyTuple>(py), "alpha_zero::get_from couldn't cast into pytuple");

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
        let value = value_vec.iter().sum::<f32>() / 8.;

        let mut vec = Vec::new();
        // unpack the board
        for policy in policy_vec.iter() {
            let mut temporal = [[0.; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    temporal[i][j] = policy[i * BOARD_SIZE + j];
                }
            }
            vec.push(temporal);
        }
        let mut recovered = augment::recover_way8(vec);
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                recovered[i][j] *= (board[i][j] == Player::None) as i32 as f32;
            }
        }
        Some((value, recovered))
    }

    /// Get best child node from current simulation based on policy.
    ///
    /// # Errors
    /// - if given simulation is end game.
    ///
    /// # Panics
    /// - if result of `prob` is NaN.
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
    fn init(&mut self, sim: &Simulate) {
        let hashed = hash(&sim.board());
        let node = self.map.entry(hashed).or_insert(Node::new(&node.board));
        node.turn = sim.turn;
    }

    // TODO : 지금 turn이라는 개념이 모호함, evaluator 인지 진짜 turn 인지 확실시 하는게 좋을거 같음
    // TODO : 근데 또 보면 evaluator 가 고정이어야 하는거로 보임 그니까 결국은 self-play지만 self.map을 나누어 가지고 있다고 가정해야 될 듯 함
    // TODO : 그렇게 되면 evaluator가 필요하지 않고 turn을 가지고 selection 때 자식 노드와 turn 매칭 해서 ?
    // TODO : 필요할진 모르겠지만 참고하면 좋을 듯 합
    /// Select position based on policy
    ///
    /// # Errors
    /// - if given simulation is end game.
    /// - if method couldn't find any different positions between maximum value node and given.
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

    /// Expand the tree in given simulation
    ///
    /// Evaluate self and add all possible child nodes to parent node with given proper probability.
    ///
    /// # Panics
    /// - if method couldn't get value and prob from pyobject.
    fn expand(&mut self, sim: &Simulate) {
        let board = sim.board();
        if let Some((value, prob)) = self.get_from(sim.turn, &board) {
            let parent_hashed = hash(&board);
            let child_num = { // borrow mut self.map: HasMap
                let parent_node = self.map.get_mut(&parent_hashed).unwrap();
                parent_node.value = value;
                parent_node.prob = prob;
                parent_node.visit += 1;
                parent_node.num_player + 1
            };

            let mut hashed_vec = Vec::new();
            for (row, col) in sim.possible() {
                let sim = sim.simulate(row, col);
                let board = sim.board();
                let hashed = hash(&board);
                hashed_vec.push(hashed);

                let tree_node = self.map.entry(hashed).or_insert(Node::new_with_num(&board, child_num));
                tree_node.n_prob = prob[row][col];
                tree_node.turn = sim.turn;
            }

            let parent_node = self.map.get_mut(&parent_hashed).unwrap();
            for hashed in hashed_vec.into_iter() {
                parent_node.next_node.push(hashed);
            }
        } else {
            panic!("alpha_zero::expand couldn't get value, prob from pyobject");
        }
    }

    /// Update the tree with given path (searching history, parent nodes)
    ///
    /// Update q_sum of immediate parent node and update all parents with visit count
    fn update(&mut self, sim: &Simulate, path: &Vec<(usize, usize)>) {
        if let Some((row, col)) = path.last() {
            let value = {
                let node = self.map.get(&hash(&sim.board())).unwrap();
                node.value
            };
            { // borrow mut self.map: HashMap
                let mut sim = sim.deep_clone();
                sim.rollback_in(*row, *col);

                let node = self.map.get_mut(&hash(&sim.board())).unwrap();
                node.q_sum += value;
            }
            let mut sim = sim.deep_clone();
            for (row, col) in path.iter().rev() {
                sim.rollback_in(*row, *col);
                let node = self.map.get_mut(&hash(&sim.board())).unwrap();
                node.visit += 1;
                node.recalc_q();
            }
        }
    }

    // TODO : check empty child node and no-one win
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

        // remove siblings
        let node = self.map.get(&hash(&simulate.board())).unwrap().clone();
        let num_player = node.num_player;
        let sibling = self.map.iter()
            .filter(|(_, node)| node.num_player == num_player)
            .map(|(hash, _)| *hash)
            .collect::<Vec<u64>>();

        for hashed in sibling {
            self.map.remove(&hashed);
        }
        // add root
        self.map.insert(hash(&node.board), node);
        res
    }
}