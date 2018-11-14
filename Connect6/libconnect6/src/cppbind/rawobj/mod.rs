use agent::{Path, RunResult};
use cppbind::CInt;
use BOARD_SIZE;

#[cfg(test)]
mod tests;

pub type AllocatorType<T> = extern "C" fn(CInt) -> *mut T;

pub struct Allocator<T> {
    cpp_allocator: AllocatorType<T>,
}

impl<T> Allocator<T> {
    pub fn new(cpp_allocator: AllocatorType<T>) -> Allocator<T> {
        Allocator { cpp_allocator }
    }

    pub fn get(&self, size: usize) -> &mut [T] {
        let res = (self.cpp_allocator)(size as CInt);
        unsafe { ::std::slice::from_raw_parts_mut(res, size) }
    }
}

/// Path object for c ffi
#[repr(C)]
#[derive(Clone)]
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
    path: *mut RawPath,
    len: CInt,
}

/// Vector object for c ffi
#[repr(C)]
pub struct RawVec<T> {
    vec: *mut T,
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

impl Default for RawPath {
    fn default() -> RawPath {
        RawPath {
            turn: 0,
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            row: 0,
            col: 0,
        }
    }
}

impl RawRunResult {
    /// Create RawRunResult from RunResult
    pub fn with_result(result: &RunResult, alloc: &Allocator<RawPath>) -> RawRunResult {
        let path = &result.path;
        let len = path.len();

        let ptr = alloc.get(len);
        let itr = path.iter().map(|x| RawPath::with_path(x));
        for (p, i) in ptr.iter_mut().zip(itr) {
            *p = i;
        }

        RawRunResult {
            winner: result.winner as CInt,
            path: ptr.as_mut_ptr(),
            len: len as CInt,
        }
    }
}

impl<T> RawVec<T> {
    pub fn with_vec(vec: Vec<T>, alloc: &Allocator<T>) -> RawVec<T> {
        let len = vec.len();

        let ptr = alloc.get(len);
        for (p, i) in ptr.iter_mut().zip(vec.into_iter()) {
            *p = i;
        }
        RawVec {
            vec: ptr.as_mut_ptr(),
            len: len as CInt,
        }
    }
}
