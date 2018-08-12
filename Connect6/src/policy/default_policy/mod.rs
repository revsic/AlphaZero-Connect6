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

struct Node {
    visit: i32,
    black_win: i32,
    board: Board,
    next_node: Vec<u64>,
}

impl Node {
    fn new(board: &Board) -> Node {
        Node {
            visit: 0,
            black_win: 0,
            board: *board,
            next_node: Vec::new(),
        }
    }

    fn prob(player: &Player) -> (fn(&Node) -> f32) {
        match *player {
            Player::None => panic!("couldn't calculate none user's prob"),
            Player::Black => |node: &Node| node.black_win as f32 / (1. + node.visit as f32),
            Player::White => |node: &Node| 1. - (node.black_win as f32 / (1. + node.visit as f32)),
        }
    }
}

fn hash(board: &Board) -> u64 {
    let mut hasher = DefaultHasher::new();
    board.hash(&mut hasher);
    hasher.finish()
}

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

pub struct DefaultPolicy {
    num_iter: i32,
    map: HashMap<u64, Node>,
}

impl DefaultPolicy {
    pub fn new() -> DefaultPolicy {
        DefaultPolicy {
            num_iter: 50,
            map: HashMap::new(),
        }
    }

    pub fn with_num_iter(num_iter: i32) -> DefaultPolicy {
        DefaultPolicy {
            num_iter,
            map: HashMap::new(),
        }
    }

    fn init(&mut self, sim: &Simulate) {
        let board = sim.board();
        self.map.entry(hash(&board)).or_insert(Node::new(&board));
    }

    fn select(&self, sim: &Simulate) -> Option<(usize, usize)> {
        let node = sim.node.borrow();
        let tree_node = self.map.get(&hash(&node.board)).unwrap();

        let prob = Node::prob(&sim.turn);
        let max = tree_node.next_node.iter()
            .max_by(|n1, n2| {
                let node1 = self.map.get(*n1).unwrap();
                let node2 = self.map.get(*n2).unwrap();
                prob(node1).partial_cmp(&prob(node2)).unwrap()
            });

        if let Some(hashed) = max {
            let max_node = self.map.get(hashed).unwrap();
            if prob(max_node) != 0. {
                let pos = diff_board(&max_node.board, &node.board);
                return pos;
            }
        }
        None
    }

    fn expand(&mut self, sim: &Simulate) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        let (row, col) = {
            let node = sim.node.borrow();
            *node.possible.choose(&mut rng).unwrap()
        };

        let board = sim.simulate(row, col).board();
        let hashed_board = hash(&board);
        self.map.insert(hashed_board, Node::new(&board));

        let parent_node = {
            let node = sim.node.borrow();
            self.map.get_mut(&hash(&node.board)).unwrap()
        };
        parent_node.next_node.push(hashed_board);

        (row, col)
    }

    fn update(&mut self, sim: &Simulate, path: &Vec<(usize, usize)>) {
        let mut simulate = sim.deep_clone();
        let mut rng = rand::thread_rng();
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

        let mut sim = sim.deep_clone();
        let mut update = |sim: &Simulate| {
            let node = self.map.get_mut(&hash(&sim.board())).unwrap();
            node.visit += 1;
            node.black_win += black_win;
        };

        update(&sim);
        for (row, col) in path.iter().rev() {
            sim.rollback_in(*row, *col);
            update(&sim);
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
        let (row, col) = self.expand(&simulate);

        path.push((row, col));
        simulate.simulate_in(row, col);
        self.update(&simulate, &path);
    }

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
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        for _ in 0..self.num_iter {
            self.search(game);
        }
        let simulate = Simulate::from_game(game);
        self.policy(&simulate)
    }
}