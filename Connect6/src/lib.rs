pub mod game;
pub mod agent;
pub mod mcts;
pub mod pybind;

#[macro_use]
extern crate cpython;
use cpython::*;

const BOARD_SIZE: usize = 12;
const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;
type Board = [[game::Player; BOARD_SIZE]; BOARD_SIZE];

py_module_initializer!(libconnect6, initlibconnect6, PyInit_connect6, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust, for Simulating Connect6"));
    try!(m.add(py, "self_play", py_fn!(py, self_play(object: PyObject))));
    try!(m.add(py, "debug", py_fn!(py, debug(object: PyObject))));
    try!(m.add(py, "with_param", py_fn!(py, with_param(object: PyObject,
                                                       num_simulation: i32,
                                                       num_expansion: usize,
                                                       initial_tau: f32,
                                                       updated_tau: f32,
                                                       tau_update_term: usize,
                                                       epsilon: f32,
                                                       dirichlet_alpha: f64,
                                                       c_puct: f32))));
    Ok(())
});

fn self_play(py: Python, object: PyObject) -> PyResult<PyTuple> {
    let mut policy = pybind::AlphaZero::new(py, object);
    let mut mcts = mcts::SinglePolicyMCTS::new(&mut policy);
    let result = mcts.run();
    Ok(result.to_py_object(py))
}

fn debug(py: Python, object: PyObject) -> PyResult<PyTuple> {
    let mut policy = pybind::AlphaZero::new(py, object);
    let mut mcts = mcts::SinglePolicyMCTS::debug(&mut policy);
    let result = mcts.run();
    Ok(result.to_py_object(py))
}

fn with_param(py: Python,
              object: PyObject,
              num_simulation: i32,
              num_expansion: usize,
              initial_tau: f32,
              updated_tau: f32,
              tau_update_term: usize,
              epsilon: f32,
              dirichlet_alpha: f64,
              c_puct: f32) -> PyResult<PyTuple> {
    let param = pybind::HyperParameter {
        num_simulation,
        num_expansion,
        initial_tau,
        updated_tau,
        tau_update_term,
        epsilon,
        dirichlet_alpha,
        c_puct
    };
    let mut policy = pybind::AlphaZero::with_param(py, object, param);
    let mut mcts = mcts::SinglePolicyMCTS::new(&mut policy);
    let result = mcts.run();
    Ok(result.to_py_object(py))
}