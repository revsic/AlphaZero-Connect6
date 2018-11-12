use game::Player;
use policy::Evaluator;
use {Board, BOARD_CAPACITY, BOARD_SIZE};

#[cfg(test)]
mod tests;

/// std::os::raw::c_int
pub type CInt = ::std::os::raw::c_int;

/// std::os::raw::c_float
pub type CFloat = ::std::os::raw::c_float;

/// void(int player, float* values, float* board[SIZE][SIZE], int length)
pub type Callback = extern "C" fn(CInt, // player
                                  *mut CFloat, // out: value
                                  *mut [[CFloat; BOARD_SIZE]; BOARD_SIZE], // in: board, out: policy
                                  CInt); // num boards

/// AlphaZero value, policy approximator with c ffi callback
pub struct CppEval {
    callback: Callback,
}

fn convert_to_c_float(board: &Board) -> [[CFloat; BOARD_SIZE]; BOARD_SIZE] {
    let mut converted = [[0.; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            converted[i][j] = board[i][j] as i32 as CFloat;
        }
    }
    converted
}

impl CppEval {
    /// Create new CppEval object
    pub fn new(callback: Callback) -> CppEval {
        CppEval { callback }
    }

    fn callback(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        let len = board.len();
        let player = turn as CInt;
        let mut values = vec![0.; len];
        let mut policies = board.iter().map(convert_to_c_float).collect::<Vec<_>>();

        (self.callback)(player, values.as_mut_ptr(), policies.as_mut_ptr(), len as CInt);
        Some((values, policies))
    }
}

impl Evaluator for CppEval {
    fn eval(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        self.callback(turn, board)
    }
}
