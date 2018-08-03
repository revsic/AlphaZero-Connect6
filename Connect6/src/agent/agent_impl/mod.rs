#[cfg(test)]
mod tests;

use std::cell::Cell;
use std::sync::{Arc, RwLock, mpsc};
use std::thread;

use super::super::game::{Player, PlayResult, Game};

enum Query {
    Position(String),
    Terminate,
}

enum InnerResult {
    GameEnd(Player),
    Status(Result<PlayResult, &'static str>),
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    GameEnd(Player),
    Status(PlayResult),
}

pub struct Agent {
    is_game_end: Cell<bool>,
    game: Arc<RwLock<Game>>,
    to_thread: mpsc::Sender<Query>,
    from_thread: mpsc::Receiver<InnerResult>,
    main_thread: Option<thread::JoinHandle<()>>,
}

impl Agent {
    pub fn with_start() -> Agent {
        let game = Arc::new(RwLock::new(Game::new()));
        let (to_thread, from_user) = mpsc::channel::<Query>();
        let (to_user, from_thread) = mpsc::channel::<InnerResult>();

        Agent {
            is_game_end: Cell::new(false),
            game: game.clone(),
            to_thread,
            from_thread,
            main_thread: Some(thread::spawn(move || {
                loop {
                    let query = match from_user.recv() {
                        Ok(q) => q,
                        Err(_) => {
                            let err = InnerResult::Status(Err("agent::main_thread - recv err"));
                            to_user.send(err).expect("agent::main_thread - recv, send both err");
                            continue
                        },
                    };

                    let msg = match query {
                        Query::Position(query) => query,
                        Query::Terminate => break,
                    };

                    let (res, is_game_end) = {
                        let mut ref_game = game.write()
                            .expect("agent::main_thread RwLock of game is posioned");

                        let res = ref_game.play(msg.as_str());
                        (res, ref_game.is_game_end())
                    };
                    match is_game_end {
                        Player::None =>
                            to_user.send(InnerResult::Status(res))
                                .expect("agent::main_thread - err in sending status"),
                        player => {
                            to_user.send(InnerResult::GameEnd(player))
                                .expect("agent::main_thread - err in sending game end");
                            break;
                        }
                    }
                }
            })),
        }
    }

    pub fn get_game(&self) -> Arc<RwLock<Game>> {
        self.game.clone()
    }

    pub fn play(&self, query: &String) -> Result<GameResult, &'static str> {
        let pos_query = Query::Position(query.clone());
        match self.to_thread.send(pos_query) {
            Ok(_) => (),
            Err(_) => return Err("Agent::play - send fail"),
        };

        let inner_result = match self.from_thread.recv() {
            Ok(r) => r,
            Err(_) => return Err("Agent::play - recv fail"),
        };

        let play_result = match inner_result {
            InnerResult::GameEnd(player) => {
                self.is_game_end.set(true);
                return Ok(GameResult::GameEnd(player))
            },
            InnerResult::Status(result) => result,
        };

        match play_result {
            Ok(result) => Ok(GameResult::Status(result)),
            Err(e) => Err(e),
        }
    }

    pub fn terminate(&mut self) {
        self.to_thread.send(Query::Terminate)
            .expect("test_with_start : couldn't terminate");

        self.is_game_end.set(true);
        if let Some(handle) = self.main_thread.take() {
            handle.join()
                .expect("test_with_start : handle join fail");
        }
    }
}

impl Drop for Agent {
    fn drop(&mut self) {
        if self.is_game_end.get() {
            return;
        }

        self.to_thread.send(Query::Terminate)
            .expect("agent.drop - send terminate code fail");

        if let Some(handle) = self.main_thread.take() {
            handle.join()
                .expect("agent.drop - join thread handle fail");
        }
    }
}