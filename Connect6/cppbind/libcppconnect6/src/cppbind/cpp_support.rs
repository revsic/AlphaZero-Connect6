use connect6::{Board, BOARD_SIZE};

/// std::os::raw::c_int
pub type CInt = ::std::os::raw::c_int;

/// std::os::raw::c_float
pub type CFloat = ::std::os::raw::c_float;

/// Convert Player:Board to CFloat:Board
pub fn board_to_float(board: &Board) -> [[CFloat; BOARD_SIZE]; BOARD_SIZE] {
    let mut converted = [[0.; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            converted[i][j] = board[i][j] as i32 as CFloat;
        }
    }
    converted
}
