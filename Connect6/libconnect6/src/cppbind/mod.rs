use game::Player;
use policy::Evaluator;
use {Board, BOARD_SIZE};

struct CppEval {}

impl Evaluator for CppEval {
    fn eval(
        &self,
        turn: Player,
        board: &Vec<Board>,
    ) -> Option<(Vec<f32>, Vec<[[f32; BOARD_SIZE]; BOARD_SIZE]>)> {
        None
    }
}
