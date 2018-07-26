extern crate connect6;

use connect6::game_impl::*;

#[test]
fn play_game() {
    let game = Game::new();
    game.print();

    assert_eq!(1, 1);
}