use super::*;

#[test]
fn test_new() {
    let obj = py_policy!();
    PyEval::new(obj);
    assert!(true);
}

#[test]
fn test_eval() {
    let obj = py_policy!();
    let pyeval = PyEval::new(obj);

    let board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    let result = pyeval.eval(Player::Black, &vec![board, board, board]);
    assert!(result.is_some());

    let (value_vec, policy_vec) = result.unwrap();
    assert_eq!(value_vec.len(), 3);
    assert_eq!(policy_vec.len(), 3);
}