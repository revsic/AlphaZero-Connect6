#[cfg(test)]
mod tests;

use std::sync::{Arc, RwLock, mpsc};
use std::thread;

use super::super::game::{Player, PlayResult, Game};

enum Query {
    Position(String),
    Terminate,
}

enum GameResult {
    GameEnd(Player),
    Status(Result<PlayResult, &'static str>),
}

struct Agent {
    game: Arc<RwLock<Game>>,
    to_thread: mpsc::Sender<Query>,
    from_thread: mpsc::Receiver<GameResult>,
    main_thread: Option<thread::JoinHandle<()>>,
}

impl Agent {
    fn with_start() -> Agent {
        let game = Arc::new(RwLock::new(Game::new()));
        let (to_thread, from_user) = mpsc::channel::<Query>();
        let (to_user, from_thread) = mpsc::channel::<GameResult>();

        Agent {
            game: game.clone(),
            to_thread,
            from_thread,
            main_thread: Some(thread::spawn(move || {
                loop {
                    let query = match from_user.recv() {
                        Ok(q) => q,
                        Err(_) => {
                            let err = GameResult::Status(Err("Agent::main_thread - recv err"));
                            to_user.send(err).expect("Agent::main_thread - recv, send both err");
                            continue
                        },
                    };

                    let msg = match query {
                        Query::Position(query) => query,
                        Query::Terminate => break,
                    };

                    let mut ref_game = game.write()
                        .expect("Agent::main_thread RwLock of game is posioned");

                    let res = ref_game.play(msg.as_str());
                    let winner = ref_game.is_game_end();

                    if winner == Player::None {
                        to_user.send(GameResult::Status(res))
                            .expect("Agent::main_thread - err in sending status");
                    }
                    else {
                        to_user.send(GameResult::GameEnd(winner))
                            .expect("Agent::main_thread - err in sending game end");
                        break;
                    }
                }
            })),
        }
    }

    fn get_game(&self) -> Arc<RwLock<Game>> {
        self.game.clone()
    }

    fn play(&self, query: &String) -> Result<GameResult, &'static str> {
        let pos_query = Query::Position(query.clone());
        match self.to_thread.send(pos_query) {
            Ok(_) => (),
            Err(_) => return Err("Agent::play - send fail"),
        };

        let result = match self.from_thread.recv() {
            Ok(r) => r,
            Err(_) => return Err("Agent::play - recv fail"),
        };

        Ok(result)
    }
}

impl Drop for Agent {
    fn drop(&mut self) {
        self.to_thread.send(Query::Terminate)
            .expect("agent.drop - send terminate code fail");

        if let Some(handle) = self.main_thread.take() {
            handle.join()
                .expect("agent.drop - join thread handle fail");
        }
    }
}