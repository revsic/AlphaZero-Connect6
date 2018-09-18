//! Implementation of Game Connect6
//!
//! It defines the game connect6 and provides the algorithm to find the winner.
//!
//! # Examples
//! ```rust
//! let mut game = Game::new();
//! let result = game.play((0, 0)).unwrap();
//!
//! let winner = game.is_game_end();
//! if winner != Player::None {
//!     println!("game end, winner: {:?}", winner);
//! } else {
//!     println!("playing result: {:?}", result);
//! }
//! ```
pub use self::game_impl::{Game, PlayResult};
pub use self::player::Player;
pub use self::search_winner::search;

mod game_impl;
mod player;
mod search_winner;
