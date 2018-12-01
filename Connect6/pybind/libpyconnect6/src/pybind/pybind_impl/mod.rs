//! Rust bindings for Python Interface.
//!
//! `rust-cpython` based rust bindings.
//! It provides some utilities related to implement AlphaZero.
use connect6::{agent, Board, BOARD_SIZE};
use cpython::*;

#[cfg(test)]
mod tests;

/// Convert PyIterator to Vec
///
/// # Panics
/// - If given `obj` couldn't generate `PyIterator`.
pub fn pyiter_to_vec<T>(py: Python, obj: PyObject) -> Option<Vec<T>>
where
    for<'a> T: FromPyObject<'a>,
{
    let vec = obj
        .iter(py)
        .ok()?
        .filter_map(|x| x.ok())
        .filter_map(|x| x.extract(py).ok())
        .collect::<Vec<T>>();
    Some(vec)
}

/// Convert board to PyList
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
/// # extern crate cpython;
/// # extern crate connect6;
/// # extern crate pyconnect6;
/// # use connect6::policy::Simulate;
/// # use pyconnect6::pybind::pylist_from_multiple;
/// let mut vec = Vec::new();
/// let mut sim = Simulate::new();
/// for i in 0..6 {
///     sim.simulate_in(0, i);
///     vec.push(sim.board());
/// }
/// # let gil = cpython::Python::acquire_gil();
/// # let py = gil.python();
/// let result = pylist_from_multiple(py, &vec);
/// ```
pub fn pylist_from_multiple(py: Python, boards: &Vec<Board>) -> PyObject {
    let lists = boards
        .iter()
        .map(|x| pylist_from_board(py, x))
        .collect::<Vec<_>>();
    PyList::new(py, lists.as_slice()).into_object()
}

/// connect6::agent::Path wrapper for Python object conversion
pub struct PathWrapper<'a>(pub &'a agent::Path);

impl<'a> ToPyObject for PathWrapper<'a> {
    type ObjectType = PyTuple;

    /// Return `PyTuple, (turn: int, board: list(int, board_size ** 2), pos: (int, int))`
    fn to_py_object(&self, py: Python) -> PyTuple {
        let turn = (self.0.turn as i32).to_py_object(py).into_object();
        let board = pylist_from_board(py, &self.0.board);
        let (row, col) = self.0.pos;

        let row = (row as i32).to_py_object(py).into_object();
        let col = (col as i32).to_py_object(py).into_object();
        let pos_tuple = PyTuple::new(py, &[row, col]).into_object();

        let tuple = PyTuple::new(py, &[turn, board, pos_tuple]);
        tuple
    }
}

/// connect6::agent::PlayResult wrapper for Python object conversion
pub struct RunResultWrapper<'a>(pub &'a agent::PlayResult);

impl<'a> ToPyObject for RunResultWrapper<'a> {
    type ObjectType = PyTuple;

    /// Return `PyTuple, (winner: int, path: list(Path as PyTuple))`
    fn to_py_object(&self, py: Python) -> PyTuple {
        let win = (self.0.winner as i32).to_py_object(py).into_object();
        let path = self
            .0
            .path
            .iter()
            .map(|x| PathWrapper(x).to_py_object(py).into_object())
            .collect::<Vec<_>>();
        let list = PyList::new(py, path.as_slice()).into_object();
        let tuple = PyTuple::new(py, &[win, list]);
        tuple
    }
}
