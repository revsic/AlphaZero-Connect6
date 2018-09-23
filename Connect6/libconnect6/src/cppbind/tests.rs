use super::*;

#[test]
fn test_convert_to_cint() {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];

    board[0][0] = Player::Black;
    board[0][BOARD_SIZE - 1] = Player::White;
    board[BOARD_SIZE - 1][BOARD_SIZE - 1] = Player::Black;
    board[BOARD_SIZE - 1][0] = Player::White;

    let result = convert_to_cint(&board);

    assert_eq!(result[0][0], -1);
    assert_eq!(result[0][BOARD_SIZE - 1], 1);
    assert_eq!(result[BOARD_SIZE - 1][BOARD_SIZE - 1], -1);
    assert_eq!(result[BOARD_SIZE - 1][0], 1);
}
