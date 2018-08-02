extern crate rand;

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

trait Policy {
    fn num_iter(&self) -> i32;
    fn select(&self, sim: &Simulate) -> (usize, usize);
    fn update(&mut self, sim: &mut Simulate);

    fn get_policy(&self, root: &Root) -> (usize, usize);
}

struct TreeSearch {
    root: Root,
    policy: Box<Policy>,
}

impl TreeSearch {
    fn from_game(game: &Game, policy: Box<Policy>) -> TreeSearch {
        TreeSearch {
            root: Root::from_game(game),
            policy,
        }
    }

    fn search(&mut self) -> (usize, usize) {
        let num_iter = self.policy.num_iter();
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
    win: i32,
    prev: Vec<u64>,
}

impl Node {
    fn new() -> Node {
        Node {
            visit: 0,
            win: 0,
            prev: Vec::new(),
        }
    }
}

struct DefaultPolicy {
    map: HashMap<Board, Node>,
}

impl DefaultPolicy {
    fn new() -> DefaultPolicy {
        DefaultPolicy {
            map: HashMap::new(),
        }
    }
}

impl Policy for DefaultPolicy {
    fn num_iter(&self) -> i32 {
        1
    }

    fn select(&self, sim: &Simulate) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        *rng.choose(sim.possible).unwrap()
    }

    fn update(&mut self, sim: &mut Simulate) {
        let backup = sim.backup();
        let mut rng = rand::thread_rng();

        let mut win = Player::None;
        let mut play: Vec<(Player, [[Player; 19]; 19])> = Vec::new();

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

            play.push((sim.turn, *sim.board));
        }

        sim.recover(backup);

        let mut hasher = DefaultHasher::new();
        sim.board.hash(&mut hasher);

        let mut prev = hasher.finish();
        for (player, board) in play.iter() {
            let node = self.map.entry(*board).or_insert(Node::new());
            node.visit += 1;
            node.win += (*player == win) as i32;
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

        let max = self.map.iter()
            .filter(|(board, node)| node.prev.contains(&hash))
            .max_by(|(_, node1), (_, node2)| {
                let prob1 = node1.win / node1.visit;
                let prob2 = node2.win / node2.visit;
                prob1.cmp(&prob2)
            });

        if let Some((board, node)) = max {
            if let Some(pos) = diff_board(board, &root.board) {
                return pos;
            }
        }
        let mut rng = rand::thread_rng();
        if let Some(rand) = rng.choose(&root.possible) {
            return *rand;
        }

        (100, 100)
    }
}

//#[derive(Clone)]
//struct Tree {
//    visit: i32,
//    win: i32,
//    end: bool,
//    map: HashMap<Board, Box<Tree>>,
//}
//
//impl Tree {
//    fn new() -> Tree {
//        Tree {
//            visit: 0,
//            win: 0,
//            end: false,
//            map: HashMap::new(),
//        }
//    }
//
//    fn selection(&self) -> Option<Box<Tree>> {
//        let max_prob = self.map.iter().max_by(|(_, tree1), (_, tree2)| {
//            let prob1 = tree1.visit / tree1.win;
//            let prob2 = tree2.visit / tree2.win;
//
//            prob1.cmp(&prob2)
//        });
//
//        match max_prob {
//            Some((_, tree)) => Some((*tree).clone()),
//            None => None
//        }
//    }
//
//    fn expansion(&self) -> Vec<Box<Tree>> {
//
//    }
//}