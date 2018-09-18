//! Implementation of policy `AlphaZero` based on combined MCTS with non-linear value approximator.
//!
//! `AlphaZero` policy is implemented based on [Mastering the game of Go with deep neural networks and tree search](https://www.nature.com/articles/nature16961)
//! and [Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm](https://arxiv.org/abs/1712.01815).
//!
//! It pass callable python object with method `__call__(self, turn, board): (value, prob)`
//! and make decision with combined MCTS and value, probability approximator as given.
//!
//! # Examples
//! ```rust
//! // pyobj = lambda t, b: (np.random.rand(len(b)), np.random.rand(len(b), board_size ** 2))
//! let mut policy = AlphaZero::new(pyobj);
//! let result = Agent::new(&mut policy).play();
//! assert!(result.is_ok());
//! ```
use game::{Game, Player};
use policy::{diff_board, Policy, Simulate};
use pybind::{pylist_from_multiple, pyseq_to_vec, PyEval};
use {Board, BOARD_CAPACITY, BOARD_SIZE};

use cpython::{ObjectProtocol, PyObject, PySequence, PyTuple, Python, ToPyObject};
use rand::distributions::{Dirichlet, Distribution};
use rand::prelude::{thread_rng, IteratorRandom};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod augment;

#[cfg(test)]
mod augment_tests;
#[cfg(test)]
mod tests;

/// Tree node, get next node as hash value of board
#[derive(Clone, Debug)]
struct Node {
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
    /// To avoid the overhead, use method `new_with_num`
    ///
    /// # Exmaples
    /// ```rust
    /// let game = Game::new();
    /// let node = Node::new(game.get_board());
    /// ```
    fn new(board: &Board) -> Node {
        let num_player = board
            .iter()
            .map(|x| x.iter().filter(|x| **x != Player::None).count())
            .sum();
        Node {
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
/// Default parameter is based on paper [AlphaGo Zero](https://www.nature.com/articles/nature24270)
/// - num_simulation : number of simulation in tree search, default 800.
/// - epsilon : exploit, exploration parameter, default 0.25.
/// - dirichlet_alpha : param for diriclet random distribution, default 0.03.
/// - c_puct : param for modulating q_value and probability, default 1.
///
#[derive(Copy, Clone)]
pub struct HyperParameter {
    pub num_simulation: i32,
    pub epsilon: f32,
    pub dirichlet_alpha: f64,
    pub c_puct: f32,
}

impl HyperParameter {
    /// Generate default HyperParameter
    pub fn default() -> HyperParameter {
        HyperParameter {
            num_simulation: 800,
            epsilon: 0.25,
            dirichlet_alpha: 0.03,
            c_puct: 1.,
        }
    }
}

/// Evaluator for applying value, policy approximator to `AlphaZero`.
pub trait Evaluator {
    fn eval(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)>;
}

/// Implementation of policy `AlphaZero` based on combined MCTS with non-linear value approximator.
///
/// `AlphaZero` policy is implemented based on [Mastering the game of Go with deep neural networks and tree search](https://www.nature.com/articles/nature16961)
/// and [Mastering Chess and Shogi by Self-Play with a General Reinforcement Learning Algorithm](https://arxiv.org/abs/1712.01815).
///
/// It pass callable python object with method `__call__(self, turn, board): (value, prob)`
/// and make decision with combined MCTS and value, probability approximator as given.
///
/// # Examples
/// ```rust
/// // pyobj = lambda t, b: (np.random.rand(len(b)), np.random.rand(len(b), board_size ** 2))
/// let mut policy = AlphaZero::new(pyobj);
/// let result = Agent::new(&mut policy).play();
/// assert!(result.is_ok());
/// ```
pub struct AlphaZero {
    map: HashMap<u64, Node>,
    param: HyperParameter,
    evaluator: Box<Evaluator + Send>,
}

impl AlphaZero {
    /// Construct a new `AlphaZero` policy with python evaluator
    ///
    /// # Examples
    /// ```rust
    /// // pyobj = lambda t, b: (np.random.rand(len(b)), np.random.rand(len(b), board_size ** 2))
    /// let mut policy = AlphaZero::new(pyobj);
    /// ```
    pub fn new(obj: PyObject) -> AlphaZero {
        let param = HyperParameter::default();
        AlphaZero {
            map: HashMap::new(),
            param,
            evaluator: Box::new(PyEval::new(obj)),
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
            map: HashMap::new(),
            param,
            evaluator: Box::new(PyEval::new(obj)),
        }
    }

    /// Get value and prob from `PyObject`
    ///
    /// # Panics
    /// - If `self.evaluator` raise panics
    ///
    /// # Errors
    /// - if `self.evaluator` returns `None` object
    fn get_from(
        &self,
        turn: Player,
        board: &Board,
    ) -> Option<(f32, [[f32; BOARD_SIZE]; BOARD_SIZE])> {
        let (value_vec, policy_vec) = self.evaluator.eval(turn, &augment::augment_way8(board))?;
        let value = value_vec.iter().sum::<f32>() / 8.;

        let mut recovered = augment::recover_way8(policy_vec);
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                // masking already set point
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
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_nodes = tree_node
            .next_node
            .iter()
            .map(|x| self.map.get(x).unwrap())
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
        let puct = |node: &Node, noise: f64| {
            let noise = noise as f32;
            let visit = node.visit as f32;
            let prob = epsilon * noise + (1. - epsilon) * node.n_prob;
            prob * (visit_sum - visit).sqrt() / (1. + visit)
        };
        let unary: fn(f32) -> f32 = match sim.turn {
            Player::Black => |x| -x,
            Player::White => |x| x,
            Player::None => {
                panic!("alpha_zero::maximum_from couldn't get unary function from player none")
            }
        };
        // formula
        let prob = |(node, noise): (&Node, f64)| unary(node.q_value) + c_puct * puct(node, noise);
        let probs = child_nodes
            .into_iter()
            .zip(dirichlet.sample(&mut thread_rng()))
            .map(|n| (n.0, prob(n)))
            .collect::<Vec<_>>();

        let max = probs
            .iter()
            .max_by(|(_, p1), (_, p2)| p1.partial_cmp(p2).unwrap())?;

        probs
            .iter()
            .filter(|(_, p)| *p == max.1)
            .choose(&mut thread_rng())
            .map(|(n, _)| hash(&n.board))
    }

    /// Initialize Policy
    ///
    /// For the first tree search, tree must be initialized with game status.
    /// `Init` initialize the tree with given `Simulate`
    fn init(&mut self, sim: &Simulate) {
        let node = sim.node.borrow();
        let hashed = hash(&node.board);
        self.map.entry(hashed).or_insert(Node::new(&node.board));
    }

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
        let parent_hashed = hash(&board);
        {
            // borrow self.map: HashMap
            let cost = sim.turn as i32 as f32;
            let node = self.map.get_mut(&parent_hashed).unwrap();
            if node.num_player == BOARD_CAPACITY {
                let winner = sim.search_winner();
                if winner == sim.turn {
                    // current player win
                    node.value = cost;
                } else if winner != Player::None {
                    // current player is lose
                    node.value = -cost;
                }
                return;
            }
        }
        if let Some((value, prob)) = self.get_from(sim.turn, &board) {
            let child_num = {
                // borrow mut self.map: HasMap
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

                let tree_node = self
                    .map
                    .entry(hashed)
                    .or_insert(Node::new_with_num(&board, child_num));
                tree_node.n_prob = prob[row][col];
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
            {
                // borrow mut self.map: HashMap
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
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();
        let child_node = tree_node
            .next_node
            .iter()
            .map(|x| self.map.get(x).unwrap())
            .collect::<Vec<_>>();

        // total visit count of child nodes
        let visit_sum = child_node.iter().map(|x| x.visit).sum::<i32>() as f32;
        let prob = |node: &Node| -> f32 {
            let visit = node.visit as f32;
            visit / (visit_sum - visit + 1.)
        };
        child_node
            .into_iter()
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
        let sibling = self
            .map
            .iter()
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
