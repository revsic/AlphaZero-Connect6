//! Python binding of libconnect6
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

extern crate connect6;
extern crate cpython;
extern crate rand;

#[macro_use]
pub mod macro_def;

pub mod pybind;

use connect6::{agent, policy};
use cpython::*;

py_module_initializer!(
    libpyconnect6,
    initlibpyconnect6,
    PyInit_pyconnect6,
    |py, m| {
        try!(m.add(
            py,
            "__doc__",
            "This module is implemented in Rust, for Simulating Connect6"
        ));
        try!(m.add(
            py,
            "self_play",
            py_fn!(
                py,
                self_play(
                    object: PyObject,
                    num_simulation: i32,
                    epsilon: f32,
                    dirichlet_alpha: f64,
                    c_puct: f32,
                    debug: bool,
                    num_game_thread: i32
                )
            )
        ));
        try!(m.add(
            py,
            "play_with",
            py_fn!(
                py,
                play_with(
                    object: PyObject,
                    num_simulation: i32,
                    epsilon: f32,
                    dirichlet_alpha: f64,
                    c_puct: f32
                )
            )
        ));
        Ok(())
    }
);

/// Returns Connect6 self-playing results with given python policy and hyper parameters
///
/// # Arguments
///
/// * `py` - Python GIL, provided by rust-cpython.
/// * `object` - PyObject, callable object for AlphaZero python policy.
/// * `num_simulation` - i32, number of simulations for each turn.
/// * `epsilon` - f32, ratio for applying exploit, exploration. lower epsilon, more exploit
/// * `dirichlet_alpha` - f64, hyperparameter for dirichlet distribution
/// * `c_puct` - f32, ratio of q-value and puct, hyperparameter of AlphaZero MCTS
/// * `debug` - bool, enable debug mode. if enable, selection and board status will be printed
/// * `num_game_thread` - i32, number of threads asynchronously self-playing connect6
///
/// # Panics
///
/// If PyObject isn't callable object
///
fn self_play(
    py: Python,
    object: PyObject,
    num_simulation: i32,
    epsilon: f32,
    dirichlet_alpha: f64,
    c_puct: f32,
    debug: bool,
    num_game_thread: i32,
) -> PyResult<PyTuple> {
    let param = policy::HyperParameter {
        num_simulation,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };
    if num_game_thread == 1 {
        let pyeval = Box::new(pybind::PyEval::new(object));
        let mut policy = policy::AlphaZero::with_param(pyeval, param);
        let result = if debug {
            agent::Agent::debug(&mut policy).play()
        } else {
            agent::Agent::new(&mut policy).play()
        };
        Ok(pybind::RunResultWrapper(&result.unwrap()).to_py_object(py))
    } else {
        let result = py.allow_threads(move || {
            let policy_gen = || {
                let object = {
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    object.clone_ref(py)
                };
                let pyeval = Box::new(pybind::PyEval::new(object));
                policy::AlphaZero::with_param(pyeval, param)
            };
            let async_agent = if debug {
                agent::AsyncAgent::debug(policy_gen)
            } else {
                agent::AsyncAgent::new(policy_gen)
            };
            async_agent.run(num_game_thread)
        });
        let py_result = result
            .iter()
            .map(|x| pybind::RunResultWrapper(x).to_py_object(py).into_object())
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
/// * `epsilon` - f32, ratio for applying exploit, exploration. lower epsilon, more exploit
/// * `dirichlet_alpha` - f64, hyperparameter for dirichlet distribution
/// * `c_puct` - f32, ratio of q-value and puct, hyperparameter of AlphaZero MCTS
///
/// # Panics
///
/// If PyObject isn't callable object
///
fn play_with(
    py: Python,
    object: PyObject,
    num_simulation: i32,
    epsilon: f32,
    dirichlet_alpha: f64,
    c_puct: f32,
) -> PyResult<PyTuple> {
    let param = policy::HyperParameter {
        num_simulation,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };
    let pyeval = Box::new(pybind::PyEval::new(object));
    let mut py_policy = policy::AlphaZero::with_param(pyeval, param);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut io_policy = policy::IoPolicy::new(&mut stdin, &mut stdout);

    let mut multi_policy = policy::MultiPolicy::new(&mut py_policy, &mut io_policy);
    let result = agent::Agent::debug(&mut multi_policy).play();
    Ok(pybind::RunResultWrapper(&result.unwrap()).to_py_object(py))
}
