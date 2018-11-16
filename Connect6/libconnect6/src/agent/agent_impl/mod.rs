//! Implementation of agent.
//!
//! Loop based single policy agent.
//!
//! `Agent` is structure for playing game with given policy.
//! As we pass the policy, agent play the game with given based on loop.
//! Method `play` return the `PlayResult` which consisted of winner and playing history (called path).
//!
//! # Examples
//! For black-white seperable policy, reference [MultiPolicy](../policy/struct.MultiPolicy.html).
//! ```ignore
//! # #[macro_use] extern crate connect6;
//! # use connect6::{agent::Agent, policy::{RandomPolicy, MultiPolicy}};
//! io_policy_stdio!(io_policy);
//! let mut rand_policy = RandomPolicy::new();
//!
//! let mut multi_policy = MultiPolicy::new(&mut rand_policy, &mut io_policy);
//! let result = Agent::debug(&mut multi_policy).play();
//! # assert!(result.is_ok());
//! ```
use game::{Game, Player};
use policy::Policy;
use Board;

use std::io;
use std::time::Instant;

#[cfg(test)]
mod tests;

/// Unit of playing history, turn, board and selected position.
#[derive(Debug, PartialEq)]
pub struct Path {
    pub turn: Player,
    pub board: Board,
    pub pos: (usize, usize),
}

/// Result of playing game, consists of winner and path (history of game).
pub struct PlayResult {
    pub winner: Player,
    pub path: Vec<Path>,
}

/// Loop based single policy agent.
///
/// Agent is structure for playing game with given policy.
/// As we pass the policy, agent play the game with given based on loop.
/// Method `play` return the `SetResult` and it can be converted as `PyObject`.
///
/// # Examples
/// For black-white seperable policy, reference [MultiPolicy](../policy/struct.MultiPolicy.html).
/// ```ignore
/// # #[macro_use] extern crate connect6;
/// # use connect6::{agent::Agent, policy::{RandomPolicy, MultiPolicy}};
/// io_policy_stdio!(io_policy);
/// let mut rand_policy = RandomPolicy::new();
///
/// let mut multi_policy = MultiPolicy::new(&mut rand_policy, &mut io_policy);
/// let result = Agent::debug(&mut multi_policy).play();
/// # assert!(result.is_ok());
/// ```
pub struct Agent<'a> {
    game: Game,
    debug: bool,
    policy: &'a mut Policy,
}

impl<'a> Agent<'a> {
    /// Construct a new `Agent` with given policy.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::{agent::Agent, policy::RandomPolicy};
    /// let mut policy = RandomPolicy::new();
    /// let mut agent = Agent::new(&mut policy);
    /// ```
    pub fn new(policy: &'a mut Policy) -> Agent<'a> {
        Agent {
            game: Game::new(),
            debug: false,
            policy,
        }
    }

    /// Construct a debug mode `Agent` with given policy, it will display the dbg info.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::{agent::Agent, policy::RandomPolicy};
    /// let mut policy = RandomPolicy::new();
    /// let mut agent = Agent::debug(&mut policy);
    /// ```
    pub fn debug(policy: &'a mut Policy) -> Agent<'a> {
        Agent {
            game: Game::new(),
            debug: true,
            policy,
        }
    }

    /// Self-play the game with given policy.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::{agent::Agent, policy::RandomPolicy};
    /// let mut policy = RandomPolicy::new();
    /// let mut agent = Agent::new(&mut policy);
    ///
    /// let result = agent.play();
    /// assert!(result.is_ok());
    /// println!("winner: {:?}", result.unwrap().winner);
    /// ```
    ///
    /// # Errors
    /// if selected position raise Err at [Game::play](../game/struct.Game.html#method.play).
    pub fn play(&mut self) -> Result<PlayResult, String> {
        let mut winner = Player::None;
        let mut path = Vec::new();
        let game = &mut self.game;

        loop {
            if self.debug {
                game.print(&mut io::stdout()).unwrap();
            }
            let before = Instant::now();
            let pos = self.policy.next(&game);
            let duration = before.elapsed();

            // if policy could't generate next selection
            if pos.is_none() {
                break;
            }
            let pos = pos.unwrap();
            path.push(Path {
                turn: game.get_turn(),
                board: *game.get_board(),
                pos,
            });

            match game.set(pos) {
                Ok(result) => if self.debug {
                    // log the selection info
                    let (row, col) = result.position;
                    let row = (row as u8 + 0x61) as char;
                    let col = (col as u8 + 0x41) as char;
                    println!(
                        "{:?} ({}, {}), remain {}, {}.{} elapsed",
                        result.player,
                        row,
                        col,
                        result.num_remain,
                        duration.as_secs(),
                        duration.subsec_millis()
                    );
                },
                Err(err) => return Err(format!("agent::play : {}", err)),
            };

            // if game end, method return the winner, or None.
            let is_end = game.is_game_end();
            if is_end != Player::None {
                winner = is_end;
                break;
            }
        }

        if self.debug {
            game.print(&mut io::stdout()).unwrap();
        }
        Ok(PlayResult { winner, path })
    }
}
