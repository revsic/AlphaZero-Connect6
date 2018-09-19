use game::Player;
use policy::Evaluator;
use {Board, BOARD_CAPACITY, BOARD_SIZE};

type CINT = ::std::os::raw::c_int;
type Callback = extern "C" fn(player: CINT, *mut CINT) -> Result;

#[repr(C)]
struct Result {
    value: *mut f32,
    policy: *mut f32,
}

struct CppEval {
    callback: Callback,
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
        let player = turn as CINT;
        let mut flatten: Vec<[i32; BOARD_CAPACITY]> = Vec::with_capacity(board.len());

        for board in board.iter() {
            let mut iter = 0;
            let mut flat = [0; BOARD_CAPACITY];
            for row in board {
                for elem in row {
                    flat[iter] = elem as CINT;
                    iter += 1;
                }
            }

            flatten.push(flat)
        }

        let result = (self.callback)(player, flatten.as_mut_ptr());
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
