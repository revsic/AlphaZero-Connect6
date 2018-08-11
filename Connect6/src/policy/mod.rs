pub use self::simulate::*;
pub use self::default_policy::*;
pub use self::io_policy::*;
pub use self::multi_policy::*;
pub use self::random_policy::*;

mod default_policy;
mod io_policy;
mod multi_policy;
mod random_policy;
mod simulate;

use super::game::Game;

pub trait Policy {
    fn next(&mut self, game: &Game) -> Option<(usize, usize)>;
}