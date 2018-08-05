use super::*;
use std::time::Instant;

#[test]
fn test_single_policy_mcts() {
    let mut policy = DefaultPolicy::with_num_iter(1);
    let mut mcts = SinglePolicyMCTS::new(&mut policy);

    let now = Instant::now();
    let result = mcts.run();
    let done = now.elapsed().as_secs();

    println!("{} elapsed", done);
    if let Some(last) = result.path.last() {
        assert_eq!(last.turn, result.winner);
    } else {
        assert!(true);
    }
}

#[test]
fn test_seperate_policy_mcts() {
    let mut black_policy = DefaultPolicy::with_num_iter(1);
    let mut white_policy = DefaultPolicy::with_num_iter(1);
    let mut mcts = SeperatePolicyMCTS::new(&mut black_policy, &mut white_policy);

    let now = Instant::now();
    let result = mcts.run();
    let done = now.elapsed().as_secs();

    println!("{} elapsed", done);
    if let Some(last) = result.path.last() {
        assert_eq!(last.turn, result.winner);
    } else {
        assert!(true);
    }
}