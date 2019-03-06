use connect6::{game::Player, policy, Board, BOARD_SIZE};
use cppbind::{board_to_float, CFloat, CInt};

#[cfg(test)]
mod tests;

/// void(int player, float* values, float* board, int length)
pub type Callback = extern "C" fn(
    CInt,                                    // player
    *mut CFloat,                             // out: value
    *mut [[CFloat; BOARD_SIZE]; BOARD_SIZE], // in: board, out: policy
    CInt,                                    // num boards
);

/// AlphaZero value, policy approximator with c ffi callback
pub struct CppEval {
    callback: Callback,
}

impl CppEval {
    /// Create new CppEval object
    pub fn new(callback: Callback) -> CppEval {
        CppEval { callback }
    }

    /// Call policy method from C++ FFI
    fn callback(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        let len = board.len();
        let player = turn as CInt;
        let mut values = vec![0.; len];
        let mut policies = board.iter().map(board_to_float).collect::<Vec<_>>();

        (self.callback)(
            player,
            values.as_mut_ptr(),
            policies.as_mut_ptr(),
            len as CInt,
        );
        Some((values, policies))
    }
}

impl policy::Evaluator for CppEval {
    fn eval(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        self.callback(turn, board)
    }
}
