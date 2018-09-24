use game::Player;
use policy::Evaluator;
use {Board, BOARD_CAPACITY, BOARD_SIZE};

#[cfg(test)]
mod tests;

#[repr(C)]
pub struct Result {
    value: *mut f32,
    policy: *mut f32,
}

pub type CINT = ::std::os::raw::c_int;

pub type Callback = extern "C" fn(CINT, *const [[CINT; BOARD_SIZE]; BOARD_SIZE], CINT) -> Result;

struct CppEval {
    callback: Callback,
}

fn convert_to_cint(board: &Board) -> [[CINT; BOARD_SIZE]; BOARD_SIZE] {
    let mut converted = [[0; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            converted[i][j] = board[i][j] as CINT;
        }
    }
    converted
}

impl CppEval {
    fn new(callback: Callback) -> CppEval {
        CppEval { callback }
    }

    fn callback(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        let len = board.len();
        let player = turn as CINT;
        let converted = board.iter().map(convert_to_cint).collect::<Vec<_>>();

        let result = (self.callback)(player, converted.as_ptr(), len as CINT);

        let value = unsafe { Vec::from_raw_parts(result.value, len, len) };
        let policy_raw = unsafe {
            Vec::from_raw_parts(result.policy, len * BOARD_CAPACITY, len * BOARD_CAPACITY)
        };

        let mut iter = 0;
        let mut policy = Vec::with_capacity(len);

        for _ in 0..len {
            let mut board = [[0.; BOARD_SIZE]; BOARD_SIZE];
            for i in 0..BOARD_SIZE {
                for j in 0..BOARD_SIZE {
                    board[i][j] = policy_raw[iter];
                    iter += 1;
                }
            }
            policy.push(board);
        }

        Some((value, policy))
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
