use super::*;

use connect6::game::Player;
use rand;

extern "C" fn test_callback(
    board_ptr: *const [[CFloat; BOARD_SIZE]; BOARD_SIZE],
    res_ptr: *mut [CInt; 2],
) {
    let board = unsafe { board_ptr.as_ref() }.unwrap();
    let res = unsafe { res_ptr.as_mut() }.unwrap();

    let mut sum = 0.;
    for row in board {
        for cell in row {
            sum += *cell;
        }
    }

    res[0] = ((sum as usize) % BOARD_SIZE) as CInt;
    res[1] = 0;
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
fn test_cpp_policy_new() {
    let _eval = CppPolicy::new(test_callback);
    assert!(true);
}

#[test]
fn test_cpp_policy_callback() {
    let board = create_random_board();

    let mut sum = 0;
    for row in board.iter() {
        for cell in row {
            sum += *cell as i32;
        }
    }

    let cpp_policy = CppPolicy::new(test_callback);
    if let Some((row, col)) = cpp_policy.callback(&board) {
        assert_eq!(row, (sum as usize) % BOARD_SIZE);
        assert_eq!(col, 0);
    } else {
        assert!(false);
    }
}
