//! Implementation of Game Connect6
//!
//! It defines the game connect6 and provides the algorithm to find either game end or winner.
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
pub use self::player::Player;
pub use self::game_impl::{PlayResult, Game};
pub use self::search_winner::search;

mod player;
mod game_impl;
mod search_winner;
