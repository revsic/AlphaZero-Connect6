//! Agent for playing multiple games asynchronously.
//!
//! `AsyncAgent` play multiple games on tokio thread-pool.
//! It pass the policy generator and return the vector of game result.
//!
//! # Examples
//! ```rust
//! # extern crate connect6;
//! # use connect6::{agent::AsyncAgent, policy::RandomPolicy};
//! let policy_gen = || RandomPolicy::new();
//! let async_agent = AsyncAgent::debug(policy_gen);
//!
//! let result = async_agent.run(4);
//! println!("ratio: {}", result.iter().map(|x| x.winner as i32).sum::<i32>() as f32 / 4.);
//! # assert_eq!(result.len(), 4);
//! ```
use agent::{Agent, PlayResult};
use policy::Policy;

use futures::future;
use std::sync::mpsc;
use std::time::Instant;
use tokio::executor::thread_pool::ThreadPool;

#[cfg(test)]
mod tests;

/// Agent for playing multiple games asynchronously.
///
/// `AsyncAgent` play multiple games on tokio thread-pool.
/// It pass the policy generator and return the vector of game result.
///
/// # Examples
/// ```rust
/// # extern crate connect6;
/// # use connect6::{agent::AsyncAgent, policy::RandomPolicy};
/// let policy_gen = || RandomPolicy::new();
/// let async_agent = AsyncAgent::debug(policy_gen);
///
/// let result = async_agent.run(4);
/// println!("ratio: {}", result.iter().map(|x| x.winner as i32).sum::<i32>());
/// # assert_eq!(result.len(), 4);
/// ```
pub struct AsyncAgent<P: 'static + Policy + Send, F: Fn() -> P> {
    policy_gen: F,
    debug: bool,
}

impl<P: 'static + Policy + Send, F: Fn() -> P> AsyncAgent<P, F> {
    /// Construct a new AsyncAgent.
    ///
    /// Get policy generator as callable object which return impl `Policy`.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::{agent::AsyncAgent, policy::RandomPolicy};
    /// let gen = || RandomPolicy::new();
    /// let async_agent = AsyncAgent::new(gen);
    /// ```
    pub fn new(policy_gen: F) -> AsyncAgent<P, F> {
        AsyncAgent {
            policy_gen,
            debug: false,
        }
    }

    /// Construct a debug mode AsyncAgent, it will display the dbg info.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::{agent::AsyncAgent, policy::RandomPolicy};
    /// let gen = || RandomPolicy::new();
    /// let async_agent = AsyncAgent::debug(gen);
    /// ```
    pub fn debug(policy_gen: F) -> AsyncAgent<P, F> {
        AsyncAgent {
            policy_gen,
            debug: true,
        }
    }

    /// Self-play the given number of games asynchronously on thread pool.
    ///
    /// # Examples
    /// ```rust
    /// # extern crate connect6;
    /// # use connect6::{agent::AsyncAgent, policy::RandomPolicy};
    /// let gen = || RandomPolicy::new();
    /// let async_agent = AsyncAgent::new(gen);
    ///
    /// let result = async_agent.run(4);
    /// println!("result: {}", result.iter().map(|x| x.winner as i32).sum::<i32>());
    /// # assert_eq!(result.len(), 4);
    /// ```
    ///
    /// # Panics
    /// If some games return the Err from [Agent::play](./struct.Agent.html#method.play).
    pub fn run(&self, num: i32) -> Vec<PlayResult> {
        let thread_pool = ThreadPool::new();
        let (sender, receiver) = mpsc::channel();
        for id in 0..num {
            let debug = self.debug;
            let policy = (self.policy_gen)();
            let sender = sender.clone();
            thread_pool.spawn(future::lazy(move || {
                let mut policy = policy;

                let now = Instant::now();
                let res = Agent::new(&mut policy).play();
                let elapsed = now.elapsed();

                if let Ok(result) = res {
                    sender.send(result).unwrap();
                }
                if debug {
                    println!(
                        "run: {}, elapsed {}.{}s",
                        id,
                        elapsed.as_secs(),
                        elapsed.subsec_millis()
                    );
                }
                Ok(())
            }));
        }

        let mut results = Vec::new();
        for _ in 0..num {
            // able to panic, when agent return Err instead PlayResult
            let res = receiver.recv().unwrap();
            results.push(res);
        }
        thread_pool.shutdown();
        results
    }
}
