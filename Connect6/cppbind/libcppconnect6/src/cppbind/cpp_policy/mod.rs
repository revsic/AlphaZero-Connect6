use connect6::{game, policy, Board, BOARD_SIZE};
use cppbind::{board_to_float, CFloat, CInt};

pub type PolicyCallback = extern "C" fn(*const [[CFloat; BOARD_SIZE]; BOARD_SIZE], *mut [CInt; 2]);

pub struct CppPolicy {
    callback: PolicyCallback,
}

impl CppPolicy {
    pub fn new(callback: PolicyCallback) -> CppPolicy {
        CppPolicy { callback }
    }

    fn callback(&self, board: &Board) -> Option<(usize, usize)> {
        let mut res: [CInt; 2] = [-1; 2];
        let board_f = board_to_float(board);

        (self.callback)(
            &board_f as *const [[CFloat; BOARD_SIZE]; BOARD_SIZE],
            &mut res as *mut [CInt; 2],
        );

        if res[0] == -1 {
            None
        } else {
            Some((res[0] as usize, res[1] as usize))
        }
    }
}

impl policy::Policy for CppPolicy {
    fn next(&mut self, game: &game::Game) -> Option<(usize, usize)> {
        self.callback(game.get_board())
    }
}
