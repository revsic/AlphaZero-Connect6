pub mod game;
pub mod agent;
pub mod mcts;
pub mod pybind;

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult, PyObject, PyTuple, ToPyObject};

py_module_initializer!(libconnect6, initlibconnect6, PyInit_connect6, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust, for Simulating Connect6"));
    try!(m.add(py, "test", py_fn!(py, test(a: i64, b: i64))));
    try!(m.add(py, "self_play", py_fn!(py, self_play(object: PyObject))));
    try!(m.add(py, "debug", py_fn!(py, debug(object: PyObject))));
    Ok(())
});

fn test(_: Python, a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

fn debug(py: Python, object: PyObject) -> PyResult<PyTuple> {
    let mut policy = pybind::py_policy::AlphaZero::debug(py, object);
    let mut mcts = mcts::SinglePolicyMCTS::debug(&mut policy);
    let result = mcts.run();
    Ok(result.to_py_object(py))
}

fn self_play(py: Python, object: PyObject) -> PyResult<PyTuple> {
    let mut policy = pybind::py_policy::AlphaZero::new(py, object);
    let mut mcts = mcts::SinglePolicyMCTS::new(&mut policy);
    let result = mcts.run();
    Ok(result.to_py_object(py))
}