//! Implementation of Game Connect6
//!
//! It defines the game connect6 and provides the algorithm to find the winner.
//!
//! # Examples
//! ```rust
//! # extern crate connect6;
//! # use connect6::game::{Game, Player};
//! let mut game = Game::new();
//! let result = game.play((0, 0)).unwrap();
//! let winner = game.is_game_end();
//! assert_eq!(winner, Player::None);
//! ```
pub use self::game_impl::{Game, Paint, PlayResult};
pub use self::player::Player;
pub use self::search_winner::{Cumulative, Block, Path, search};

mod game_impl;
mod player;
mod search_winner;
