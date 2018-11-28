use connect6::{BOARD_CAPACITY, BOARD_SIZE, game::Player};
use cppbind::{CInt, RawPath};

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

#[no_mangle]
pub extern "C" fn test_sample_raw_path() -> RawPath {
    let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = (i * BOARD_SIZE + j) as CInt;
        }
    }
    RawPath {
        turn: Player::Black as CInt,
        board,
        row: 0,
        col: (BOARD_SIZE - 1) as CInt,
    }
}
