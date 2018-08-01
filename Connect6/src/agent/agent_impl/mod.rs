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

pub enum GameResult {
    GameEnd(Player),
    Status(Result<PlayResult, &'static str>),
}

pub struct Agent {
    is_game_end: Cell<bool>,
    game: Arc<RwLock<Game>>,
    to_thread: mpsc::Sender<Query>,
    from_thread: mpsc::Receiver<GameResult>,
    main_thread: Option<thread::JoinHandle<()>>,
}

impl Agent {
    pub fn with_start() -> Agent {
        let game = Arc::new(RwLock::new(Game::new()));
        let (to_thread, from_user) = mpsc::channel::<Query>();
        let (to_user, from_thread) = mpsc::channel::<GameResult>();

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
                            let err = GameResult::Status(Err("agent::main_thread - recv err"));
                            to_user.send(err).expect("agent::main_thread - recv, send both err");
                            continue
                        },
                    };

                    let msg = match query {
                        Query::Position(query) => query,
                        Query::Terminate => break,
                    };

                    let mut ref_game = game.write()
                        .expect("agent::main_thread RwLock of game is posioned");

                    let res = ref_game.play(msg.as_str());
                    match ref_game.is_game_end() {
                        Player::None =>
                            to_user.send(GameResult::Status(res))
                                .expect("agent::main_thread - err in sending status"),
                        player => {
                            to_user.send(GameResult::GameEnd(player))
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

        let result = match self.from_thread.recv() {
            Ok(r) => r,
            Err(_) => return Err("Agent::play - recv fail"),
        };

        if let GameResult::GameEnd(_) = result {
            self.is_game_end.set(true);
        }

        Ok(result)
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