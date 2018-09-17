use super::super::game::Player;
use super::super::policy::Evaluator;
use super::super::{Board, BOARD_SIZE};
use super::{pylist_from_multiple, pyseq_to_vec};

use cpython::{Python, PyObject, PythonObject, PySequence, PyTuple, ToPyObject, ObjectProtocol};

#[cfg(test)]
mod tests;

/// `AlphaZero` value, policy approximator with python callable object
///
/// it implement trait `Evaluator` for evaluating given boards with `PyObject`
///
/// # Examples
/// ```rust
/// let pyeval = PyEval(pyobj);
/// let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
/// assert!(pyeval.eval(Player::Black, &vec![board]).is_some());
/// ```
pub struct PyEval {
    pyobj: PyObject,
}

impl PyEval {
    /// Create new `PyEval` with given `PyObject`.
    pub fn new(pyobj: PyObject) -> PyEval {
        PyEval { pyobj }
    }
}

impl Evaluator for PyEval {
    /// Get value and prob from `PyObject`
    ///
    /// # Panics
    /// - If `self.pyobj` is not callable object, or method `__call__` is not a type of `__call__(self, turn, board): (value, prob)`
    /// - if return value of `self.pyobj.call()` is not a tuple type object.
    ///
    /// # Errors
    /// - if `value` is not a sequence type object consists of floats.
    /// - if `policy` is not a 2D sequence type object consists of floats.
    /// - if `policy` is not shaped `[boards.len(), BOARD_SIZE ** 2]`
    fn eval(&self, turn: Player, board: &Vec<Board>) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        // acquire python gil
        let gil = Python::acquire_gil();
        let py = gil.python();

        // convert parameter to python object
        let py_turn = (turn as i32).to_py_object(py);
        let py_board = pylist_from_multiple(py, board);
        let res = must!(self.pyobj.call(py, (py_turn, py_board), None), "alpha_zero::get_from couldn't call pyobject");
        let pytuple = must!(res.cast_into::<PyTuple>(py), "alpha_zero::get_from couldn't cast into pytuple");

        let value = pytuple.get_item(py, 0);
        let policy = pytuple.get_item(py, 1);

        // convert python object to proper vector
        let value_vec = pyseq_to_vec(py, value)?;
        let policy_iter = policy.cast_into::<PySequence>(py).ok()?
            .iter(py).ok()?
            .filter_map(|x| x.ok()) // pyiter returns iterator of Result
            .filter_map(|x| pyseq_to_vec(py, x));

        let mut policy_vec = Vec::with_capacity(board.len());
        for policy in policy_iter {
            let mut temporal = [[0.; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    temporal[i][j] = policy[i * BOARD_SIZE + j];
                }
            }
            policy_vec.push(temporal);
        }

        Some((value_vec, policy_vec))
    }
}