use super::*;
use policy::{DefaultPolicy, RandomPolicy};
use agent::Agent;
use game::Player;

use std::time::Instant;

#[test]
fn test_multi_policy() {
    let mut black_policy = RandomPolicy::new();
    let mut white_policy = DefaultPolicy::with_num_iter(10);
    let mut policy = MultiPolicy::new(&mut black_policy, &mut white_policy);

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