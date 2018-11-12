use super::*;

use rand;
use std::slice;

extern "C" fn test_callback(
    player: CInt,
    values: *mut CFloat,
    policies: *mut [[CFloat; BOARD_SIZE]; BOARD_SIZE],
    len: CInt,
) {
    let len = len as usize;
    let value_ref = unsafe { slice::from_raw_parts_mut(values, len) };
    let policy_ref = unsafe { slice::from_raw_parts_mut(policies, len) };

    for i in 0..len {
        value_ref[i] = i as f32;

        let ind = &mut policy_ref[i];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                ind[i][j] *= 2.;
            }
        }
    }
    value_ref[0] = player as f32;
}

#[test]
fn test_convert_to_c_float() {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];

    board[0][0] = Player::Black;
    board[0][BOARD_SIZE - 1] = Player::White;
    board[BOARD_SIZE - 1][BOARD_SIZE - 1] = Player::Black;
    board[BOARD_SIZE - 1][0] = Player::White;

    let result = convert_to_c_float(&board);

    assert_eq!(result[0][0], -1.);
    assert_eq!(result[0][BOARD_SIZE - 1], 1.);
    assert_eq!(result[BOARD_SIZE - 1][BOARD_SIZE - 1], -1.);
    assert_eq!(result[BOARD_SIZE - 1][0], 1.);
}

#[test]
fn test_cppeval_new() {
    let _eval = CppEval::new(test_callback);
    assert!(true);
}

fn create_random_board() -> Board {
    let mut board = [[Player::None; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            board[i][j] = Player::from(rand::random::<i32>() % 3 - 1);
        }
    }
    board
}

#[test]
fn test_cppeval_callback() {
    let eval = CppEval::new(test_callback);

    let player = Player::Black;
    let len = rand::random::<usize>() % 10 + 10;
    let boards = (0..len).map(|_| create_random_board()).collect::<Vec<_>>();

    let result = eval.callback(player, &boards);
    assert!(result.is_some());

    let (value, policy) = result.unwrap();

    let mut target_value = (0..len).map(|x| x as f32).collect::<Vec<_>>();
    target_value[0] = player as i32 as f32;

    assert_eq!(value, target_value);

    let double = |x: &[[Player; BOARD_SIZE]; BOARD_SIZE]| {
        let mut board = [[0.; BOARD_SIZE]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                board[i][j] = 2. * x[i][j] as i32 as f32
            }
        }
        board
    };

    let target_policy = boards.iter().map(double).collect::<Vec<_>>();
    assert_eq!(policy, target_policy);
}
