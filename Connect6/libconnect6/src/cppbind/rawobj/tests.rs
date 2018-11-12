use super::*;

use agent::Agent;
use game::Player;
use policy::RandomPolicy;
use rand;
use Board;

fn convert_board_from(board: &[[CInt; BOARD_SIZE]; BOARD_SIZE]) -> Board {
    let mut player_board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            player_board[i][j] = Player::from(board[i][j]);
        }
    }
    player_board
}

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

    assert_eq!(raw_path.turn, turn as CInt);
    assert_eq!(convert_board_from(&raw_path.board), board);
    assert_eq!(raw_path.row, pos.0 as CInt);
    assert_eq!(raw_path.col, pos.1 as CInt);
}

#[test]
fn test_raw_run_result() {
    let mut policy = RandomPolicy::new();
    let result = Agent::new(&mut policy).play();
    assert!(result.is_ok());

    let result = result.unwrap();
    let raw_result = RawRunResult::with_result(&result);

    assert_eq!(raw_result.winner, result.winner as CInt);
    assert_eq!(raw_result.len, result.path.len() as CInt);

    let len = raw_result.len as usize;
    let path_ptr = raw_result.path as *mut RawPath;
    let raw_paths = unsafe { Vec::from_raw_parts(path_ptr, len, len) };

    for i in 0..len {
        let path = &result.path[i];
        let raw_path = &raw_paths[i];

        assert_eq!(raw_path.turn, path.turn as CInt);
        assert_eq!(raw_path.row, path.pos.0 as CInt);
        assert_eq!(raw_path.col, path.pos.1 as CInt);
        assert_eq!(convert_board_from(&raw_path.board), path.board);
    }
}

#[test]
fn test_raw_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    let raw_vec = RawVec::from(vec.clone());

    let ptr = raw_vec.vec as *mut i32;
    let len = raw_vec.len as usize;

    let converted = unsafe { Vec::from_raw_parts(ptr, len, len) };
    assert_eq!(vec, converted);
}
