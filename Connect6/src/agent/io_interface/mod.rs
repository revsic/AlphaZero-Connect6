#[cfg(test)]
mod tests;

use std::io;
use std::sync::{Arc, RwLock};
use super::super::game::{Player, Game};
use super::{Agent, GameResult};

#[macro_export]
macro_rules! play_with_stdio {
    () => {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        let mut agent_io = AgentIO::new(
            &mut stdin as &mut io::Read,
            &mut stdout as &mut io::Write,
        );
        agent_io.play();
    }
}

struct AgentIO<'a> {
    agent: Agent,
    reader: &'a mut io::Read,
    writer: &'a mut io::Write,
}

impl<'a> AgentIO<'a> {
    fn new(reader: &'a mut io::Read, writer: &'a mut io::Write) -> AgentIO<'a> {
        AgentIO {
            agent: Agent::with_start(),
            reader,
            writer,
        }
    }

    fn draw_board(&mut self, game: &Arc<RwLock<Game>>) -> io::Result<usize> {
        let game = game.read()
            .expect("agent_io::draw_board - game rwlock is poisoned");
        game.print(self.writer)
    }

    fn play(&mut self) -> Player {
        let game = self.agent.get_game();

        let mut player = Player::Black;
        let mut num_remain = 1;
        loop {
            let msg = format!("{:?} - remain {}\n", player, num_remain);
            self.writer.write(msg.as_bytes())
                .expect("agent_io::play - write msg fail");

            self.draw_board(&game).expect("agent_io::play - draw_board fail");

            let mut buffer = [0; 10];
            self.reader.read(&mut buffer)
                .expect("agent_io::play - couldn't read from self.reader");

            let query: String = buffer.iter()
                .filter(|x| x.is_ascii_alphabetic())
                .map(|x| *x as char).collect();

            if query.len() != 2 {
                self.writer.write(b"invalid input, retry\n")
                    .expect("agent_io::play - write invalid query msg fail");

                continue;
            }

            let game_result = match self.agent.play(&query) {
                Err(e) => {
                    self.writer.write(e.as_bytes())
                        .expect("agent_io::play - write err of game_result fail");

                    continue
                },
                Ok(result) => result,
            };

            let game_play = match game_result {
                GameResult::GameEnd(player) => {
                    let msg = format!("{:?} - win\n", player);
                    self.writer.write(msg.as_bytes())
                        .expect("agent_io::play - write game end fail");

                    return player;
                },
                GameResult::Status(status) => status,
            };

            let game_result = match game_play {
                Err(e) => {
                    self.writer.write(e.as_bytes())
                        .expect("agent_io::play - write err of game_result fail");

                    continue
                },
                Ok(result) => result,
            };

            let msg = format!("{:?} - remain {} - pos {:?}\n",
                              game_result.player,
                              game_result.num_remain,
                              game_result.position);
            self.writer.write(msg.as_bytes())
                .expect("agent_io::play - write result msg fail");

            num_remain -= 1;
            if num_remain <= 0 {
                num_remain = 2;
                player.mut_switch();
            }
        }
    }
}