use super::*;
use super::super::game::*;
use super::super::agent::*;

#[cfg(test)]
mod tests;

struct Path {
    turn: Player,
    pos: (usize, usize),
}

struct RunResult {
    winner: Player,
    path: Vec<Path>,
}

struct SinglePolicyMCTS<'a, P> where P: 'a + Policy + Sized {
    policy: &'a mut P,
}

struct SeperatePolicyMCTS<'a, 'b, P, Q>
    where P: 'a + Policy + Sized,
          Q: 'b + Policy + Sized {
    black_policy: &'a mut P,
    white_policy: &'b mut Q,
}

impl<'a, P> SinglePolicyMCTS<'a, P> where P: 'a + Policy + Sized {
    fn new(policy: &'a mut P,) -> SinglePolicyMCTS<'a, P> {
        SinglePolicyMCTS {
            policy,
        }
    }

    fn run(&mut self) -> RunResult {
        let mut winner = Player::None;
        let agent = Agent::with_start();
        let game = agent.get_game();

        let mut path = Vec::new();
        loop {
            let (turn, (row, col)) = {
                let game = game.read().unwrap();
                let turn = game.get_turn();
                (turn, self.policy.get_policy(&*game))
            };

            path.push(Path{ turn, pos: (row, col) });
            let row = (row as u8 + 0x61) as char;
            let col = (col as u8 + 0x41) as char;
            let query: String = vec![row, col].iter().collect();

            match agent.play(&query) {
                Ok(GameResult::GameEnd(player)) => {
                    winner = player;
                    break;
                },
                Ok(GameResult::Status(_)) => (),
                Err(err) => panic!(format!("single_policy_mcts::run : {}", err)),
            };
        }

        RunResult { winner, path }
    }
}

impl<'a, 'b, P, Q> SeperatePolicyMCTS<'a, 'b, P, Q>
    where P: 'a + Policy + Sized,
          Q: 'b + Policy + Sized
{
    fn new(black_policy: &'a mut P, white_policy: &'b mut Q) -> SeperatePolicyMCTS<'a, 'b, P, Q> {
        SeperatePolicyMCTS {
            black_policy,
            white_policy,
        }
    }

    fn run(&mut self) -> RunResult {
        let mut winner = Player::None;
        let agent = Agent::with_start();
        let game = agent.get_game();

        let mut path = Vec::new();
        loop {
            let (turn, (row, col)) = {
                let game = game.read().unwrap();
                let turn = game.get_turn();
                let pos = match turn {
                    Player::None =>
                        panic!("seperate_policy_mcts::run : couldn't play with none player"),
                    Player::Black => self.black_policy.get_policy(&*game),
                    Player::White => self.white_policy.get_policy(&*game),
                };
                (turn, pos)
            };

            path.push(Path{ turn, pos: (row, col) });
            let row = (row as u8 + 0x61) as char;
            let col = (col as u8 + 0x41) as char;
            let query: String = vec![row, col].iter().collect();

            match agent.play(&query) {
                Ok(GameResult::GameEnd(player)) => {
                    winner = player;
                    break;
                },
                Ok(GameResult::Status(_)) => (),
                Err(err) => panic!(format!("seperate_policy_mcts::run : {}", err)),
            };
        }

        RunResult { winner, path }
    }
}