use super::*;
use game::Player;
use policy::RandomPolicy;

use std::time::Instant;

#[test]
fn test_run() {
    let policy_gen = || RandomPolicy::new();
    let async_agent = AsyncAgent::debug(policy_gen);

    let now = Instant::now();
    let result = async_agent.run(4);
    let elapsed = now.elapsed();
    println!("{}.{}s elapsed", elapsed.as_secs(), elapsed.subsec_millis());

    assert_eq!(result.len(), 4);
    for run_result in result {
        if let Some(last) = run_result.path.last() {
            if run_result.winner != Player::None {
                assert_eq!(last.turn, run_result.winner);
            }
        }
    }
}
