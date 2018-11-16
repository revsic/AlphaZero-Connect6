//! Agent for playing game with given policy
//!
//! This module consists of two structures, `Agent` and `AsyncAgent`.
//! `Agent` is implementation of loop based single policy agent.
//! As we pass the policy, method `play` return the `PlayResult` consisted of winner and playing history (called path).
//!
//! `AsyncAgent` is multi-thread based agent, playing multiple games asynchronously.
//! It pass the policy generator and return the vector of `PlayResult`.
//!
//! # Examples
//! Play single game with single policy.
//! ```rust
//! # extern crate connect6;
//! # use connect6::{agent::Agent, policy::RandomPolicy};
//! let mut policy = RandomPolicy::new();
//! let mut agent = Agent::new(&mut policy);
//!
//! let result = agent.play();
//! assert!(result.is_ok());
//! println!("winner: {:?}", result.unwrap().winner);
//! ```
//! Play multiple game asynchronously.
//! ```rust
//! # extern crate connect6;
//! # use connect6::{agent::AsyncAgent, policy::RandomPolicy};
//! let policy_gen = || RandomPolicy::new();
//! let async_agent = AsyncAgent::debug(policy_gen);
//!
//! let result = async_agent.run(4);
//! println!("ratio: {}", result.iter().map(|x| x.winner as i32).sum::<i32>());
//! assert_eq!(result.len(), 4);
//! ```
//!
pub use self::agent_impl::*;
pub use self::async_agent::*;

mod agent_impl;
mod async_agent;
