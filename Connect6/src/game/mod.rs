pub use self::player::Player;
pub use self::game_impl::{PlayResult, Game};
pub use self::winner_searcher::search;

mod player;
mod position;
mod game_impl;
mod winner_searcher;
