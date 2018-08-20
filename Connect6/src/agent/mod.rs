//! Agent for playing game with given policy
//!
//! This module exports two modules, agent_impl and async_agent.
//! `agent_impl` is implementation of loop based single policy agent.
//! As we pass the policy, method `play` return the `PlayResult` consisted of winner and playing history (called path).
//!
//! `async_agent` is multi-thread based agent, pass the policy generator and return vector of `PlayResult`.
//!
//! # Examples
//! agent_impl
//! ```rust
//! let mut policy = RandomPolicy::new();
//! let mut agent = Agent::new(&mut policy);
//!
//! let result = agent.play();
//! println!("winner: {:?}", result.winner);
//! ```
//! async_agent
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