use agent::{Path, RunResult};
use cppbind::CInt;
use BOARD_SIZE;

#[cfg(test)]
mod tests;

/// Allocator Type for FFI, (ex. C++ new operator)
pub type AllocatorType<T> = extern "C" fn(CInt) -> *mut T;

/// Allocator for FFI, (ex. C++ new operator)
pub struct Allocator<T> {
    allocator: AllocatorType<T>,
}

impl<T> Allocator<T> {
    /// Create new Allocator with given allocator
    pub fn new(allocator: AllocatorType<T>) -> Allocator<T> {
        Allocator { allocator }
    }

    /// Obtain new dynamic memory from self.allocator
    pub fn get(&self, size: usize) -> &mut [T] {
        let res = (self.allocator)(size as CInt);
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
    /// Create zero initialized RawPath
    pub fn new() -> RawPath {
        RawPath {
            turn: 0,
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            row: 0,
            col: 0,
        }
    }

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
    /// Alias of RawPath::new
    fn default() -> RawPath {
        RawPath::new()
    }
}

impl RawRunResult {
    /// Create RawRunResult from RunResult with given allocator (for C++ new operation)
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
    /// Create RawVec from Vec with given allocator (for C++ new operation)
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
