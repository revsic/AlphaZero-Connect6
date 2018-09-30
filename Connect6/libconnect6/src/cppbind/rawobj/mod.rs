use agent::{Path, RunResult};
use cppbind::CINT;
use BOARD_SIZE;

use std::mem;

#[cfg(test)]
mod tests;

#[repr(C)]
pub struct RawPath {
    turn: CINT,
    board: [[CINT; BOARD_SIZE]; BOARD_SIZE],
    row: CINT,
    col: CINT,
}

#[repr(C)]
pub struct RawRunResult {
    winner: CINT,
    path: *const RawPath,
    len: CINT,
}

#[repr(C)]
pub struct RawVec<T> {
    vec: *const T,
    len: CINT,
}

impl RawPath {
    pub fn with_path(path: &Path) -> RawPath {
        let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                board[i][j] = path.board[i][j] as CINT;
            }
        }

        let (row, col) = path.pos;
        RawPath {
            turn: path.turn as CINT,
            board,
            row: row as CINT,
            col: col as CINT,
        }
    }
}

impl RawRunResult {
    pub fn with_result(result: &RunResult) -> RawRunResult {
        let vec = result
            .path
            .iter()
            .map(|x| RawPath::with_path(x))
            .collect::<Vec<_>>();

        let len = vec.len() as CINT;
        let path = vec.as_ptr();
        mem::forget(vec);

        RawRunResult {
            winner: result.winner as CINT,
            path,
            len,
        }
    }
}

impl<T> From<Vec<T>> for RawVec<T> {
    fn from(vec: Vec<T>) -> RawVec<T> {
        let len = vec.len() as CINT;
        let ptr = vec.as_ptr();

        mem::forget(vec);
        RawVec { vec: ptr, len }
    }
}
