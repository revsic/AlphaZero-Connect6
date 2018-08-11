pub use self::player::Player;
pub use self::game_impl::{PlayResult, Game};
pub use self::search_winner::search;

mod player;
mod game_impl;
mod search_winner;
