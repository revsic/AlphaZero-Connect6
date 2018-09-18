use agent::Agent;
use game::Player;

use std::time::Instant;

#[test]
#[ignore]
fn test_agent_io() {
    io_policy_stdio!(policy);
    let now = Instant::now();
    let result = Agent::debug(&mut policy).play();
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