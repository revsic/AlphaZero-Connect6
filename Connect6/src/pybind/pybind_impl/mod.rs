extern crate cpython;

use cpython::*;
use super::super::game::*;

#[cfg(test)]
mod tests;

type Board = [[Player; 19]; 19];

pub fn pyseq_to_vec(py: Python, obj: PyObject) -> Option<Vec<f32>> {
    let vec = obj.cast_into::<PySequence>(py).ok()?
        .iter(py).ok()?
        .filter_map(|x| x.ok())
        .filter_map(|x| x.extract::<f32>(py).ok())
        .collect::<Vec<f32>>();
    Some(vec)
}

pub fn pylist_from_board(py: Python, board: &Board) -> PyObject {
    let mut ordered: Vec<PyObject> = Vec::with_capacity(19 * 19);
    for i in 0..19 {
        for j in 0..19 {
            ordered.push((board[i][j] as i32).to_py_object(py).into_object());
        }
    }
    PyList::new(py, ordered.as_slice()).into_object()
}

pub fn pylist_from_multiple(py: Python, boards: &Vec<Board>) -> PyObject {
    let lists = boards.iter()
        .map(|x| pylist_from_board(py, x))
        .collect::<Vec<_>>();
    PyList::new(py, lists.as_slice()).into_object()
}