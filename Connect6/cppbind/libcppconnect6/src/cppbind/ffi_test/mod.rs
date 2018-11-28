use connect6::{agent, game::Player, BOARD_CAPACITY, BOARD_SIZE};
use cppbind::{Allocator, AllocatorType, CInt, RawPath, RawPlayResult, RawVec};

#[no_mangle]
pub extern "C" fn test_new_raw_path() -> RawPath {
    RawPath::new()
}

#[no_mangle]
pub extern "C" fn test_with_raw_path() -> RawPath {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let id = ((i * BOARD_SIZE + j) % 3) as i32 - 1;
            board[i][j] = Player::from(id);
        }
    }

    let path = agent::Path {
        turn: Player::White,
        board,
        pos: (0, BOARD_SIZE % 5 + 1),
    };
    RawPath::with_path(&path)
}

#[no_mangle]
pub extern "C" fn test_echo_raw_path(
    turn: CInt,
    board_ptr: *mut CInt,
    row: CInt,
    col: CInt,
) -> RawPath {
    let board_slice = unsafe { ::std::slice::from_raw_parts(board_ptr, BOARD_CAPACITY) };
    let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = board_slice[i * BOARD_SIZE + j];
        }
    }
    RawPath {
        turn,
        board,
        row,
        col,
    }
}

// #[no_mangle]
// pub extern "C" fn test_with_raw_play_result() -> RawPlayResult {

// }

// #[no_mangle]
// pub extern "C" fn test_echo_raw_play_result() -> RawPlayResult {

// }

#[no_mangle]
pub extern "C" fn test_with_raw_vec(allocator: AllocatorType<CInt>) -> RawVec<CInt> {
    let vec = vec![0, 1, 2, 3, 4, 5];
    let alloc = Allocator::new(allocator);
    RawVec::with_vec(vec, &alloc)
}

#[no_mangle]
pub extern "C" fn test_echo_raw_vec(
    ptr: *const CInt,
    len: CInt,
    allocator: AllocatorType<CInt>,
) -> RawVec<CInt> {
    let raw_slice = unsafe { ::std::slice::from_raw_parts(ptr, len as usize) };
    let mut vec = Vec::new();
    for i in 0..len {
        vec.push(raw_slice[i as usize]);
    }
    let alloc = Allocator::new(allocator);
    RawVec::with_vec(vec, &alloc)
}
