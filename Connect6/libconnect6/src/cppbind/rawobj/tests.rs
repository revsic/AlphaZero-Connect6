use super::*;

use game::Player;
use rand;

#[test]
fn test_raw_path() {
    let rand_player = || Player::from(rand::random::<i32>() % 3 - 1);

    let turn = rand_player();

    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = rand_player();
        }
    }

    let pos = (rand::random(), rand::random());

    let path = Path { turn, board, pos };
    let raw_path = RawPath::with_path(&path);

    assert_eq!(raw_path.turn, turn as CINT);
    // assert_eq!()
    assert_eq!(raw_path.row, pos.0 as CINT);
}
