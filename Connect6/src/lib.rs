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
    try!(m.add(py, "self_play", py_fn!(py, self_play(object: PyObject))));
    try!(m.add(py, "debug", py_fn!(py, debug(object: PyObject))));
    try!(m.add(py, "with_param", py_fn!(py, with_param(object: PyObject,
                                                       num_simulation: i32,
                                                       num_expansion: usize,
                                                       epsilon: f32,
                                                       dirichlet_alpha: f64,
                                                       c_puct: f32,
                                                       debug: bool,
                                                       num_thread: i32))));
    Ok(())
});

fn self_play(py: Python, object: PyObject) -> PyResult<PyTuple> {
    let mut policy = pybind::AlphaZero::new(object);
    let result = agent::Agent::new(&mut policy).play();
    Ok(result.unwrap().to_py_object(py))
}

fn debug(py: Python, object: PyObject) -> PyResult<PyTuple> {
    let mut policy = pybind::AlphaZero::new(object);
    let result = agent::Agent::debug(&mut policy).play();
    Ok(result.unwrap().to_py_object(py))
}

fn with_param(py: Python,
              object: PyObject,
              num_simulation: i32,
              num_expansion: usize,
              epsilon: f32,
              dirichlet_alpha: f64,
              c_puct: f32,
              debug: bool,
              num_thread: i32) -> PyResult<PyTuple> {
    let param = pybind::HyperParameter {
        num_simulation,
        num_expansion,
        epsilon,
        dirichlet_alpha,
        c_puct
    };
    if num_thread == 1 {
        let mut policy = pybind::AlphaZero::with_param(object, param);
        let result =
            if debug { agent::Agent::new(&mut policy).play() }
                else { agent::Agent::debug(&mut policy).play() };
        Ok(result.unwrap().to_py_object(py))
    } else {
        py.allow_threads(|| {
            let policy_gen = || pybind::AlphaZero::with_param(object.clone_ref(py), param);
            let async_agent =
            if debug { agent::AsyncAgent::new(policy_gen) }
                else { agent::AsyncAgent::debug(policy_gen) };
        })


        let result = async_agent.run(num_thread).iter()
            .map(|x| x.to_py_object(py).into_object())
            .collect::<Vec<_>>();

        Ok(PyTuple::new(py, result.as_slice()))
    }

}