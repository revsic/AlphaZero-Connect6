//! Game simulator with shared memory for efficient tree searching
//!
//! It provides simulation structure and some utilies to make next decision in policy.
//!
//! It shares possible selections and board by `Node` to make simulation without copying it.
//! It generate new simulation with `simulate` and recover the shared memory `Node` when it drop.
//! It can simulate itself mutablely by `simulate_in` and recover it by `rollback_in`.
//!
//! # Examples
//! ```rust
//! let game = Game::new();
//! let sim = Simulate::from_game(&game);
//! {
//!     let sim2 = sim.simulate(0, 0);
//!     let board = sim2.board();
//!     assert_eq!(board[0][0], Player::Black);
//! }
//! let board = sim.board();
//! assert_eq!(board[0][0][, Player::None);
//! ```
use std::cell::RefCell;
use std::rc::Rc;
use super::super::game::{Game, Player, search};
use super::super::{BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

/// Shared memory for making simulation wihout copying board and possible selections.
pub struct Node {
    pub board: Board,
    pub possible: Vec<(usize, usize)>,
}

impl Node {
    /// Make all possible selections within the board.
    #[inline]
    fn possible() -> Vec<(usize, usize)> {
        (0..BOARD_SIZE).flat_map(|x| (0..BOARD_SIZE).map(move |y| (x, y))).collect()
    }

    /// Construct a new `Node`
    fn new() -> Node {
        Node {
            board: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
            possible: Self::possible(),
        }
    }

    /// Construct a `Node` from the board
    ///
    /// It make possible selections depending on the board status.
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let node = Node::from_board(game.get_board());
    /// ```
    fn from_board(board: &Board) -> Node {
        let possible = Self::possible()
            .into_iter()
            .filter(|(r, c)| board[*r][*c] == Player::None)
            .collect();

        Node {
            board: *board,
            possible,
        }
    }
}

/// Game simulator with shared memory for efficient tree searching
///
/// It provides simulation structure and some utilies to make next decision in policy.
///
/// It shares possible selections and board by `Node` to make simulation without copying it.
/// It generate new simulation with `simulate` and recover the shared memory `Node` when it drop.
/// It can simulate itself mutablely by `simulate_in` and recover it by `rollback_in`.
///
/// # Examples
/// ```rust
/// let game = Game::new();
/// let sim = Simulate::from_game(&game);
/// {
///     let sim2 = sim.simulate(0, 0);
///     let board = sim2.board();
///     assert_eq!(board[0][0], Player::Black);
/// }
/// let board = sim.board();
/// assert_eq!(board[0][0][, Player::None);
/// ```
pub struct Simulate {
    pub turn: Player,
    pub num_remain: i32,
    pub pos: Option<(usize, usize)>,
    pub node: Rc<RefCell<Node>>,
}

impl Simulate {
    /// Construct a new `Simulate`.
    pub fn new() -> Simulate {
        Simulate {
            turn: Player::Black,
            num_remain: 1,
            pos: None,
            node: Rc::new(RefCell::new(Node::new())),
        }
    }

    /// Construct a `Simulate` from the game
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let sim = Simulate::from_game(&game);
    /// ```
    pub fn from_game(game: &Game) -> Simulate {
        Simulate {
            turn: game.get_turn(),
            num_remain: game.get_remain(),
            pos: None,
            node: Rc::new(RefCell::new(Node::from_board(game.get_board()))),
        }
    }

    /// *Deep* clone the simulation.
    ///
    /// With `Rc<RefCell<Node>>`, the `Clone` implementation make the shallow copy of `Node`.
    /// By this reason, we require the `deep_clone` implementation which make the *deep* copy of `Node`.
    pub fn deep_clone(&self) -> Simulate {
        let node = self.node.borrow();
        let board = node.board;

        Simulate {
            turn: self.turn,
            num_remain: self.num_remain,
            pos: None,
            node: Rc::new(RefCell::new(Node::from_board(&board))),
        }
    }

    /// Get the board from node.
    pub fn board(&self) -> Board {
        let node = self.node.borrow();
        node.board
    }

    /// Find either the winner or game end, like `Game::is_game_end`.
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let sim = Simulate::from_game(&game);
    /// assert_eq!(game.is_game_end(), sim.search_winner());
    /// ```
    pub fn search_winner(&self) -> Player {
        let board = &self.node.borrow().board;
        search(board)
    }

    /// Validate the position, invalid position or already selected position.
    ///
    /// # Examples
    /// ```rust
    /// let game = Game::new();
    /// let sim = Simulate::from_game(&game);
    /// assert!(sim.validate(0, 0));
    ///
    /// let sim2 = sim.simulate(0, 0);
    /// assert!(!sim2.validate(0, 0));
    /// ```
    pub fn validate(&self, row: usize, col: usize) -> bool {
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return false;
        }
        let board = &self.node.borrow().board;
        if board[row][col] != Player::None {
            return false;
        }
        true
    }

    /// Return next turn of game.
    pub fn next_turn(&self) -> Player {
        if self.num_remain <= 1 {
            self.turn.switch()
        } else {
            self.turn
        }
    }

    /// Make the new simulation with given position.
    ///
    /// By memory sharing of structure `Node`, once the new simulation is created,
    /// node of parents simulations will be also modified.
    /// This means, valid simulation is only *one* in same time.
    /// Simulation will recover the shared memory If it dropped,
    /// It can make the tree searching more efficiently and precisely under the borrowing system of Rust.
    ///
    /// # Examples
    /// ```rust
    /// let sim = Simulate::new();
    /// assert_eq!(sim.board()[0][0], Player::None);
    /// {
    ///     let sim2 = sim.simulate(0, 0);
    ///     assert_eq!(sim.board()[0][0], Player::Black);
    /// }
    /// assert_eq!(sim.board()[0][0], Player::None);
    /// ```
    pub fn simulate(&self, row: usize, col: usize) -> Simulate {
        let mut node = self.node.borrow_mut();
        // remove given position from possible selections
        let item = node.possible.iter()
            .position(|x| *x == (row, col));
        node.possible.remove(item.unwrap());

        node.board[row][col] = self.turn;
        // switching turn
        let (turn, num_remain) =
            if self.num_remain <= 1 {
                (self.turn.switch(), 2)
            } else {
                (self.turn, 1)
            };

        Simulate {
            turn,
            num_remain,
            pos: Some((row, col)),
            node: self.node.clone(),
        }
    }

    /// Modify the current state to simulate given position.
    ///
    /// It is for making simulation in the loop.
    /// It modify the inner state to simulate given state and do not make the new one.
    /// If you want to recover the inner state, reference `rollback_in`
    ///
    /// # Examples
    /// ```rust
    /// let mut sim = Simulate::new();
    /// assert_eq!(sim.board()[0][0], Player::None);
    /// sim.simulate_in(0, 0);
    /// assert_eq!(sim.board()[0][0], Player::Black);
    /// ```
    pub fn simulate_in(&mut self, row: usize, col: usize) {
        let mut node = self.node.borrow_mut();
        let item = node.possible.iter()
            .position(|x| *x == (row, col));
        node.possible.remove(item.unwrap());

        node.board[row][col] = self.turn;
        self.num_remain -= 1;

        if self.num_remain <= 0 {
            self.num_remain = 2;
            self.turn.mut_switch();
        }
    }

    /// Modify the inner state to recover the given simulation.
    ///
    /// It clear the given position and switching turn properly.
    ///
    /// # Examples
    /// ```rust
    /// let mut sim = Simulate::new();
    /// sim.simulate_in(0, 0);
    /// assert_eq!(sim.board()[0][0], Player::Black);
    /// sim.rollback_in(0, 0);
    /// assert_eq!(sim.board()[0][0], Player::None);
    /// ```
    pub fn rollback_in(&mut self, row: usize, col: usize) {
        let mut node = self.node.borrow_mut();

        node.board[row][col] = Player::None;
        node.possible.push((row, col));

        self.num_remain += 1;
        if self.num_remain > 2 {
            self.num_remain = 1;
            self.turn.mut_switch();
        }
    }
}

impl Drop for Simulate {
    fn drop(&mut self) {
        if let Some((row, col)) = self.pos {
            let mut node = self.node.borrow_mut();
            node.possible.push((row, col));
            node.board[row][col] = Player::None;
        }
    }
}
