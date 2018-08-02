extern crate rand;

#[cfg(test)]
mod tests;

use super::super::game::Player;
use super::agent_impl::*;

pub struct RandomPlayResult {
    pub vec: Vec<(String, String)>,
    pub winner: Player,
}

pub struct RandomPlayer {
    agent: Agent
}

impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        RandomPlayer {
            agent: Agent::with_start()
        }
    }

    pub fn play(&self) -> Result<RandomPlayResult, &'static str> {
        self.play_io(|_: &Agent| ())
    }

    pub fn play_io(&self, io_action: impl Fn(&Agent)) -> Result<RandomPlayResult, &'static str> {
        let lower: Vec<char> = (0..19).map(|x: u8| (x + 0x61) as char).collect();
        let upper: Vec<char> = (0..19).map(|x: u8| (x + 0x41) as char).collect();

        let possible = {
            let mut possible: Vec<(char, char)> = lower.iter().cloned().flat_map(
                |x| upper.iter().map(move |y| (x, *y))).collect();

            use self::rand::Rng;
            rand::thread_rng().shuffle(&mut possible);

            possible
        };

        let mut prev = String::new();
        let mut vec = Vec::new();

        for (row, col) in possible {
            let query: String = vec![row, col].into_iter().collect();

            let game_result = match self.agent.play(&query) {
                Ok(result) => result,
                Err(err) => return Err(err),
            };

            if prev.is_empty() {
                prev = query.clone();
            } else {
                vec.push((prev, query));
                prev = String::new();
            }

            io_action(&self.agent);

            if let GameResult::GameEnd(player) = game_result {
                return Ok(RandomPlayResult { vec, winner: player });
            }
        }

        Ok(RandomPlayResult { vec, winner: Player::None })
    }
}