//! Policy for pure Monte Carlo tree search implementation
//!
//! # Examples
//! ```rust
//! let mut policy = DefaultPolicy::new();
//! let result = Agent::new(&mut policy).play();
//! assert!(result.is_ok());
//! ```
extern crate rand;

use std::collections::HashMap;
use std::collections::hash_map::*;
use std::hash::{Hash, Hasher};

use self::rand::seq::*;
use self::rand::prelude::*;
use super::Policy;
use super::simulate::Simulate;
use super::super::game::*;
use super::super::{BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

/// Tree node, get child node as hash value of board array
struct Node {
    visit: i32,
    black_win: i32,
    board: Board,
    next_node: Vec<u64>,
}

impl Node {
    /// Construct a new `Node`
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let node = Node::new(game.get_board());
    /// ```
    fn new(board: &Board) -> Node {
        Node {
            visit: 0,
            black_win: 0,
            board: *board,
            next_node: Vec::new(),
        }
    }
}

/// generate hash value of board
///
/// # Examples
/// ```rust
/// let game = Game::new();
/// let hashed = hash(game.get_board());
/// assert_eq!(hashed, hash([[Player::None; BOARD_SIZE]; BOARD_SIZE]));
/// ```
fn hash(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

/// compare the board and return the difference between position by position.
///
/// # Examples
/// ```rust
/// let game = Game::new();
/// let mut sim = Simulate::from_game(&game);
/// sim.simulate_in(0, 0);
///
/// let diff = diff_board(game.get_board(), &sim.board());
/// assert_eq!(diff, Some((0, 0)));
/// ```
pub fn diff_board(board1: &Board, board2: &Board) -> Option<(usize, usize)> {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board1[row][col] != board2[row][col] {
                return Some((row, col))
            }
        }
    }
    return None
}
/// Policy for pure Monte Carlo tree search implementation
///
/// # Examples
/// ```rust
/// let mut policy = DefaultPolicy::new();
/// let result = Agent::new(&mut policy).play();
/// assert!(result.is_ok());
/// ```
pub struct DefaultPolicy {
    num_iter: i32,
    map: HashMap<u64, Node>,
}

impl DefaultPolicy {
    /// Construct a new `DefaultPolicy`
    pub fn new() -> DefaultPolicy {
        DefaultPolicy {
            num_iter: 50,
            map: HashMap::new(),
        }
    }

    /// Construct a `DefaultPolicy` with number of iteration in simulation task.
    pub fn with_num_iter(num_iter: i32) -> DefaultPolicy {
        DefaultPolicy {
            num_iter,
            map: HashMap::new(),
        }
    }

    /// Initialize policy
    ///
    /// For the first tree search, tree must be initialized with game status.
    /// `Init` initialize the tree with given `Simulate`
    ///
    /// # Examples
    /// ```rust
    /// let sim = Simulate::new();
    /// let mut policy = DefaultPolicy();
    /// policy.init(&sim);
    /// ```
    fn init(&mut self, sim: &Simulate) {
        let board = sim.board();
        self.map.entry(hash(&board)).or_insert(Node::new(&board));
    }

    /// Select the position of the highest winning probability.
    ///
    /// *Note* Given simulation must be initialized by `init` or `expand`.
    fn select(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();

        // `Node` structure is based on player Black.
        // To calculate probability of given player, it should condition on given player and apply unary function.
        let unary: fn(f32) -> f32 = match sim.turn {
            Player::None => panic!("couldn't calculate none user's prob"),
            Player::Black => |x| x,
            Player::White => |x| -x,
        };
        let prob = |node: &Node| unary(node.black_win as f32 / (1. + node.visit as f32));
        // get the maximum probability node
        let max = tree_node.next_node.iter()
            .max_by(|n1, n2| {
                let node1 = self.map.get(*n1).unwrap();
                let node2 = self.map.get(*n2).unwrap();
                prob(node1).partial_cmp(&prob(node2)).unwrap()
            });

        // if tree_node.next_node is not empty
        if let Some(hashed) = max {
            let max_node = self.map.get(hashed).unwrap();
            // if child_node has meaningful probability
            if prob(max_node) != 0. {
                let pos = diff_board(&max_node.board, &node.board);
                return pos;
            }
        }
        None
    }

    /// Expand the tree in given simulation
    fn expand(&mut self, sim: &Simulate) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let (row, col) = {
            let node = sim.node.borrow();
            *node.possible.choose(&mut rng).unwrap()
        };
        // simulate random selected position
        let board = sim.simulate(row, col).board();
        let hashed_board = hash(&board);
        // generate node
        self.map.insert(hashed_board, Node::new(&board));

        let parent_node = {
            let node = sim.node.borrow();
            self.map.get_mut(&hash(&node.board)).unwrap()
        };
        // make connection between parent and child
        parent_node.next_node.push(hashed_board);

        (row, col)
    }

    /// Update the tree, random simulation on child node and update visit count of parents'.
    ///
    /// Make random simulation of child node and trace to update visit count, black_win of parent nodes.
    /// If random simulation of child node is end with no one win, method will be returned without update.
    fn update(&mut self, sim: &Simulate, path: &Vec<(usize, usize)>) {
        let mut simulate = sim.deep_clone();
        let mut rng = rand::thread_rng();
        // random simulation
        while simulate.search_winner() == Player::None {
            let (row, col) = {
                let node = simulate.node.borrow();
                match node.possible.choose(&mut rng) {
                    Some(pos) => *pos,
                    None => break,
                }
            };
            simulate.simulate_in(row, col);
        }
        let win = simulate.search_winner();
        if win == Player::None {
            return;
        }
        let black_win = (win == Player::Black) as i32;

        // update parent node
        let mut sim = sim.deep_clone();
        let mut update = |sim: &Simulate| {
            let node = self.map.get_mut(&hash(&sim.board())).unwrap();
            node.visit += 1;
            node.black_win += black_win;
        };

        update(&sim);
        // trace the parent nodes
        for (row, col) in path.iter().rev() {
            sim.rollback_in(*row, *col);
            update(&sim);
        }
    }

    /// Search the tree. Pack of select, expand, update.
    fn search(&mut self, game: &Game) {
        // 1. initialize
        let mut simulate = Simulate::from_game(game);
        self.init(&simulate);

        // 2. searching the tree with selection policy
        let mut path = Vec::new();
        while let Some((row, col)) = self.select(&simulate) {
            // store the history for method `update` to trace parents
            path.push((row, col));
            simulate.simulate_in(row, col);
        }

        if simulate.search_winner() != Player::None {
            return;
        }
        // 3. expand
        let (row, col) = self.expand(&simulate);

        path.push((row, col));
        simulate.simulate_in(row, col);
        // 4. update
        self.update(&simulate, &path);
    }

    /// Generate the policy, prob based selection or else random selection.
    fn policy(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let res = if let Some(pos) = self.select(sim) {
            pos
        } else {
            let node = sim.node.borrow();
            *node.possible.choose(&mut thread_rng()).unwrap()
        };
        Some(res)
    }
}

impl Policy for DefaultPolicy {
    /// Select position based on pure MCTS.
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        // Simulation
        for _ in 0..self.num_iter {
            self.search(game);
        }
        let simulate = Simulate::from_game(game);
        // generate
        self.policy(&simulate)
    }
}