extern crate rand;

use std::any::Any;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use self::rand::Rng;
use super::simulate::*;
use super::super::game::*;

#[cfg(test)]
mod tests;

type Board = [[Player; 19]; 19];

fn diff_board(board1: &Board, board2: &Board) -> Option<(usize, usize)> {
    for row in 0..19 {
        for col in 0..19 {
            if board1[row][col] != board2[row][col] {
                return Some((row, col))
            }
        }
    }

    return None
}

pub trait Policy {
    fn as_any(&self) -> &Any;
    fn num_expand(&self) -> i32;
    fn select(&self, sim: &Simulate) -> (usize, usize);
    fn update(&mut self, sim: &mut Simulate);
    fn get_policy(&self, root: &Root) -> (usize, usize);
}

pub struct TreeSearch<'a, P> where P: 'a + Policy + Sized{
    root: Root,
    policy: &'a mut P,
}

impl<'a, P> TreeSearch<'a, P> where P: 'a + Policy + Sized {
    pub fn new(policy: &'a mut P) -> TreeSearch<'a, P> {
        TreeSearch {
            root: Root::new(),
            policy,
        }
    }

    pub fn from_game(game: &Game, policy: &'a mut P) -> TreeSearch<'a, P> {
        TreeSearch {
            root: Root::from_game(game),
            policy,
        }
    }

    pub fn search(&mut self) -> (usize, usize) {
        let num_iter = self.policy.num_expand();
        for _ in 0..num_iter {
            let mut sim = self.root.to_simulate();
            let (row, col) = self.policy.select(&sim);

            let mut selected = sim.simulate(row, col);
            self.policy.update(&mut selected);
        }

        self.policy.get_policy(&self.root)
    }
}

struct Node {
    visit: i32,
    black_win: i32,
    prev: Vec<u64>,
}

impl Node {
    fn new() -> Node {
        Node {
            visit: 1,
            black_win: 0,
            prev: Vec::new(),
        }
    }

    fn calc(player: &Player) -> fn(&Node) -> f32 {
        match *player {
            Player::None => panic!("couldn't calculate none user's prob"),
            Player::Black => |node: &Node| node.black_win as f32 / node.visit as f32,
            Player::White => |node: &Node| 1. - (node.black_win as f32 / node.visit as f32),
        }
    }
}

pub struct DefaultPolicy {
    debug: bool,
    map: HashMap<Board, Node>,
}

impl DefaultPolicy {
    pub fn new() -> DefaultPolicy {
        DefaultPolicy {
            debug: false,
            map: HashMap::new(),
        }
    }

    pub fn debug() -> DefaultPolicy {
        DefaultPolicy {
            debug: true,
            map: HashMap::new(),
        }
    }
}

impl Policy for DefaultPolicy {
    fn as_any(&self) -> &Any {
        self
    }

    fn num_expand(&self) -> i32 {
        50
    }

    fn select(&self, sim: &Simulate) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        *rng.choose(sim.possible).unwrap()
    }

    fn update(&mut self, sim: &mut Simulate) {
        let backup = sim.backup();
        let mut rng = rand::thread_rng();

        let mut win = Player::None;
        let mut play: Vec<[[Player; 19]; 19]> = Vec::new();

        loop {
            match sim.is_game_end() {
                Player::None => (),
                player => {
                    win = player;
                    break;
                }
            }
            let (row, col) = *rng.choose(sim.possible).unwrap();
            sim.simulate_mut(row, col);

            play.push(*sim.board);
        }

        sim.recover(backup);

        let mut hasher = DefaultHasher::new();
        sim.board.hash(&mut hasher);

        let mut prev = hasher.finish();
        for board in play.iter() {
            let node = self.map.entry(*board).or_insert(Node::new());
            node.visit += 1;
            if win == Player::Black {
                node.black_win += 1
            };
            node.prev.push(prev);

            let mut hasher = DefaultHasher::new();
            board.hash(&mut hasher);
            prev = hasher.finish();
        }
    }

    fn get_policy(&self, root: &Root) -> (usize, usize) {
        let mut hasher = DefaultHasher::new();
        root.board.hash(&mut hasher);

        let hash = hasher.finish();
        let calc = Node::calc(&root.turn);

        let max = self.map.iter()
            .filter(|(_, node)| node.prev.contains(&hash))
            .max_by(|(_, node1), (_, node2)| {
                calc(node1).partial_cmp(&calc(node2)).unwrap()
            });

        if let Some((board, node)) = max{
            if calc(node) != 0. {
                if let Some(pos) = diff_board(board, &root.board) {
                    if self.debug {
                        println!("find {} ({})", calc(node), node.visit);
                    }
                    return pos;
                }
            }
        }
        let mut rng = rand::thread_rng();
        if let Some(rand) = rng.choose(&root.possible) {
            if self.debug {
                println!("rand");
            }
            return *rand;
        }

        (100, 100)
    }
}