extern crate futures;
extern crate tokio;

use super::agent_impl::*;
use super::super::policy::*;

use std::sync::mpsc;
use std::time::Instant;
use self::futures::future;
use self::tokio::executor::thread_pool::ThreadPool;

#[cfg(test)]
mod tests;

pub struct AsyncAgent<P: 'static + Policy + Send, F: Fn() -> P> {
    policy_gen: F,
    debug: bool,
}

impl<P: 'static + Policy + Send, F: Fn() -> P> AsyncAgent<P, F> {
    pub fn new(policy_gen: F) -> AsyncAgent<P, F> {
        AsyncAgent {
            policy_gen,
            debug: false,
        }
    }

    pub fn debug(policy_gen: F) -> AsyncAgent<P, F> {
        AsyncAgent {
            policy_gen,
            debug: true,
        }
    }

    pub fn run(&self, num: i32) -> Vec<RunResult> {
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
                    println!("run: {}, elapsed {}.{}s", id, elapsed.as_secs(), elapsed.subsec_millis());
                }
                Ok(())
            }));
        }

        let mut results = Vec::new();
        for _ in 0..num {
            let res = receiver.recv().unwrap();
            results.push(res);
        }
        thread_pool.shutdown();
        results
    }
}