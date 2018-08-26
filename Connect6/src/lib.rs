#![doc(html_root_url = "https://revsic.github.io/AlphaZero-Connect6/")]
// #![doc(html_logo_url = "https://raw.githubusercontent.com/revsic/AlphaZero-Connect6/master/logo.png")]
//! [Rust](https://www.rust-lang.org) implementation of connect6 and policies
//! for learning [AlphaZero](https://arxiv.org/abs/1712.01815) with [Python](https://www.python.org/) interface.
//!
//! Connect6 provides game environment and policy based self-playing agent with some pre-defined policies.
//! It also provides multi-threading async agent
//! and MCTS policy [AlphaZero](https://arxiv.org/abs/1712.01815) with some hyperparameters control.
//!
//! You can install `pyconnect6` with `setup.py` and join the AlphaZero self-playing agent
//! just passing the callable python object with method `__call__(self, turn, board): (value, prob)`.
//!
//! # Examples
//! ```python
//! import pyconnect6
//! import numpy as np
//!
//! board_size = 15
//! param = pyconnect6.default_param()
//! param['num_simulation'] = 10
//! param['debug'] = True
//!
//! policy = lambda turn, board: (np.random.rand(len(board)), np.random.rand(len(board), board_size ** 2))
//! play_result = pyconnect6.self_play(policy, param)
//!
//! win, path = play_result
//! print(win)
//! ```
#[macro_use]
mod macro_def;

pub mod agent;
pub mod game;
pub mod policy;
pub mod pybind;

#[macro_use]
extern crate cpython;

use cpython::*;

const BOARD_SIZE: usize = 15;
const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;

type Board = [[game::Player; BOARD_SIZE]; BOARD_SIZE];

py_module_initializer!(libconnect6, initlibconnect6, PyInit_connect6, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust, for Simulating Connect6"));
    try!(m.add(py, "self_play", py_fn!(py, self_play(object: PyObject,
                                                     num_simulation: i32,
                                                     epsilon: f32,
                                                     dirichlet_alpha: f64,
                                                     c_puct: f32,
                                                     debug: bool,
                                                     num_game_thread: i32))));
    try!(m.add(py, "play_with", py_fn!(py, play_with(object: PyObject,
                                                     num_simulation: i32,
                                                     epsilon: f32,
                                                     dirichlet_alpha: f64,
                                                     c_puct: f32))));
    Ok(())
});

/// Returns Connect6 self-playing results with given python policy and hyper parameters
///
/// # Arguments
///
/// * `py` - Python GIL, provided by rust-cpython.
/// * `object` - PyObject, callable object for AlphaZero python policy.
/// * `num_simulation` - i32, number of simulations for each turn.
/// * `num_expansion` - usize, number of child node expansion per simulation.
/// * `epsilon` - f32, ratio for applying exploit, exploration. lower epsilon, more exploit
/// * `dirichlet_alpha` - f64, hyperparameter for dirichlet distribution
/// * `c_puct` - f32, ratio of q-value and puct, hyperparameter of AlphaZero MCTS
/// * `debug` - bool, enable debug mode. if enable, selection and board status will be printed
/// * `num_game_thread` - i32, number of threads asynchronously self-playing connect6
///
/// # Panics
///
/// If PyObject isn't callable object, pybind::py_policy::AlphaZero::get_from will panic
///
fn self_play(py: Python,
             object: PyObject,
             num_simulation: i32,
             epsilon: f32,
             dirichlet_alpha: f64,
             c_puct: f32,
             debug: bool,
             num_game_thread: i32) -> PyResult<PyTuple> {
    let param = policy::HyperParameter {
        num_simulation,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };
    if num_game_thread == 1 {
        let mut policy = policy::AlphaZero::with_param(object, param);
        let result =
            if debug { agent::Agent::debug(&mut policy).play() } else { agent::Agent::new(&mut policy).play() };
        Ok(result.unwrap().to_py_object(py))
    } else {
        let result = py.allow_threads(move || {
            let policy_gen = || {
                let object = {
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    object.clone_ref(py)
                };
                policy::AlphaZero::with_param(object, param)
            };
            let async_agent =
                if debug { agent::AsyncAgent::debug(policy_gen) } else { agent::AsyncAgent::new(policy_gen) };
            async_agent.run(num_game_thread)
        });
        let py_result = result.iter()
            .map(|x| x.to_py_object(py).into_object())
            .collect::<Vec<_>>();
        Ok(PyTuple::new(py, py_result.as_slice()))
    }
}

/// Returns Connect6 results with given python policy and user selection as io_policy
///
/// # Arguments
///
/// * `py` - Python GIL, provided by rust-cpython.
/// * `object` - PyObject, callable object for AlphaZero python policy.
/// * `num_simulation` - i32, number of simulations for each turn.
/// * `num_expansion` - usize, number of child node expansion per simulation.
/// * `epsilon` - f32, ratio for applying exploit, exploration. lower epsilon, more exploit
/// * `dirichlet_alpha` - f64, hyperparameter for dirichlet distribution
/// * `c_puct` - f32, ratio of q-value and puct, hyperparameter of AlphaZero MCTS
///
/// # Panics
///
/// If PyObject isn't callable object, pybind::py_policy::AlphaZero::get_from will panic
///
fn play_with(py: Python,
             object: PyObject,
             num_simulation: i32,
             epsilon: f32,
             dirichlet_alpha: f64,
             c_puct: f32) -> PyResult<PyTuple> {
    let param = policy::HyperParameter {
        num_simulation,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };
    let mut py_policy = policy::AlphaZero::with_param(object, param);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut io_policy = policy::IoPolicy::new(&mut stdin, &mut stdout);

    let mut multi_policy = policy::MultiPolicy::new(&mut py_policy, &mut io_policy);
    let result = agent::Agent::debug(&mut multi_policy).play();
    Ok(result.unwrap().to_py_object(py))
}