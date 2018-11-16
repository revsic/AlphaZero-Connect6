//! Rust sample usage of connect6.
extern crate connect6;

use connect6::{agent, policy};

fn main() {
    let param = policy::HyperParameter::light_weight();
    let eval = Box::new(policy::RandomEvaluator {});
    let mut policy = policy::AlphaZero::with_param(eval, param);

    let result = agent::Agent::debug(&mut policy).play();

    assert!(result.is_ok());
    println!("{:?} is win", result.unwrap().winner);
}
