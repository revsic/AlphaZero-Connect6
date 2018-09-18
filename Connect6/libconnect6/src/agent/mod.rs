//! Agent for playing game with given policy
//!
//! This module consists of two structures, `Agent` and `AsyncAgent`.
//! `Agent` is implementation of loop based single policy agent.
//! As we pass the policy, method `play` return the `PlayResult` consisted of winner and playing history (called path).
//!
//! `AsyncAgent` is multi-thread based agent, playing multiple game asynchronously.
//! It pass the policy generator and return the vector of `PlayResult`.
//!
//! # Examples
//! Play single game with single policy.
//! ```rust
//! let mut policy = RandomPolicy::new();
//! let mut agent = Agent::new(&mut policy);
//!
//! let result = agent.play();
//! println!("winner: {:?}", result.winner);
//! ```
//! Play multiple game asynchronously.
//! ```rust
//! let policy_gen = || RandomPolicy::new();
//! let async_agent = AsyncAgent::debug(policy_gen);
//!
//! let result = async_agent.run(4)
//! println!("ratio: {}", result.map(|x| x.winner as i32).sum::<i32>());
//! ```
//!
pub use self::agent_impl::*;
pub use self::async_agent::*;

mod agent_impl;
mod async_agent;
