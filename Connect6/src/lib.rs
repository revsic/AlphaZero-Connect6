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
                                                     num_expansion: usize,
                                                     epsilon: f32,
                                                     dirichlet_alpha: f64,
                                                     c_puct: f32,
                                                     debug: bool,
                                                     num_game_thread: i32))));
    try!(m.add(py, "play_with", py_fn!(py, play_with(object: PyObject,
                                                     num_simulation: i32,
                                                     num_expansion: usize,
                                                     epsilon: f32,
                                                     dirichlet_alpha: f64,
                                                     c_puct: f32))));
    Ok(())
});

fn self_play(py: Python,
             object: PyObject,
             num_simulation: i32,
             num_expansion: usize,
             epsilon: f32,
             dirichlet_alpha: f64,
             c_puct: f32,
             debug: bool,
             num_game_thread: i32) -> PyResult<PyTuple> {
    let param = pybind::HyperParameter {
        num_simulation,
        num_expansion,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };
    if num_game_thread == 1 {
        let mut policy = pybind::AlphaZero::with_param(object, param);
        let result =
            if debug { agent::Agent::debug(&mut policy).play() }
                else { agent::Agent::new(&mut policy).play() };
        Ok(result.unwrap().to_py_object(py))
    } else {
        let result = py.allow_threads(move || {
            let policy_gen = || {
                let object = {
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    object.clone_ref(py)
                };
                pybind::AlphaZero::with_param(object, param)
            };
            let async_agent =
                if debug { agent::AsyncAgent::debug(policy_gen) }
                    else { agent::AsyncAgent::new(policy_gen) };
            async_agent.run(num_game_thread)
        });
        let py_result = result.iter()
            .map(|x| x.to_py_object(py).into_object())
            .collect::<Vec<_>>();
        Ok(PyTuple::new(py, py_result.as_slice()))
    }
}

fn play_with(py: Python,
             object: PyObject,
             num_simulation: i32,
             num_expansion: usize,
             epsilon: f32,
             dirichlet_alpha: f64,
             c_puct: f32) -> PyResult<PyTuple> {
    let param = pybind::HyperParameter {
        num_simulation,
        num_expansion,
        epsilon,
        dirichlet_alpha,
        c_puct,
    };
    let mut py_policy = pybind::AlphaZero::with_param(object, param);

    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let mut io_policy = policy::IoPolicy::new(&mut stdin, &mut stdout);

    let mut multi_policy = policy::MultiPolicy::new(&mut py_policy, &mut io_policy);
    let result = agent::Agent::debug(&mut multi_policy).play();
    Ok(result.unwrap().to_py_object(py))
}