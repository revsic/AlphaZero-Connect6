use super::Policy;
use super::super::game::*;

#[cfg(test)]
mod tests;

pub struct MultiPolicy<'a, 'b> {
    black_policy: &'a mut Policy,
    white_policy: &'b mut Policy,
}

impl<'a, 'b> MultiPolicy<'a, 'b> {
    pub fn new(black_policy: &'a mut Policy, white_policy: &'b mut Policy) -> MultiPolicy<'a, 'b> {
        MultiPolicy { black_policy, white_policy }
    }
}

impl<'a, 'b> Policy for MultiPolicy<'a, 'b> {
    fn next(&mut self, game: &Game) -> Option<(usize, usize)> {
        match game.get_turn() {
            Player::None => panic!("seperate_policy::init couldn't get next policy for player none"),
            Player::Black => self.black_policy.next(game),
            Player::White => self.white_policy.next(game),
        }
    }
}