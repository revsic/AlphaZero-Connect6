extern crate rand;
extern crate cpython;

use super::*;
use super::super::super::game::Player;
use super::super::super::pybind::*;
use super::super::super::BOARD_CAPACITY;
use self::cpython::*;

use std::time::Instant;

py_class!(class PyPolicy |py| {
    def __call__(&self, _turn: PyObject, boards: PyObject) -> PyResult<PyObject> {
        let len = boards.cast_into::<PyList>(py)?.len(py);

        let value = (0..len)
            .map(|_| rand::random::<f32>().to_py_object(py).into_object())
            .collect::<Vec<PyObject>>();
        let value = PyList::new(py, value.as_slice()).into_object();

        let policy = (0..len).map(|_| {
            let rand_policy = (0..BOARD_CAPACITY)
                .map(|_| rand::random::<f32>().to_py_object(py).into_object())
                .collect::<Vec<PyObject>>();
            PyList::new(py, rand_policy.as_slice()).into_object()
        }).collect::<Vec<PyObject>>();

        let policy = PyList::new(py, policy.as_slice()).into_object();
        Ok(PyTuple::new(py, &[value, policy]).into_object())
    }
});

macro_rules! py_policy {
    () => {{
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyPolicy::create_instance(py).unwrap().into_object()
    }}
}

#[test]
fn test_run() {
    let policy_gen = || RandomPolicy::new();
    let async_agent = AsyncAgent::debug(policy_gen);

    let now = Instant::now();
    let result = async_agent.run(4);
    let elapsed = now.elapsed();
    println!("{}.{}s elapsed", elapsed.as_secs(), elapsed.subsec_millis());

    assert_eq!(result.len(), 4);
    for run_result in result {
        if let Some(last) = run_result.path.last() {
            if run_result.winner != Player::None {
                assert_eq!(last.turn, run_result.winner);
            }
        }
    }
}

#[test]
fn test_alphazero_run() {
    let mut param = HyperParameter::default();
    param.num_simulation = 10;

    let object = py_policy!();
    let policy_gen = || {
        let object = {
            let gil = Python::acquire_gil();
            let py = gil.python();
            object.clone_ref(py)
        };
        AlphaZero::with_param(object, param)
    };
    let async_agent = AsyncAgent::debug(policy_gen);

    let now = Instant::now();
    let result = async_agent.run(4);
    let elapsed = now.elapsed();
    println!("{}.{}s elapsed", elapsed.as_secs(), elapsed.subsec_millis());

    assert_eq!(result.len(), 4);
    for run_result in result {
        if let Some(last) = run_result.path.last() {
            if run_result.winner != Player::None {
                assert_eq!(last.turn, run_result.winner);
            }
        }
    }
}