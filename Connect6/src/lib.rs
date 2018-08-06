pub mod game;
pub mod agent;
pub mod mcts;
pub mod pybind;

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

py_module_initializer!(libconnect6, initlibconnect6, PyInit_connect6, |py, m| {
    try!(m.add(py, "__doc__", "This module is implemented in Rust, for Simulating Connect6"));
    try!(m.add(py, "test", py_fn!(py, test(a: i64, b: i64))));
    Ok(())
});

fn test(_: Python, a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

