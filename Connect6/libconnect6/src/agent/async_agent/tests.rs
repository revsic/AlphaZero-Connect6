use super::*;
use game::Player;
use policy::{AlphaZero, HyperParameter, RandomEvaluator, RandomPolicy};

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

#[test]
fn test_alphazero_run() {
    let param = HyperParameter::light_weight();

    let policy_gen = || AlphaZero::with_param(Box::new(RandomEvaluator {}), param);
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
