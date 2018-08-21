//! Rust bindings for Python Interface.
//!
//! `rust-cpython` based rust bindings.
//! It provides some utilities related to implement AlphaZero.
extern crate cpython;

use cpython::*;
use super::super::{BOARD_SIZE, Board};

#[cfg(test)]
mod tests;

/// Panic with given error string if given Result is Err
///
/// # Examples
/// ```rust
/// let pyseq = pycheck!(obj.cast_into::<PySequence>(py), "couldn't cast obj into PySequence");
/// ```
#[macro_export]
macro_rules! pycheck {
    ($e:expr, $err:expr) => {
        match $e {
            Ok(obj) => obj,
            Err(e) => panic!("{} : {:?}", $err, e),
        }
    }
}

/// Convert PySequence to Vec<f32>
///
/// # Examples
/// ```rust
/// // python : obj = [1.0, 2.0, 3.0]
/// let vec = pyseq_to_vec(py, obj);
/// assert_eq!(vec, Som(vec![1., 2., 3.]));
///
/// # Panics
/// - If given `obj` couldn't cast into `PySequence`.
/// - If casted `obj` couldn't generate `PyIterator`.
/// ```
pub fn pyseq_to_vec(py: Python, obj: PyObject) -> Option<Vec<f32>> {
    let pyseq = pycheck!(obj.cast_into::<PySequence>(py), "pyseq_to_vec couldn't cast obj into pyseq");
    let pyiter = pycheck!(pyseq.iter(py), "pyseq_to_vec couldn't get iter from pyseq");
    let vec = pyiter
        .filter_map(|x| x.ok())
        .filter_map(|x| x.extract::<f32>(py).ok())
        .collect::<Vec<f32>>();
    Some(vec)
}

/// Convert board to PyList
///
/// # Examples
/// ```rust
/// let game = Game::new();
/// let obj = pylist_from_board(py, game.get_board());
/// // python : assert obj == [0, 0, 0, ...]
/// ```
pub fn pylist_from_board(py: Python, board: &Board) -> PyObject {
    let mut ordered: Vec<PyObject> = Vec::with_capacity(BOARD_SIZE * BOARD_SIZE);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            ordered.push((board[i][j] as i32).to_py_object(py).into_object());
        }
    }
    PyList::new(py, ordered.as_slice()).into_object()
}

/// Convert multiple boards to PyList
///
/// # Examples
/// ```rust
/// let mut vec = Vec::new();
/// let mut sim = Simulate::new();
/// for i in 0..6 {
///     sim.simulate_in(0, i);
///     vec.push(sim.board());
/// }
/// let result = pylist_from_multiple(py, &vec);
/// // python : assert result == [[-1, 0, ...], [-1, 1, 0, ...], [-1, 1, 1, 0, ...], ...]
/// ```
pub fn pylist_from_multiple(py: Python, boards: &Vec<Board>) -> PyObject {
    let lists = boards.iter()
        .map(|x| pylist_from_board(py, x))
        .collect::<Vec<_>>();
    PyList::new(py, lists.as_slice()).into_object()
}