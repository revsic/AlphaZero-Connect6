use connect6::{agent, game::Player, BOARD_CAPACITY, BOARD_SIZE};
use cppbind::{CInt, RawPath};

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
