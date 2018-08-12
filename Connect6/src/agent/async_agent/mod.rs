extern crate futures;
extern crate tokio;

use super::agent_impl::*;
use super::super::policy::*;
use self::futures::*;

struct Container<P: Policy, F: FnOnce() -> P> {
    policy_gen: Option<F>,
}

struct Docker { }

impl<P: Policy, F: FnOnce() -> P> Container<P, F> {
    fn new(gen: F) -> Container<P, F> {
        Container {
            policy_gen: Some(gen),
        }
    }
}

impl<P: Policy, F: FnOnce() -> P> Future for Container<P, F> {
    type Item = RunResult;
    type Error = String;
    fn poll(&mut self) -> Result<Async<RunResult>, String> {
        if let Some(policy_gen) = self.policy_gen.take() {
            let mut policy = policy_gen();
            Agent::new(&mut policy).play().map(|x| Async::Ready(x))
        } else {
            Err(String::from("container::poll couldn't get policy generator"))
        }
    }
}

impl Docker {
    fn new() -> Docker {
        Docker {}
    }

    fn run(num: i32) -> Vec<RunResult> {
        

        Vec::new()
    }
}