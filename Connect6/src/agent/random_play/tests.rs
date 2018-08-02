use super::*;

// use std::{io, thread, time};

#[test]
fn test_random_play() {
    let agent = RandomPlayer::new();
    let result = agent.play();
    assert!(result.is_ok());
}

#[test]
fn test_random_play_io() {
    let agent = RandomPlayer::new();
    let result = agent.play_io(|agent: &Agent| {
        let game = agent.get_game();
        let _game = game.read().unwrap();

        // game.print(&mut io::stdout());
    });
    assert!(result.is_ok());
}