use agent::{Path, RunResult};
use cppbind::CInt;
use BOARD_SIZE;

use std::mem;

#[cfg(test)]
mod tests;

/// Path object for c ffi
#[repr(C)]
pub struct RawPath {
    turn: CInt,
    board: [[CInt; BOARD_SIZE]; BOARD_SIZE],
    row: CInt,
    col: CInt,
}

/// RunResult object for c ffi
#[repr(C)]
pub struct RawRunResult {
    winner: CInt,
    path: *const RawPath,
    len: CInt,
}

/// Vector object for c ffi
#[repr(C)]
pub struct RawVec<T> {
    vec: *const T,
    len: CInt,
}

impl RawPath {
    /// Create RawPath from Path
    pub fn with_path(path: &Path) -> RawPath {
        let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                board[i][j] = path.board[i][j] as CInt;
            }
        }

        let (row, col) = path.pos;
        RawPath {
            turn: path.turn as CInt,
            board,
            row: row as CInt,
            col: col as CInt,
        }
    }
}

impl RawRunResult {
    /// Create RawRunResult from RunResult
    pub fn with_result(result: &RunResult) -> RawRunResult {
        let vec = result
            .path
            .iter()
            .map(|x| RawPath::with_path(x))
            .collect::<Vec<_>>();

        let len = vec.len() as CInt;
        let path = vec.as_ptr();
        mem::forget(vec);

        RawRunResult {
            winner: result.winner as CInt,
            path,
            len,
        }
    }
}

impl<T> From<Vec<T>> for RawVec<T> {
    fn from(vec: Vec<T>) -> RawVec<T> {
        let len = vec.len() as CInt;
        let ptr = vec.as_ptr();

        mem::forget(vec);
        RawVec { vec: ptr, len }
    }
}
