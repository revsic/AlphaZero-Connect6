use super::*;
use super::super::super::agent::*;
use super::super::super::game::*;

use std::time::Instant;

#[test]
fn test_random_play() {
    let mut policy = RandomPolicy::new();

    let now = Instant::now();
    let result = Agent::new(&mut policy).play();
    let done = now.elapsed();

    println!("{}.{}s elapsed", done.as_secs(), done.subsec_millis());
    let result = result.map_err(|_| assert!(false)).unwrap();
    if let Some(last) = result.path.last() {
        if result.winner != Player::None {
            assert_eq!(last.turn, result.winner);
        }
    }
    assert!(true);
}