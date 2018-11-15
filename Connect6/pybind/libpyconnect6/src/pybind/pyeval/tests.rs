use super::*;

use connect6::game::Player;
use connect6::policy::Evaluator;

#[test]
fn test_eval() {
    let pyeval = py_policy!();

    let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    let result = pyeval.eval(Player::Black, &vec![board, board, board]);
    assert!(result.is_some());

    let (value_vec, policy_vec) = result.unwrap();
    assert_eq!(value_vec.len(), 3);
    assert_eq!(policy_vec.len(), 3);
}
