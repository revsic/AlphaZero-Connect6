use super::*;
use game::Player;
use policy::{AlphaZero, HyperParameter, RandomPolicy};

use cpython::{Python, PythonObject, PyClone};
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
#[ignore]
fn test_alphazero_run() {
    let mut param = HyperParameter::default();
    param.num_simulation = 10;

    let object = py_policy!();
    let policy_gen = || {
        let object = {
            let gil = Python::acquire_gil();
            let py = gil.python();
            object.clone_ref(py)
        };
        AlphaZero::with_param(object, param)
    };
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